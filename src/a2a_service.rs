// SPDX-License-Identifier: Apache-2.0
use crate::{
    protocol::{PROFILE_VERSION, parse_json},
    worker::{Worker, WorkerError},
};
use a2a::*;
use a2a_server::{RequestHandler, middleware::ServiceParams};
use async_trait::async_trait;
use axum::{
    Json, Router,
    body::{Body, to_bytes},
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::sync::Arc;
pub struct ServiceConfig {
    credential_hash: [u8; 32],
    valid: bool,
    expires: DateTime<Utc>,
}
impl ServiceConfig {
    pub fn new(credential: String, expires: DateTime<Utc>) -> Self {
        Self {
            credential_hash: Sha256::digest(credential.as_bytes()).into(),
            valid: credential.len() >= 16
                && credential.len() <= 4096
                && credential.bytes().all(|b| b.is_ascii_graphic()),
            expires,
        }
    }
}
#[derive(Clone)]
struct ServiceState {
    worker: Worker,
    config: Arc<ServiceConfig>,
}
pub fn service(worker: Worker, config: ServiceConfig) -> Result<Router, WorkerError> {
    if !config.valid {
        return Err(WorkerError::InvalidTaskInput);
    }
    let state = ServiceState {
        worker: worker.clone(),
        config: Arc::new(config),
    };
    Ok(Router::new()
        .route("/", post(rpc))
        .with_state(state.clone())
        .merge(
            Router::new()
                .route("/.well-known/agent-card.json", get(card))
                .route("/control/v1/models", get(models))
                .route("/control/v1/receipts/{task}", get(receipt))
                .route("/control/v1/receipts/{task}/ack", post(ack))
                .with_state(state.clone()),
        )
        .layer(middleware::from_fn_with_state(state, guard)))
}
fn failure(code: WorkerError) -> Response {
    let status = match code {
        WorkerError::AuthenticationRequired | WorkerError::Forbidden => StatusCode::UNAUTHORIZED,
        WorkerError::ConversationBusy
        | WorkerError::DuplicateConflict
        | WorkerError::ConversationStale => StatusCode::CONFLICT,
        WorkerError::RuntimeUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::BAD_REQUEST,
    };
    (status, Json(json!({"code":code.to_string()}))).into_response()
}
fn closed(value: &Value, names: &[&str]) -> bool {
    value
        .as_object()
        .is_some_and(|m| m.keys().all(|k| names.contains(&k.as_str())))
}
fn invocation(value: &Value) -> Result<(), WorkerError> {
    if !closed(value, &["jsonrpc", "id", "method", "params"])
        || value["jsonrpc"] != "2.0"
        || value.get("id").is_none()
    {
        return Err(WorkerError::InvalidTaskInput);
    }
    let params = &value["params"];
    match value["method"].as_str() {
        Some("SendMessage") => {
            if !closed(params, &["message", "configuration"]) {
                return Err(WorkerError::InvalidTaskInput);
            }
            if let Some(configuration) = params.get("configuration") {
                if !closed(
                    configuration,
                    &["acceptedOutputModes", "returnImmediately", "historyLength"],
                ) || configuration
                    .get("acceptedOutputModes")
                    .is_some_and(|m| m != &json!(["application/json"]))
                    || configuration.get("historyLength").is_some_and(|n| n != 0)
                    || configuration
                        .get("returnImmediately")
                        .is_some_and(|b| b != false)
                {
                    return Err(WorkerError::InvalidTaskInput);
                }
            }
            let message = &params["message"];
            if !closed(
                message,
                &["messageId", "contextId", "taskId", "role", "parts"],
            ) || message["role"] != "ROLE_USER"
                || message["messageId"]
                    .as_str()
                    .is_none_or(|s| s.is_empty() || s.len() > 128)
            {
                return Err(WorkerError::InvalidTaskInput);
            }
            let parts = message["parts"]
                .as_array()
                .ok_or(WorkerError::InvalidTaskInput)?;
            if parts.len() != 1 || !closed(&parts[0], &["data"]) {
                return Err(WorkerError::InvalidTaskInput);
            }
            let envelope = &parts[0]["data"];
            if !closed(envelope, &["profileVersion", "packet"])
                || envelope["profileVersion"] != PROFILE_VERSION
                || envelope.get("packet").is_none()
            {
                return Err(WorkerError::InvalidTaskInput);
            }
            let packet = &envelope["packet"];
            if message["taskId"] != packet["taskId"]
                || message["contextId"] != packet["conversation"]["id"]
            {
                return Err(WorkerError::ContextMismatch);
            }
        }
        Some("GetTask" | "CancelTask") => {
            if !closed(params, &["id"])
                || params["id"]
                    .as_str()
                    .and_then(|s| uuid::Uuid::parse_str(s).ok())
                    .is_none()
            {
                return Err(WorkerError::InvalidTaskInput);
            }
        }
        _ => return Err(WorkerError::InvalidTaskInput),
    }
    Ok(())
}
async fn guard(State(state): State<ServiceState>, request: Request, next: Next) -> Response {
    let mut auth = request.headers().get_all("authorization").iter();
    let authorized = auth
        .next()
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .is_some_and(|token| {
            let hash = Sha256::digest(token.as_bytes());
            hash.iter()
                .zip(state.config.credential_hash)
                .fold(0u8, |n, (a, b)| n | (a ^ b))
                == 0
        });
    if !authorized || auth.next().is_some() || Utc::now() >= state.config.expires {
        return failure(WorkerError::AuthenticationRequired);
    }
    if request.uri().query().is_some() {
        return failure(WorkerError::InvalidTaskInput);
    }
    if request.method() == "POST" {
        let (parts, body) = request.into_parts();
        let Ok(bytes) = to_bytes(body, 2_097_152).await else {
            return failure(WorkerError::ContextTooLarge);
        };
        let value = match parse_json(&bytes) {
            Ok(v) => v,
            Err(_) => return failure(WorkerError::InvalidTaskInput),
        };
        if parts.uri.path() == "/" {
            let within_admission = crate::protocol::timestamp(
                &value["params"]["message"]["parts"][0]["data"]["packet"]["expiresAt"],
            )
            .is_ok_and(|expiry| expiry <= state.config.expires);
            let validation = invocation(&value).and_then(|()| {
                if value["method"] == "SendMessage" && !within_admission {
                    Err(WorkerError::AuthenticationRequired)
                } else {
                    Ok(())
                }
            });
            if let Err(e) = validation {
                return (StatusCode::OK,Json(json!({"jsonrpc":"2.0","id":value["id"],"error":{"code":-32602,"message":e.to_string()}}))).into_response();
            }
        }
        next.run(Request::from_parts(parts, Body::from(bytes)))
            .await
    } else {
        next.run(request).await
    }
}
async fn card() -> Json<Value> {
    let value = json!({"name":"ThoughtKhoral Codex Agent","description":"Explicitly addressed shared room participant","version":"0.1.0","supportedInterfaces":[{"url":"http://thought-khoral-codex-agent:9091","protocolBinding":"JSONRPC","protocolVersion":"1.0"}],"capabilities":{"streaming":false,"pushNotifications":false,"extensions":[{"uri":PROFILE_VERSION,"required":true,"params":{"agentId":"74686f75-6768-746b-686f-72616c000004","conversationScope":"room","invocation":"explicitly-addressed","delivery":"room","roomHistory":"baseline-and-delta","modelSelection":true,"reasoningEffort":true,"usageReporting":true}}]},"defaultInputModes":["application/json"],"defaultOutputModes":["application/json"],"skills":[{"id":"chat","name":"Room chat","description":"Answer an authorized room discussion","tags":["chat"]}],"securitySchemes":{"invocation":{"httpAuthSecurityScheme":{"scheme":"bearer"}}},"securityRequirements":[{"invocation":[]}]});
    Json(value)
}
async fn models(State(state): State<ServiceState>) -> Response {
    match state.worker.catalog().await {
        Ok(v) => Json(v).into_response(),
        Err(e) => failure(e),
    }
}
async fn receipt(State(state): State<ServiceState>, Path(task): Path<String>) -> Response {
    if uuid::Uuid::parse_str(&task).is_err() {
        return failure(WorkerError::InvalidTaskInput);
    }
    match state.worker.receipt(&task).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => failure(e),
    }
}
async fn ack(
    State(state): State<ServiceState>,
    Path(task): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    match state.worker.acknowledge(&task, body).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => failure(e),
    }
}
async fn rpc(State(state): State<ServiceState>, Json(value): Json<Value>) -> Response {
    let id = value["id"].clone();
    let params = value["params"].clone();
    let handler = Handler {
        worker: state.worker,
    };
    let service_params = ServiceParams::new();
    let result: Result<Value, A2AError> = match value["method"].as_str() {
        Some("SendMessage") => match serde_json::from_value(params) {
            Ok(request) => handler
                .send_message(&service_params, request)
                .await
                .and_then(|reply| {
                    serde_json::to_value(reply)
                        .map_err(|_| A2AError::internal("runtime_unavailable"))
                }),
            Err(_) => Err(A2AError::invalid_request("invalid_task_input")),
        },
        Some("GetTask") => match serde_json::from_value(params) {
            Ok(request) => handler
                .get_task(&service_params, request)
                .await
                .and_then(|reply| {
                    serde_json::to_value(reply)
                        .map_err(|_| A2AError::internal("runtime_unavailable"))
                }),
            Err(_) => Err(A2AError::invalid_request("invalid_task_input")),
        },
        Some("CancelTask") => match serde_json::from_value(params) {
            Ok(request) => handler
                .cancel_task(&service_params, request)
                .await
                .and_then(|reply| {
                    serde_json::to_value(reply)
                        .map_err(|_| A2AError::internal("runtime_unavailable"))
                }),
            Err(_) => Err(A2AError::invalid_request("invalid_task_input")),
        },
        _ => Err(A2AError::invalid_request("unsupported_operation")),
    };
    match result {
        Ok(result) => Json(json!({"jsonrpc":"2.0","id":id,"result":result})).into_response(),
        Err(error) => Json(
            json!({"jsonrpc":"2.0","id":id,"error":{"code":error.code,"message":error.message}}),
        )
        .into_response(),
    }
}
struct Handler {
    worker: Worker,
}
fn task(receipt: Value) -> Task {
    let result = receipt["result"].clone();
    let completed = receipt["phase"] == "completed";
    Task {
        id: receipt["taskId"].as_str().unwrap().into(),
        context_id: receipt["conversationId"].as_str().unwrap().into(),
        status: TaskStatus {
            state: if completed {
                TaskState::Completed
            } else if receipt["phase"] == "running" {
                TaskState::Working
            } else if receipt["phase"] == "reserved" {
                TaskState::Submitted
            } else {
                TaskState::Failed
            },
            message: if !completed && receipt["error"].is_string() {
                Some(Message::new(
                    Role::Agent,
                    vec![Part::data(json!({"code":receipt["error"]}))],
                ))
            } else {
                None
            },
            timestamp: None,
        },
        artifacts: completed.then(|| {
            vec![Artifact {
                artifact_id: "reply".into(),
                name: None,
                description: None,
                parts: vec![Part::data(
                    json!({"reply":result,"runtimeBinding":receipt["runtimeBinding"]}),
                )],
                metadata: None,
                extensions: None,
            }]
        }),
        history: None,
        metadata: None,
    }
}
#[async_trait]
impl RequestHandler for Handler {
    async fn send_message(
        &self,
        _: &ServiceParams,
        req: SendMessageRequest,
    ) -> Result<SendMessageResponse, A2AError> {
        let packet = req
            .message
            .parts
            .first()
            .and_then(|p| match &p.content {
                PartContent::Data(value) => Some(value),
                _ => None,
            })
            .ok_or_else(|| A2AError::invalid_request("invalid_task_input"))?["packet"]
            .clone();
        let id = packet["taskId"]
            .as_str()
            .ok_or_else(|| A2AError::invalid_request("invalid_task_input"))?
            .to_owned();
        if let Err(error) = self.worker.execute(packet).await {
            return Err(A2AError::invalid_request(error.to_string()));
        }
        let receipt = self
            .worker
            .receipt(&id)
            .await
            .map_err(|e| A2AError::internal(e.to_string()))?;
        Ok(SendMessageResponse::Task(task(receipt)))
    }
    async fn get_task(&self, _: &ServiceParams, req: GetTaskRequest) -> Result<Task, A2AError> {
        self.worker
            .receipt(&req.id)
            .await
            .map(task)
            .map_err(|e| A2AError::task_not_found(&e.to_string()))
    }
    async fn cancel_task(
        &self,
        _: &ServiceParams,
        req: CancelTaskRequest,
    ) -> Result<Task, A2AError> {
        self.worker
            .cancel(&req.id)
            .await
            .map(task)
            .map_err(|e| A2AError::invalid_request(e.to_string()))
    }
    async fn send_streaming_message(
        &self,
        _params: &ServiceParams,
        _req: SendMessageRequest,
    ) -> Result<BoxStream<'static, Result<StreamResponse, A2AError>>, A2AError> {
        Err(A2AError::invalid_request("unsupported_operation"))
    }
    async fn list_tasks(
        &self,
        _params: &ServiceParams,
        _req: ListTasksRequest,
    ) -> Result<ListTasksResponse, A2AError> {
        Err(A2AError::invalid_request("unsupported_operation"))
    }
    async fn subscribe_to_task(
        &self,
        _params: &ServiceParams,
        _req: SubscribeToTaskRequest,
    ) -> Result<BoxStream<'static, Result<StreamResponse, A2AError>>, A2AError> {
        Err(A2AError::invalid_request("unsupported_operation"))
    }
    async fn create_push_config(
        &self,
        _params: &ServiceParams,
        _req: TaskPushNotificationConfig,
    ) -> Result<TaskPushNotificationConfig, A2AError> {
        Err(A2AError::invalid_request("unsupported_operation"))
    }
    async fn get_push_config(
        &self,
        _params: &ServiceParams,
        _req: GetTaskPushNotificationConfigRequest,
    ) -> Result<TaskPushNotificationConfig, A2AError> {
        Err(A2AError::invalid_request("unsupported_operation"))
    }
    async fn list_push_configs(
        &self,
        _params: &ServiceParams,
        _req: ListTaskPushNotificationConfigsRequest,
    ) -> Result<ListTaskPushNotificationConfigsResponse, A2AError> {
        Err(A2AError::invalid_request("unsupported_operation"))
    }
    async fn delete_push_config(
        &self,
        _params: &ServiceParams,
        _req: DeleteTaskPushNotificationConfigRequest,
    ) -> Result<(), A2AError> {
        Err(A2AError::invalid_request("unsupported_operation"))
    }
    async fn get_extended_agent_card(
        &self,
        _params: &ServiceParams,
        _req: GetExtendedAgentCardRequest,
    ) -> Result<AgentCard, A2AError> {
        Err(A2AError::invalid_request("unsupported_operation"))
    }
}
