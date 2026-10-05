// SPDX-License-Identifier: Apache-2.0
use chrono::{Duration, Utc};
use serde_json::{Value, json};
use std::{ffi::OsString, path::PathBuf};
use tempfile::TempDir;
use thought_khoral_codex_agent::{config::Config, protocol::RuntimeRequest};
pub struct Fixture {
    pub directory: TempDir,
    pub config: Config,
    pub capture: PathBuf,
}
impl Fixture {
    pub fn new(scenario: &str) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let cwd = directory.path().join("workspace");
        let home = directory.path().join("native");
        std::fs::create_dir(&cwd).unwrap();
        std::fs::create_dir(&home).unwrap();
        let capture = directory.path().join("requests.jsonl");
        let mut config = Config::new(PathBuf::from("/usr/bin/python3"), cwd, home);
        config.arguments = vec![
            OsString::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/support/fake_app_server.py"
            )),
            "--scenario".into(),
            scenario.into(),
            "--capture".into(),
            capture.as_os_str().into(),
        ];
        config.deadline = std::time::Duration::from_secs(2);
        config.interrupt_grace = std::time::Duration::from_millis(100);
        config.model_allowlist = vec!["model-a".into()];
        Self {
            directory,
            config,
            capture,
        }
    }
    pub fn records(&self) -> Vec<Value> {
        std::fs::read_to_string(&self.capture)
            .unwrap_or_default()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    }
}
pub fn request() -> RuntimeRequest {
    let mut packet: Value = serde_json::from_str(include_str!(
        "../../contracts/agent-conversation-v1/fixtures/valid/baseline-hidden-sequence-gap.json"
    ))
    .unwrap();
    let now = Utc::now();
    packet["issuedAt"] = json!(now);
    packet["expiresAt"] = json!(now + Duration::seconds(180));
    packet["authorizationExpiresAt"] = json!(now + Duration::seconds(185));
    packet["leaseExpiresAt"] = packet["authorizationExpiresAt"].clone();
    RuntimeRequest {
        packet,
        thread_id: None,
    }
}
