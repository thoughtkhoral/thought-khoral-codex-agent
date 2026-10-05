// SPDX-License-Identifier: Apache-2.0
use chrono::{DateTime, Utc};
use jsonschema::Resource;
use serde::{
    Deserialize,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};
pub const PROFILE_VERSION: &str = "thought-khoral.agent-conversation.v1";
pub const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeError {
    InvalidTaskInput,
    ContextTooLarge,
    ContextMismatch,
    RuntimeUnavailable,
    AuthenticationRequired,
    SessionUnavailable,
    Timeout,
    ConversationInterrupted,
    ExecutionFailed,
}
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidTaskInput => "invalid_task_input",
            Self::ContextTooLarge => "context_too_large",
            Self::ContextMismatch => "context_mismatch",
            Self::RuntimeUnavailable => "runtime_unavailable",
            Self::AuthenticationRequired => "authentication_required",
            Self::SessionUnavailable => "session_unavailable",
            Self::Timeout => "timeout",
            Self::ConversationInterrupted => "conversation_interrupted",
            Self::ExecutionFailed => "execution_failed",
        })
    }
}
impl std::error::Error for RuntimeError {}
impl From<std::io::Error> for RuntimeError {
    fn from(_: std::io::Error) -> Self {
        Self::RuntimeUnavailable
    }
}
pub struct RuntimeRequest {
    pub packet: Value,
    pub thread_id: Option<String>,
}
#[derive(Debug)]
pub struct RuntimeOutcome {
    pub thread_id: String,
    pub turn_id: String,
    pub reply: Value,
}
static PROFILE: LazyLock<HashMap<&'static str, jsonschema::Validator>> = LazyLock::new(|| {
    let schemas = [
        (
            "input",
            include_str!("../contracts/agent-conversation-v1/schemas/input.schema.json"),
        ),
        (
            "turn",
            include_str!("../contracts/agent-conversation-v1/schemas/turn.schema.json"),
        ),
        (
            "result",
            include_str!("../contracts/agent-conversation-v1/schemas/result.schema.json"),
        ),
        (
            "catalog",
            include_str!("../contracts/agent-conversation-v1/schemas/catalog.schema.json"),
        ),
    ]
    .map(|(name, text)| (name, serde_json::from_str::<Value>(text).unwrap()));
    schemas
        .iter()
        .map(|(name, schema)| {
            (
                *name,
                jsonschema::draft202012::options()
                    .should_validate_formats(true)
                    .with_resources(schemas.iter().map(|(_, s)| {
                        (
                            s["$id"].as_str().unwrap().to_owned(),
                            Resource::from_contents(s.clone()),
                        )
                    }))
                    .build(schema)
                    .unwrap(),
            )
        })
        .collect()
});
static NATIVE: LazyLock<HashMap<&'static str, jsonschema::Validator>> = LazyLock::new(|| {
    let schema: Value = serde_json::from_str(include_str!(
        "../contracts/codex-app-server-0.160.0/schema.json"
    ))
    .unwrap();
    let names = [
        "InitializeParams",
        "ThreadStartParams",
        "ThreadResumeParams",
        "TurnStartParams",
        "ModelListParams",
        "ThreadStartResponse",
        "ThreadResumeResponse",
        "TurnStartResponse",
        "ModelListResponse",
        "ThreadStartedNotification",
        "TurnStartedNotification",
        "ItemStartedNotification",
        "ItemCompletedNotification",
        "TurnCompletedNotification",
        "ThreadSettingsUpdatedNotification",
        "ThreadTokenUsageUpdatedNotification",
        "ModelReroutedNotification",
        "AgentMessageDeltaNotification",
        "ContextCompactedNotification",
        "ReasoningTextDeltaNotification",
        "ReasoningSummaryTextDeltaNotification",
        "ReasoningSummaryPartAddedNotification",
        "ThreadStatusChangedNotification",
    ];
    let mut validators: HashMap<_, _> = names
        .into_iter()
        .filter(|name| schema["definitions"].get(*name).is_some())
        .map(|name| {
            let mut selected = schema.clone();
            selected["$ref"] = json!(format!("#/definitions/{name}"));
            (
                name,
                jsonschema::draft7::options().build(&selected).unwrap(),
            )
        })
        .collect();
    let initialize: Value = serde_json::from_str(include_str!(
        "../contracts/codex-app-server-0.160.0/initialize-response.json"
    ))
    .unwrap();
    validators.insert(
        "InitializeResponse",
        jsonschema::draft7::options().build(&initialize).unwrap(),
    );
    validators
});
pub fn validate_native(name: &str, value: &Value) -> Result<(), RuntimeError> {
    if NATIVE.get(name).is_some_and(|v| v.is_valid(value)) {
        Ok(())
    } else {
        Err(RuntimeError::RuntimeUnavailable)
    }
}
pub fn validate_profile(name: &str, value: &Value) -> Result<(), RuntimeError> {
    if PROFILE.get(name).is_some_and(|v| v.is_valid(value)) {
        Ok(())
    } else {
        Err(RuntimeError::InvalidTaskInput)
    }
}
pub fn canonical_bytes(value: &Value) -> Result<Vec<u8>, RuntimeError> {
    fn canonical(value: &Value) -> Result<Value, RuntimeError> {
        match value {
            Value::Number(n) if n.as_u64().is_none_or(|n| n > MAX_SAFE_INTEGER) => {
                Err(RuntimeError::InvalidTaskInput)
            }
            Value::Object(map) => {
                let mut output = serde_json::Map::new();
                let mut keys = map.keys().collect::<Vec<_>>();
                keys.sort_unstable();
                for key in keys {
                    if !key.is_ascii() {
                        return Err(RuntimeError::InvalidTaskInput);
                    }
                    output.insert(key.clone(), canonical(&map[key])?);
                }
                Ok(Value::Object(output))
            }
            Value::Array(values) => Ok(Value::Array(
                values.iter().map(canonical).collect::<Result<_, _>>()?,
            )),
            _ => Ok(value.clone()),
        }
    }
    serde_json::to_vec(&canonical(value)?).map_err(|_| RuntimeError::InvalidTaskInput)
}
pub fn context_digest(packet: &Value) -> Result<String, RuntimeError> {
    let mut context = packet["context"].clone();
    context
        .as_object_mut()
        .ok_or(RuntimeError::InvalidTaskInput)?
        .remove("digest");
    let preimage = json!({"roomId":packet["roomId"],"agentId":packet["agentId"],"conversationId":packet["conversation"]["id"],"generation":packet["conversation"]["generation"],"triggerEventId":packet["triggerEventId"],"guidanceRevision":packet["guidanceRevision"],"context":context});
    let bytes = canonical_bytes(&preimage)?;
    if bytes.len() > 1_048_576 {
        return Err(RuntimeError::ContextTooLarge);
    }
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
pub fn timestamp(value: &Value) -> Result<DateTime<Utc>, RuntimeError> {
    DateTime::parse_from_rfc3339(value.as_str().ok_or(RuntimeError::InvalidTaskInput)?)
        .map(|time| time.with_timezone(&Utc))
        .map_err(|_| RuntimeError::InvalidTaskInput)
}
pub fn validate_request(request: &RuntimeRequest, guidance: &str) -> Result<(), RuntimeError> {
    let p = &request.packet;
    validate_profile("input", p)?;
    if p["agentId"] != "74686f75-6768-746b-686f-72616c000004" || p["guidanceRevision"] != guidance {
        return Err(RuntimeError::ContextMismatch);
    }
    if context_digest(p)? != p["context"]["digest"] {
        return Err(RuntimeError::ContextMismatch);
    }
    let issued = timestamp(&p["issuedAt"])?;
    let end = timestamp(&p["expiresAt"])?;
    let auth = timestamp(&p["authorizationExpiresAt"])?;
    let lease = timestamp(&p["leaseExpiresAt"])?;
    if end <= Utc::now() || auth <= Utc::now() || lease <= Utc::now() {
        return Err(RuntimeError::AuthenticationRequired);
    }
    if end <= issued || (end - issued).num_milliseconds() > 180_000 || end > auth || lease > auth {
        return Err(RuntimeError::InvalidTaskInput);
    }
    let new = p["conversation"]["mode"] == "new";
    if new != request.thread_id.is_none()
        || request
            .thread_id
            .as_ref()
            .is_some_and(|id| id.is_empty() || id.len() > 256)
    {
        return Err(RuntimeError::SessionUnavailable);
    }
    let base = p["context"]["baseRevision"]
        .as_u64()
        .ok_or(RuntimeError::InvalidTaskInput)?;
    let revision = p["context"]["revision"]
        .as_u64()
        .ok_or(RuntimeError::InvalidTaskInput)?;
    if base >= revision
        || (new && (base != 0 || p["context"]["kind"] != "baseline"))
        || (!new && (base == 0 || p["context"]["kind"] != "delta"))
    {
        return Err(RuntimeError::ContextMismatch);
    }
    let entries = p["context"]["entries"]
        .as_array()
        .ok_or(RuntimeError::InvalidTaskInput)?;
    let bindings = p["context"]["nativeReplyBindings"]
        .as_array()
        .ok_or(RuntimeError::InvalidTaskInput)?;
    if entries.len() + bindings.len() > 2000 || (new && !bindings.is_empty()) {
        return Err(RuntimeError::ContextTooLarge);
    }
    if bindings
        .iter()
        .any(|binding| binding["generation"] != p["conversation"]["generation"])
    {
        return Err(RuntimeError::ContextMismatch);
    }
    let mut sources = HashSet::new();
    let mut sequences = HashSet::new();
    let mut prior = base;
    let mut trigger = 0;
    for entry in entries.iter().chain(bindings) {
        let sequence = entry["sequence"]
            .as_u64()
            .ok_or(RuntimeError::InvalidTaskInput)?;
        if sequence <= base
            || sequence > revision
            || !sequences.insert(sequence)
            || !sources.insert(entry["eventId"].as_str())
        {
            return Err(RuntimeError::ContextMismatch);
        }
    }
    for entry in entries {
        let sequence = entry["sequence"].as_u64().unwrap();
        if sequence <= prior {
            return Err(RuntimeError::ContextMismatch);
        }
        prior = sequence;
        if entry["eventId"] == p["triggerEventId"] {
            trigger += 1;
            if sequence != revision
                || entry["authorRole"] != "human"
                || entry["authorId"] != p["requesterId"]
                || entry["text"]
                    .as_str()
                    .is_none_or(|s| s.chars().count() > 8000)
            {
                return Err(RuntimeError::ContextMismatch);
            }
        }
    }
    if trigger != 1 {
        return Err(RuntimeError::ContextMismatch);
    }
    Ok(())
}
/// Reject duplicate decoded keys before correlation; raw lines are never diagnostics.
pub fn parse_json(bytes: &[u8]) -> Result<Value, RuntimeError> {
    let mut decoder = serde_json::Deserializer::from_slice(bytes);
    let value = StrictValue::deserialize(&mut decoder)
        .map_err(|_| RuntimeError::RuntimeUnavailable)?
        .0;
    decoder
        .end()
        .map_err(|_| RuntimeError::RuntimeUnavailable)?;
    Ok(value)
}
struct StrictValue(Value);
impl<'de> Deserialize<'de> for StrictValue {
    fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        struct StrictVisitor;
        impl<'de> Visitor<'de> for StrictVisitor {
            type Value = StrictValue;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("unique-key JSON")
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
                Ok(StrictValue(json!(v)))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(StrictValue(json!(v)))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(StrictValue(json!(v)))
            }
            fn visit_f64<E: de::Error>(self, _: f64) -> Result<Self::Value, E> {
                Err(E::custom("float"))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(StrictValue(json!(v)))
            }
            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(StrictValue(json!(v)))
            }
            fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(StrictValue(Value::Null))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut values = vec![];
                while let Some(v) = seq.next_element::<StrictValue>()? {
                    values.push(v.0)
                }
                Ok(StrictValue(Value::Array(values)))
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut values = serde_json::Map::new();
                while let Some(key) = map.next_key::<String>()? {
                    if values.contains_key(&key) {
                        return Err(de::Error::custom("duplicate key"));
                    }
                    values.insert(key, map.next_value::<StrictValue>()?.0);
                }
                Ok(StrictValue(Value::Object(values)))
            }
        }
        decoder.deserialize_any(StrictVisitor)
    }
}
pub(crate) fn prepare() {
    LazyLock::force(&PROFILE);
    LazyLock::force(&NATIVE);
}
