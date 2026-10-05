// SPDX-License-Identifier: Apache-2.0
mod support;
use serde_json::json;
use support::{Fixture, request};
use thought_khoral_codex_agent::{app_server::AppServer, protocol::RuntimeError};

#[tokio::test]
async fn real_process_requests_are_ordered_explicit_and_deliver_one_trigger() {
    let fixture = Fixture::new("success");
    let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
    let barrier = fixture.directory.path().join("submission-intent");
    let outcome = server
        .execute(request(), |thread| async {
            assert_eq!(thread, "thread-exact");
            std::fs::write(&barrier, thread).unwrap();
            Ok(())
        })
        .await
        .unwrap();
    assert_eq!(outcome.thread_id, "thread-exact");
    assert_eq!(outcome.turn_id, "turn-exact");
    assert_eq!(outcome.reply["assistantText"], "Maya proposed green.");
    assert!(barrier.exists());
    let records = fixture.records();
    let methods = records
        .iter()
        .map(|r| r["request"]["method"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        methods,
        vec![
            "initialize",
            "initialized",
            "model/list",
            "model/list",
            "thread/start",
            "turn/start"
        ]
    );
    let start = &records[4]["request"]["params"];
    assert_eq!(start["sandbox"], "read-only");
    assert_eq!(start["approvalPolicy"], "never");
    assert_eq!(start["config"]["web_search"], "disabled");
    assert_eq!(start["config"]["mcp_servers"], json!({}));
    assert_eq!(start["config"]["plugins"], json!({}));
    assert_eq!(start["config"]["project_doc_max_bytes"], 0);
    assert!(
        start["config"]["features"]
            .as_object()
            .unwrap()
            .values()
            .all(|value| value == false)
    );
    assert!(
        records
            .iter()
            .all(|record| record["environment"]["operatorSecretPresent"] == false)
    );
    assert_eq!(start["model"], "test-native-model");
    assert_eq!(start["config"]["model_reasoning_effort"], "medium");
    let turn = &records[5]["request"]["params"];
    assert_eq!(turn["effort"], "medium");
    assert_eq!(
        turn["sandboxPolicy"],
        json!({"type":"readOnly","networkAccess":false})
    );
    assert_eq!(turn["approvalPolicy"], "never");
    let text = turn["input"][0]["text"].as_str().unwrap();
    assert_eq!(
        text.matches("@codex-agent What did Maya propose?").count(),
        1
    );
    assert!(text.contains("Use the blue example."));
    assert!(text.contains("The example is green."));
    assert!(!text.contains("private secret"));
    server.close().await.unwrap();
}
#[tokio::test]
async fn barrier_failure_prevents_turn_submission() {
    let fixture = Fixture::new("success");
    let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
    assert_eq!(
        server
            .execute(request(), |_| async { Err(RuntimeError::ExecutionFailed) })
            .await
            .unwrap_err(),
        RuntimeError::ExecutionFailed
    );
    assert!(
        !fixture
            .records()
            .iter()
            .any(|r| r["request"]["method"] == "turn/start")
    );
    server.close().await.unwrap();
}
#[tokio::test]
async fn commentary_incomplete_deltas_and_protocol_failures_never_succeed() {
    for scenario in [
        "commentary_only",
        "delta_only",
        "failed",
        "interrupted",
        "early_exit",
        "malformed",
        "duplicate_key",
        "overflow",
        "stderr_overflow",
        "tool_request",
        "approval",
        "wrong_thread",
        "wrong_turn",
        "rpc_wrong_id",
        "incomplete",
        "text_overflow",
        "duplicate_catalog",
        "cycle_catalog",
    ] {
        let fixture = Fixture::new(scenario);
        let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
        assert!(
            server
                .execute(request(), |_| async { Ok(()) })
                .await
                .is_err(),
            "{scenario} must fail"
        );
        server.close().await.unwrap();
    }
}
#[tokio::test]
async fn only_completed_final_items_are_joined_and_missing_phase_uses_last_message() {
    for (scenario, expected) in [
        ("multiple_final", "Maya proposed green.\nLeo agreed."),
        ("phase_missing", "Maya proposed green."),
    ] {
        let fixture = Fixture::new(scenario);
        let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
        assert_eq!(
            server
                .execute(request(), |_| async { Ok(()) })
                .await
                .unwrap()
                .reply["assistantText"],
            expected
        );
        server.close().await.unwrap();
    }
}
#[tokio::test]
async fn deadline_interrupts_the_exact_turn_and_reaps_an_ignoring_process() {
    for scenario in ["hang", "ignore_interrupt"] {
        let fixture = Fixture::new(scenario);
        let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
        assert_eq!(
            server
                .execute(request(), |_| async { Ok(()) })
                .await
                .unwrap_err(),
            RuntimeError::Timeout
        );
        let records = fixture.records();
        assert!(
            records
                .iter()
                .any(|r| r["request"]["method"] == "turn/interrupt"
                    && r["request"]["params"]
                        == json!({"threadId":"thread-exact","turnId":"turn-exact"}))
        );
        server.close().await.unwrap();
    }
}

#[tokio::test]
async fn continuation_resumes_the_exact_stored_id_without_importing_history() {
    let fixture = Fixture::new("success");
    let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
    let mut input = request();
    input.packet = serde_json::from_str(include_str!(
        "../contracts/agent-conversation-v1/fixtures/valid/native-reply-substitution.json"
    ))
    .unwrap();
    let times = request().packet;
    for field in [
        "issuedAt",
        "expiresAt",
        "authorizationExpiresAt",
        "leaseExpiresAt",
    ] {
        input.packet[field] = times[field].clone();
    }
    input.thread_id = Some("stored-exact-id".into());
    let result = server
        .execute(input, |thread| async move {
            assert_eq!(thread, "stored-exact-id");
            Ok(())
        })
        .await
        .unwrap();
    assert_eq!(result.thread_id, "stored-exact-id");
    let records = fixture.records();
    let resumed = records
        .iter()
        .find(|record| record["request"]["method"] == "thread/resume")
        .unwrap();
    assert_eq!(resumed["request"]["params"]["threadId"], "stored-exact-id");
    assert!(resumed["request"]["params"].get("path").is_none());
    assert!(resumed["request"]["params"].get("history").is_none());
    assert!(
        !records
            .iter()
            .any(|record| record["request"]["method"] == "thread/start")
    );
    server.close().await.unwrap();
}
#[tokio::test]
async fn normal_reasoning_and_status_notifications_are_never_public_output() {
    let fixture = Fixture::new("noise");
    let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
    let reply = server
        .execute(request(), |_| async { Ok(()) })
        .await
        .unwrap()
        .reply;
    assert_eq!(reply["assistantText"], "Maya proposed green.");
    assert!(!reply.to_string().contains("Private reasoning"));
    server.close().await.unwrap();
}
#[tokio::test]
async fn dropping_an_execution_future_kills_the_child_even_while_server_is_retained() {
    let fixture = Fixture::new("hang");
    let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
    let mut execution = Box::pin(server.execute(request(), |_| async { Ok(()) }));
    let ready = async {
        loop {
            if fixture
                .records()
                .iter()
                .any(|r| r["request"]["method"] == "turn/start")
            {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    };
    tokio::select! {
        result = &mut execution => panic!("execution ended before cancellation: {result:?}"),
        result = tokio::time::timeout(std::time::Duration::from_secs(2), ready) => result.unwrap(),
    }
    drop(execution);
    let pid = fixture.records().first().unwrap()["pid"].as_u64().unwrap() as i32;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    // Retained AppServer must not retain live work after cancellation.
    assert_ne!(unsafe { libc::kill(pid, 0) }, 0);
    server.close().await.unwrap();
}
#[tokio::test]
async fn wrong_generation_native_bindings_cannot_reach_a_resume_or_submission() {
    let fixture = Fixture::new("success");
    let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
    let mut input = request();
    input.packet = serde_json::from_str(include_str!(
        "../contracts/agent-conversation-v1/fixtures/valid/native-reply-substitution.json"
    ))
    .unwrap();
    let times = request().packet;
    for field in [
        "issuedAt",
        "expiresAt",
        "authorizationExpiresAt",
        "leaseExpiresAt",
    ] {
        input.packet[field] = times[field].clone();
    }
    input.thread_id = Some("stored-exact-id".into());
    input.packet["context"]["nativeReplyBindings"][0]["generation"] = json!(2);
    input.packet["context"]["digest"] =
        json!(thought_khoral_codex_agent::protocol::context_digest(&input.packet).unwrap());
    assert_eq!(
        server
            .execute(input, |_| async { Ok(()) })
            .await
            .unwrap_err(),
        RuntimeError::ContextMismatch
    );
    assert!(fixture.records().is_empty());
    server.close().await.unwrap();
}

#[tokio::test]
async fn settings_model_change_invalidates_the_previous_usage_window() {
    let mut fixture = Fixture::new("settings_switch");
    fixture.config.model_allowlist.push("blocked".into());
    let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
    let reply = server
        .execute(request(), |_| async { Ok(()) })
        .await
        .unwrap()
        .reply;
    assert_eq!(reply["effectiveSettings"]["model"], "blocked");
    assert_eq!(reply["effectiveSettings"]["confirmation"], "confirmed");
    assert!(
        reply["usage"].is_null(),
        "old model's usage must be invalidated"
    );
    server.close().await.unwrap();
}

#[tokio::test]
async fn unavailable_effort_is_never_presented_as_runtime_confirmation() {
    let fixture = Fixture::new("null_effort");
    let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
    let reply = server
        .execute(request(), |_| async { Ok(()) })
        .await
        .unwrap()
        .reply;
    assert_eq!(reply["effectiveSettings"]["confirmation"], "unconfirmed");
    assert!(reply["effectiveSettings"]["reasoningEffort"].is_null());
    server.close().await.unwrap();
}
#[tokio::test]
async fn invalid_packets_expired_authority_and_catalog_choices_cannot_submit() {
    for case in [
        "digest", "expired", "guidance", "model", "effort", "catalog",
    ] {
        let fixture = Fixture::new("success");
        let mut input = request();
        match case {
            "digest" => input.packet["context"]["digest"] = json!("0".repeat(64)),
            "expired" => {
                input.packet["authorizationExpiresAt"] =
                    json!(chrono::Utc::now() - chrono::Duration::seconds(1))
            }
            "guidance" => input.packet["guidanceRevision"] = json!("unapproved"),
            "model" => input.packet["model"] = json!("blocked"),
            "effort" => input.packet["reasoningEffort"] = json!("effort-unsupported"),
            "catalog" => input.packet["catalogRevision"] = json!("unknown"),
            _ => unreachable!(),
        }
        let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
        assert!(
            server.execute(input, |_| async { Ok(()) }).await.is_err(),
            "{case}"
        );
        assert!(
            !fixture.records().iter().any(|r| matches!(
                r["request"]["method"].as_str(),
                Some("thread/start" | "thread/resume" | "turn/start")
            )),
            "{case}"
        );
        server.close().await.unwrap();
    }
}
#[tokio::test]
async fn dedicated_process_cannot_execute_a_second_turn_without_acknowledgement() {
    let fixture = Fixture::new("success");
    let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
    server
        .execute(request(), |_| async { Ok(()) })
        .await
        .unwrap();
    assert_eq!(
        server
            .execute(request(), |_| async { Ok(()) })
            .await
            .unwrap_err(),
        RuntimeError::ConversationInterrupted
    );
    assert_eq!(
        fixture
            .records()
            .iter()
            .filter(|r| r["request"]["method"] == "turn/start")
            .count(),
        1
    );
    server.close().await.unwrap();
}
#[tokio::test]
async fn deadline_kills_descendant_processes_and_reaps_the_app_server() {
    let fixture = Fixture::new("descendant");
    let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
    assert_eq!(
        server
            .execute(request(), |_| async { Ok(()) })
            .await
            .unwrap_err(),
        RuntimeError::Timeout
    );
    let parent = fixture.records()[0]["pid"].as_u64().unwrap() as i32;
    assert_ne!(
        unsafe { libc::kill(parent, 0) },
        0,
        "app server must be reaped"
    );
    let child = std::fs::read_to_string(fixture.capture.with_extension("jsonl.child")).unwrap();
    // An adopted zombie may await the operating system's reaper; it cannot run.
    let output = std::process::Command::new("/bin/ps")
        .args(["-o", "stat=", "-p", child.trim()])
        .output()
        .unwrap();
    assert!(
        output.stdout.is_empty()
            || String::from_utf8_lossy(&output.stdout)
                .trim()
                .starts_with('Z'),
        "descendant must not remain running"
    );
    server.close().await.unwrap();
}

#[tokio::test]
async fn bound_reroute_and_compaction_invalidate_usage_without_changing_selection() {
    for scenario in ["usage", "reroute", "compaction", "wrong_reroute"] {
        let fixture = Fixture::new(scenario);
        let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
        let result = server.execute(request(), |_| async { Ok(()) }).await;
        if scenario == "wrong_reroute" {
            assert_eq!(result.unwrap_err(), RuntimeError::ContextMismatch);
        } else {
            let reply = result.unwrap().reply;
            assert_eq!(reply["effectiveSettings"]["model"], "model-a");
            if scenario == "usage" {
                assert_eq!(reply["usage"]["lastTotalTokens"], 40);
                assert_eq!(reply["usage"]["modelContextWindow"], 100);
                assert_eq!(reply["usage"]["freshness"], "fresh");
            } else {
                assert!(reply["usage"].is_null());
                if scenario == "reroute" {
                    assert_eq!(
                        reply["effectiveSettings"]["reroutedModel"],
                        "reported-runtime-model"
                    );
                }
            }
        }
        server.close().await.unwrap();
    }
}
