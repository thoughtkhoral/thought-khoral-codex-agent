// SPDX-License-Identifier: Apache-2.0
mod support;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sha2::Digest;
use support::{Fixture, request};
use thought_khoral_codex_agent::{
    a2a_service::{ServiceConfig, service},
    worker::Worker,
};
use tower::ServiceExt;
async fn call(app: Router, path: &str, body: Option<Value>, auth: bool) -> (StatusCode, Value) {
    let mut r = Request::builder()
        .uri(path)
        .method(if body.is_some() { "POST" } else { "GET" });
    if auth {
        r = r.header("Authorization", "Bearer synthetic-invocation-token");
    }
    let r = r
        .header("Content-Type", "application/json")
        .body(Body::from(body.map(|v| v.to_string()).unwrap_or_default()))
        .unwrap();
    let response = app.oneshot(r).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}
fn message(packet: Value) -> Value {
    json!({"jsonrpc":"2.0","id":"rpc-1","method":"SendMessage","params":{"message":{"messageId":packet["taskId"],"taskId":packet["taskId"],"contextId":packet["conversation"]["id"],"role":"ROLE_USER","parts":[{"data":{"profileVersion":packet["profileVersion"],"packet":packet}}]}}})
}
fn worker_fixture(scenario: &str) -> Fixture {
    let mut fixture = Fixture::new(scenario);
    fixture.config.deadline = std::time::Duration::from_secs(10);
    fixture
}
#[tokio::test]
async fn card_control_and_task_transport_require_distinct_invocation_authentication() {
    let f = worker_fixture("success");
    let worker = Worker::open(
        f.config.clone(),
        &f.directory.path().join("receipts.sqlite"),
    )
    .await
    .unwrap();
    let app = service(
        worker.clone(),
        ServiceConfig::new(
            "synthetic-invocation-token".into(),
            chrono::Utc::now() + chrono::Duration::hours(1),
        ),
    )
    .unwrap();
    for path in [
        "/.well-known/agent-card.json",
        "/control/v1/models",
        "/control/v1/receipts/00000006-1111-4111-8111-000000000006",
    ] {
        assert_eq!(
            call(app.clone(), path, None, false).await.0,
            StatusCode::UNAUTHORIZED
        );
    }
    let card = call(app.clone(), "/.well-known/agent-card.json", None, true).await;
    assert_eq!(card.0, StatusCode::OK);
    let _: a2a::AgentCard = serde_json::from_value(card.1.clone()).unwrap();
    assert_eq!(card.1["skills"][0]["id"], "chat");
    let catalog = call(app.clone(), "/control/v1/models", None, true).await;
    assert_eq!(catalog.0, StatusCode::OK);
    assert_eq!(catalog.1["data"][0]["id"], "model-a");
    let packet = request().packet;
    let body = message(packet.clone());
    let (status, result) = call(app.clone(), "/", Some(body.clone()), true).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        result["result"]["task"]["status"]["state"], "TASK_STATE_COMPLETED",
        "{result}"
    );
    assert_eq!(
        result["result"]["task"]["artifacts"][0]["parts"][0]["data"]["reply"]["assistantText"],
        "Maya proposed green."
    );
    let artifact = &result["result"]["task"]["artifacts"][0]["parts"][0]["data"];
    assert_eq!(artifact.as_object().unwrap().len(), 2);
    assert_eq!(
        artifact["runtimeBinding"],
        json!({"threadId":"thread-exact","turnId":"turn-exact"})
    );
    assert_eq!(
        call(app.clone(), "/", Some(body), true).await.1["result"],
        result["result"]
    );
    let task = packet["taskId"].as_str().unwrap();
    let path = format!("/control/v1/receipts/{task}");
    assert_eq!(
        call(app.clone(), &path, None, true).await.1["phase"],
        "completed"
    );
    let receipt = call(app.clone(), &path, None, true).await.1;
    assert_eq!(receipt["runtimeBinding"], artifact["runtimeBinding"]);
    assert_eq!(receipt["result"], artifact["reply"]);
    assert_eq!(receipt.as_object().unwrap().len(), 8);
    let ack = json!({"profileVersion":packet["profileVersion"],"taskId":task,"conversationId":packet["conversation"]["id"],"generation":packet["conversation"]["generation"],"replyEventId":"00000016-1111-4111-8111-000000000016","replySequence":6,"textDigest":format!("{:x}",sha2::Sha256::digest(b"Maya proposed green.")),"consumedRevision":5,"contextDigest":packet["context"]["digest"]});
    assert_eq!(
        call(
            app.clone(),
            &format!("{path}/ack"),
            Some(ack.clone()),
            false
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        call(app.clone(), &format!("{path}/ack"), Some(ack.clone()), true)
            .await
            .0,
        StatusCode::NO_CONTENT
    );
    assert_eq!(worker.receipt(task).await.unwrap()["acknowledgement"], ack);
    let polled = call(
        app.clone(),
        "/",
        Some(json!({"jsonrpc":"2.0","id":2,"method":"GetTask","params":{"id":task}})),
        true,
    )
    .await
    .1;
    assert_eq!(polled["result"]["id"], task);
    assert_eq!(
        polled["result"]["artifacts"][0]["parts"][0]["data"],
        *artifact
    );
    let canceled = call(
        app.clone(),
        "/",
        Some(json!({"jsonrpc":"2.0","id":3,"method":"CancelTask","params":{"id":task}})),
        true,
    )
    .await
    .1;
    assert_eq!(
        canceled["result"]["artifacts"][0]["parts"][0]["data"],
        *artifact
    );
    assert_eq!(
        f.records()
            .iter()
            .filter(|r| r["request"]["method"] == "turn/start")
            .count(),
        1
    );
}
#[tokio::test]
async fn unsupported_parts_handoffs_mismatched_task_and_expired_admission_fail_before_inference() {
    let f = worker_fixture("success");
    let worker = Worker::open(
        f.config.clone(),
        &f.directory.path().join("receipts.sqlite"),
    )
    .await
    .unwrap();
    let app = service(
        worker.clone(),
        ServiceConfig::new(
            "synthetic-invocation-token".into(),
            chrono::Utc::now() + chrono::Duration::hours(1),
        ),
    )
    .unwrap();
    for case in [
        "bare",
        "envelope_extra",
        "profile",
        "url",
        "extra",
        "task",
        "handoff",
        "tool",
    ] {
        let mut body = message(request().packet);
        match case {
            "bare" => body["params"]["message"]["parts"][0]["data"] = request().packet,
            "envelope_extra" => {
                body["params"]["message"]["parts"][0]["data"]["extra"] = json!(true)
            }
            "profile" => {
                body["params"]["message"]["parts"][0]["data"]["profileVersion"] = json!("wrong")
            }
            "url" => {
                body["params"]["message"]["parts"] =
                    json!([{"url":"https://untrusted.invalid/artifact"}])
            }
            "extra" => body["params"]["message"]["parts"]
                .as_array_mut()
                .unwrap()
                .push(json!({"text":"extra"})),
            "task" => {
                body["params"]["message"]["taskId"] = json!("00000017-1111-4111-8111-000000000017")
            }
            "handoff" => body["params"]["message"]["referenceTaskIds"] = json!(["untrusted"]),
            "tool" => body["method"] = json!("ExecuteTool"),
            _ => unreachable!(),
        }
        let result = call(app.clone(), "/", Some(body), true).await;
        assert!(
            result.1.get("error").is_some() || !result.0.is_success(),
            "{case}: {result:?}"
        );
    }
    let expired = service(
        worker,
        ServiceConfig::new(
            "synthetic-invocation-token".into(),
            chrono::Utc::now() - chrono::Duration::seconds(1),
        ),
    )
    .unwrap();
    assert_eq!(
        call(expired, "/", Some(message(request().packet)), true)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert!(f.records().is_empty());
}

#[tokio::test]
async fn admission_cannot_authorize_a_turn_beyond_its_expiry() {
    let f = worker_fixture("success");
    let worker = Worker::open(
        f.config.clone(),
        &f.directory.path().join("receipts.sqlite"),
    )
    .await
    .unwrap();
    let app = service(
        worker,
        ServiceConfig::new(
            "synthetic-invocation-token".into(),
            chrono::Utc::now() + chrono::Duration::seconds(2),
        ),
    )
    .unwrap();
    let result = call(app, "/", Some(message(request().packet)), true).await;
    assert!(!result.0.is_success() || result.1.get("error").is_some());
    assert!(f.records().is_empty());
}

#[tokio::test]
async fn a2a_cancel_stops_active_work_and_get_task_has_no_native_history() {
    let mut f = worker_fixture("hang");
    f.config.deadline = std::time::Duration::from_secs(10);
    let worker = Worker::open(
        f.config.clone(),
        &f.directory.path().join("receipts.sqlite"),
    )
    .await
    .unwrap();
    let app = service(
        worker.clone(),
        ServiceConfig::new(
            "synthetic-invocation-token".into(),
            chrono::Utc::now() + chrono::Duration::hours(1),
        ),
    )
    .unwrap();
    let packet = request().packet;
    let task = packet["taskId"].as_str().unwrap().to_owned();
    let send = tokio::spawn(call(app.clone(), "/", Some(message(packet)), true));
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            if worker
                .receipt(&task)
                .await
                .is_ok_and(|r| r["phase"] == "running")
            {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    let get = call(
        app.clone(),
        "/",
        Some(json!({"jsonrpc":"2.0","id":2,"method":"GetTask","params":{"id":task}})),
        true,
    )
    .await
    .1;
    assert_eq!(get["result"]["status"]["state"], "TASK_STATE_WORKING");
    assert!(!get.to_string().contains("thread-exact"));
    let cancel = call(
        app,
        "/",
        Some(json!({"jsonrpc":"2.0","id":3,"method":"CancelTask","params":{"id":task}})),
        true,
    )
    .await
    .1;
    assert_eq!(cancel["result"]["status"]["state"], "TASK_STATE_FAILED");
    assert!(send.await.unwrap().1.get("error").is_some());
    let receipt = worker.receipt(&task).await.unwrap();
    assert!(receipt["result"].is_null());
    assert!(receipt["runtimeBinding"].is_null());
}

#[tokio::test]
async fn provider_key_in_native_binding_is_rejected_before_persistence() {
    for identifier in ["thread-exact", "turn-exact"] {
        let mut f = worker_fixture("success");
        let secret = f.directory.path().join("provider-key");
        std::fs::write(&secret, "synthetic-provider-key").unwrap();
        f.config.provider = Some(std::sync::Arc::new(
            thought_khoral_codex_agent::config::ProviderCredentials::from_file(&secret).unwrap(),
        ));
        let script = f.directory.path().join("malicious-runtime.py");
        std::fs::write(
            &script,
            include_str!("support/fake_app_server.py")
                .replace(identifier, "synthetic-provider-key"),
        )
        .unwrap();
        f.config.arguments[0] = script.into_os_string();
        let db = f.directory.path().join("receipts.sqlite");
        let worker = Worker::open(f.config.clone(), &db).await.unwrap();
        let packet = request().packet;
        assert_eq!(
            worker.execute(packet.clone()).await.unwrap_err(),
            thought_khoral_codex_agent::worker::WorkerError::ExecutionFailed
        );
        assert!(
            !worker
                .receipt(packet["taskId"].as_str().unwrap())
                .await
                .unwrap()
                .to_string()
                .contains("synthetic-provider-key")
        );
        let pool = sqlx::SqlitePool::connect(&format!("sqlite://{}", db.display()))
            .await
            .unwrap();
        let row: (Option<String>, Option<String>) =
            sqlx::query_as("SELECT thread_id,turn_id FROM receipts")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert!(!format!("{row:?}").contains("synthetic-provider-key"));
    }
}
