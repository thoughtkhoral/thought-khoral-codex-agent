// SPDX-License-Identifier: Apache-2.0
mod support;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use support::{Fixture, request};
use thought_khoral_codex_agent::{
    receipts::ReceiptStore,
    worker::{Worker, WorkerError},
};
fn acknowledgement(packet: &Value, result: &Value) -> Value {
    json!({"profileVersion":packet["profileVersion"],"taskId":packet["taskId"],"conversationId":packet["conversation"]["id"],"generation":packet["conversation"]["generation"],"replyEventId":"00000016-1111-4111-8111-000000000016","replySequence":6,"textDigest":format!("{:x}",Sha256::digest(result["assistantText"].as_str().unwrap().as_bytes())),"consumedRevision":result["consumedRevision"],"contextDigest":result["contextDigest"]})
}
fn worker_fixture(scenario: &str) -> Fixture {
    let mut fixture = Fixture::new(scenario);
    fixture.config.deadline = std::time::Duration::from_secs(10);
    fixture
}
#[tokio::test]
async fn completed_result_and_lost_ack_replay_without_a_second_provider_turn() {
    let f = worker_fixture("success");
    let db = f.directory.path().join("receipts.sqlite");
    let packet = request().packet;
    let worker = Worker::open(f.config.clone(), &db).await.unwrap();
    let result = worker.execute(packet.clone()).await.unwrap();
    drop(worker);
    let worker = Worker::open(f.config.clone(), &db).await.unwrap();
    assert_eq!(worker.execute(packet.clone()).await.unwrap(), result);
    let ack = acknowledgement(&packet, &result);
    worker
        .acknowledge(packet["taskId"].as_str().unwrap(), ack.clone())
        .await
        .unwrap();
    worker
        .acknowledge(packet["taskId"].as_str().unwrap(), ack.clone())
        .await
        .unwrap();
    let mut wrong = ack;
    wrong["replyEventId"] = json!("00000017-1111-4111-8111-000000000017");
    assert_eq!(
        worker
            .acknowledge(packet["taskId"].as_str().unwrap(), wrong)
            .await
            .unwrap_err(),
        WorkerError::DuplicateConflict
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
async fn uncertain_receipts_including_missing_turn_id_never_submit_on_restart() {
    for submitted in [false, true] {
        let f = worker_fixture("success");
        let db = f.directory.path().join("receipts.sqlite");
        let packet = request().packet;
        let store = ReceiptStore::open(&db).await.unwrap();
        store.reserve(&packet).await.unwrap();
        if submitted {
            store
                .submission_intent(packet["taskId"].as_str().unwrap(), "thread-exact")
                .await
                .unwrap();
        }
        drop(store);
        let worker = Worker::open(f.config.clone(), &db).await.unwrap();
        assert_eq!(
            worker.execute(packet.clone()).await.unwrap_err(),
            WorkerError::ConversationInterrupted
        );
        assert!(f.records().is_empty());
    }
}
#[tokio::test]
async fn changed_binding_or_expired_authority_cannot_replay_a_completed_result() {
    let f = worker_fixture("success");
    let db = f.directory.path().join("receipts.sqlite");
    let packet = request().packet;
    let worker = Worker::open(f.config.clone(), &db).await.unwrap();
    worker.execute(packet.clone()).await.unwrap();
    let mut conflict = packet.clone();
    conflict["model"] = json!("other-model");
    assert_eq!(
        worker.execute(conflict).await.unwrap_err(),
        WorkerError::DuplicateConflict
    );
    let mut expired = packet;
    expired["authorizationExpiresAt"] = json!(chrono::Utc::now() - chrono::Duration::seconds(1));
    assert!(worker.execute(expired).await.is_err());
    assert_eq!(
        f.records()
            .iter()
            .filter(|r| r["request"]["method"] == "turn/start")
            .count(),
        1
    );
}

fn continuation(packet: &Value, ack: &Value) -> Value {
    let mut next: Value = serde_json::from_str(include_str!(
        "../contracts/agent-conversation-v1/fixtures/valid/native-reply-substitution.json"
    ))
    .unwrap();
    next["taskId"] = json!("00000018-1111-4111-8111-000000000018");
    for field in [
        "issuedAt",
        "expiresAt",
        "authorizationExpiresAt",
        "leaseExpiresAt",
    ] {
        next[field] = packet[field].clone();
    }
    let binding = &mut next["context"]["nativeReplyBindings"][0];
    binding["sourceTaskId"] = packet["taskId"].clone();
    binding["eventId"] = ack["replyEventId"].clone();
    binding["textDigest"] = ack["textDigest"].clone();
    next["context"]["digest"] =
        json!(thought_khoral_codex_agent::protocol::context_digest(&next).unwrap());
    next
}
#[tokio::test]
async fn restart_continues_only_after_durable_ack_and_exact_native_binding() {
    let mut f = worker_fixture("success");
    f.config.arguments.push("--persistent".into());
    let db = f.directory.path().join("receipts.sqlite");
    let packet = request().packet;
    let worker = Worker::open(f.config.clone(), &db).await.unwrap();
    let result = worker.execute(packet.clone()).await.unwrap();
    let ack = acknowledgement(&packet, &result);
    let next = continuation(&packet, &ack);
    assert_eq!(
        worker.execute(next.clone()).await.unwrap_err(),
        WorkerError::ConversationBusy
    );
    worker
        .acknowledge(packet["taskId"].as_str().unwrap(), ack)
        .await
        .unwrap();
    drop(worker);
    let worker = Worker::open(f.config.clone(), &db).await.unwrap();
    worker.execute(next).await.unwrap();
    let records = f.records();
    let start = records
        .iter()
        .find(|r| r["request"]["method"] == "thread/start")
        .unwrap();
    let resume = records
        .iter()
        .find(|r| r["request"]["method"] == "thread/resume")
        .unwrap();
    assert_ne!(resume["request"]["params"]["threadId"], "");
    assert_eq!(
        start["request"]["params"]["model"],
        resume["request"]["params"]["model"]
    );
    assert_eq!(
        records
            .iter()
            .filter(|r| r["request"]["method"] == "turn/start")
            .count(),
        2
    );
}
#[tokio::test]
async fn missing_native_history_never_falls_back_to_a_new_thread() {
    let mut f = worker_fixture("success");
    f.config.arguments.push("--persistent".into());
    let db = f.directory.path().join("receipts.sqlite");
    let packet = request().packet;
    let worker = Worker::open(f.config.clone(), &db).await.unwrap();
    let result = worker.execute(packet.clone()).await.unwrap();
    let ack = acknowledgement(&packet, &result);
    worker
        .acknowledge(packet["taskId"].as_str().unwrap(), ack.clone())
        .await
        .unwrap();
    drop(worker);
    for file in std::fs::read_dir(f.config.native_home.join("sessions")).unwrap() {
        std::fs::remove_file(file.unwrap().path()).unwrap();
    }
    let worker = Worker::open(f.config.clone(), &db).await.unwrap();
    assert!(worker.execute(continuation(&packet, &ack)).await.is_err());
    assert_eq!(
        f.records()
            .iter()
            .filter(|r| r["request"]["method"] == "turn/start")
            .count(),
        1
    );
    assert_eq!(
        f.records()
            .iter()
            .filter(|r| r["request"]["method"] == "thread/start")
            .count(),
        1
    );
}
#[tokio::test]
async fn active_turn_is_durable_cancelled_and_not_reexecuted() {
    let mut f = worker_fixture("hang");
    f.config.deadline = std::time::Duration::from_secs(10);
    let db = f.directory.path().join("receipts.sqlite");
    let packet = request().packet;
    let task = packet["taskId"].as_str().unwrap().to_owned();
    let worker = Worker::open(f.config.clone(), &db).await.unwrap();
    let w = worker.clone();
    let p = packet.clone();
    let execution = tokio::spawn(async move { w.execute(p).await });
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            if f.records()
                .iter()
                .any(|r| r["request"]["method"] == "turn/start")
            {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(worker.receipt(&task).await.unwrap()["phase"], "running");
    assert_eq!(
        worker.execute(packet.clone()).await.unwrap_err(),
        WorkerError::ConversationBusy
    );
    worker.cancel(&task).await.unwrap();
    assert_eq!(
        execution.await.unwrap().unwrap_err(),
        WorkerError::ConversationInterrupted
    );
    assert_eq!(
        worker.execute(packet).await.unwrap_err(),
        WorkerError::ConversationInterrupted
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
async fn provider_denial_has_no_fallback_and_never_persists_success() {
    let f = worker_fixture("provider_denied");
    let db = f.directory.path().join("receipts.sqlite");
    let packet = request().packet;
    let task = packet["taskId"].as_str().unwrap();
    let worker = Worker::open(f.config.clone(), &db).await.unwrap();
    assert!(worker.execute(packet.clone()).await.is_err());
    assert!(worker.receipt(task).await.unwrap()["result"].is_null());
    assert!(worker.execute(packet.clone()).await.is_err());
    assert_eq!(
        f.records()
            .iter()
            .filter(|r| r["request"]["method"] == "turn/start")
            .count(),
        1
    );
}

#[tokio::test]
async fn four_global_slots_limit_different_rooms_and_shutdown_reaps_all_work() {
    let mut f = worker_fixture("hang");
    f.config.deadline = std::time::Duration::from_secs(10);
    let worker = Worker::open(
        f.config.clone(),
        &f.directory.path().join("receipts.sqlite"),
    )
    .await
    .unwrap();
    let mut executions = Vec::new();
    for index in 1..=4 {
        let mut p = request().packet;
        p["roomId"] = json!(format!("{index:08x}-1111-4111-8111-000000000001"));
        p["taskId"] = json!(format!("{index:08x}-1111-4111-8111-000000000002"));
        p["conversation"]["id"] = json!(format!("{index:08x}-1111-4111-8111-000000000003"));
        p["context"]["digest"] =
            json!(thought_khoral_codex_agent::protocol::context_digest(&p).unwrap());
        let w = worker.clone();
        executions.push(tokio::spawn(async move { w.execute(p).await }));
    }
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            if f.records()
                .iter()
                .filter(|r| r["request"]["method"] == "turn/start")
                .count()
                == 4
            {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        worker.execute(request().packet).await.unwrap_err(),
        WorkerError::ConversationBusy
    );
    worker.shutdown().await;
    for execution in executions {
        assert_eq!(
            execution.await.unwrap().unwrap_err(),
            WorkerError::ConversationInterrupted
        );
    }
    assert_eq!(
        f.records()
            .iter()
            .filter(|r| r["request"]["method"] == "turn/start")
            .count(),
        4
    );
}

#[tokio::test]
async fn mismatched_native_reply_and_policy_cannot_start_another_turn() {
    for case in ["binding", "missing-binding", "policy", "generation"] {
        let f = worker_fixture("success");
        let db = f.directory.path().join("receipts.sqlite");
        let packet = request().packet;
        let worker = Worker::open(f.config.clone(), &db).await.unwrap();
        let result = worker.execute(packet.clone()).await.unwrap();
        let ack = acknowledgement(&packet, &result);
        worker
            .acknowledge(packet["taskId"].as_str().unwrap(), ack.clone())
            .await
            .unwrap();
        let mut next = continuation(&packet, &ack);
        match case {
            "binding" => {
                next["context"]["nativeReplyBindings"][0]["textDigest"] = json!("0".repeat(64))
            }
            "missing-binding" => next["context"]["nativeReplyBindings"] = json!([]),
            "policy" => next["context"]["policyRevision"] = json!("revised-policy"),
            "generation" => next["conversation"]["generation"] = json!(2),
            _ => unreachable!(),
        }
        next["context"]["digest"] =
            json!(thought_khoral_codex_agent::protocol::context_digest(&next).unwrap());
        assert!(worker.execute(next).await.is_err(), "{case}");
        if case == "policy" || case == "missing-binding" {
            assert!(worker.execute(continuation(&packet, &ack)).await.is_err());
            assert!(worker.execute(packet.clone()).await.is_err());
        }
        assert_eq!(
            f.records()
                .iter()
                .filter(|r| r["request"]["method"] == "turn/start")
                .count(),
            1,
            "{case}"
        );
    }
}

#[tokio::test]
async fn provider_credential_is_only_in_child_environment_and_echo_cannot_succeed() {
    for scenario in ["success", "key_echo"] {
        let mut f = worker_fixture(scenario);
        let secret = f.directory.path().join("provider-key");
        std::fs::write(&secret, "synthetic-provider-key").unwrap();
        f.config.provider = Some(std::sync::Arc::new(
            thought_khoral_codex_agent::config::ProviderCredentials::from_file(&secret).unwrap(),
        ));
        let worker = Worker::open(
            f.config.clone(),
            &f.directory.path().join("receipts.sqlite"),
        )
        .await
        .unwrap();
        let packet = request().packet;
        let result = worker.execute(packet.clone()).await;
        if scenario == "key_echo" {
            assert_eq!(result.unwrap_err(), WorkerError::ExecutionFailed);
        } else {
            assert!(
                !result
                    .unwrap()
                    .to_string()
                    .contains("synthetic-provider-key")
            );
        }
        assert!(
            f.records()
                .iter()
                .all(|r| r["environment"]["hasProviderKey"] == true)
        );
        assert!(
            !serde_json::to_string(&f.records())
                .unwrap()
                .contains("synthetic-provider-key")
        );
        assert!(
            !worker
                .receipt(packet["taskId"].as_str().unwrap())
                .await
                .unwrap()
                .to_string()
                .contains("synthetic-provider-key")
        );
    }
}

#[tokio::test]
async fn replayed_reply_as_ordinary_entry_must_be_rejected() {
    let mut f = worker_fixture("success");
    f.config.arguments.push("--persistent".into());
    let worker = Worker::open(
        f.config.clone(),
        &f.directory.path().join("receipts.sqlite"),
    )
    .await
    .unwrap();
    let packet = request().packet;
    let result = worker.execute(packet.clone()).await.unwrap();
    let ack = acknowledgement(&packet, &result);
    worker
        .acknowledge(packet["taskId"].as_str().unwrap(), ack.clone())
        .await
        .unwrap();
    let mut next = continuation(&packet, &ack);
    next["context"]["nativeReplyBindings"] = json!([]);
    let entry = json!({"eventId":ack["replyEventId"],"sequence":ack["replySequence"],"authorId":packet["agentId"],"authorRole":"agent","occurredAt":"2026-10-05T10:00:00Z","text":result["assistantText"]});
    next["context"]["entries"]
        .as_array_mut()
        .unwrap()
        .insert(0, entry);
    next["context"]["digest"] =
        json!(thought_khoral_codex_agent::protocol::context_digest(&next).unwrap());
    let response = worker.execute(next).await;
    assert_eq!(response.unwrap_err(), WorkerError::ContextMismatch);
}

#[tokio::test]
async fn cancellation_before_completion_commit_must_prevent_success() {
    let f = worker_fixture("delayed_success");
    let db = f.directory.path().join("receipts.sqlite");
    let worker = Worker::open(f.config.clone(), &db).await.unwrap();
    let packet = request().packet;
    let task = packet["taskId"].as_str().unwrap().to_owned();
    let w = worker.clone();
    let exec = tokio::spawn(async move { w.execute(packet).await });
    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&db);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    use sqlx::Row;
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let r = sqlx::query("SELECT turn_id FROM receipts WHERE task_id=?")
                .bind(&task)
                .fetch_optional(&pool)
                .await
                .unwrap();
            if r.is_some_and(|r| r.get::<Option<String>, _>("turn_id").is_some()) {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    let mut conn = pool.acquire().await.unwrap();
    sqlx::query("BEGIN IMMEDIATE")
        .execute(&mut *conn)
        .await
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(800)).await;
    let w = worker.clone();
    let t = task.clone();
    let cancel = tokio::spawn(async move { w.cancel(&t).await });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    sqlx::query("ROLLBACK").execute(&mut *conn).await.unwrap();
    let cancelled = cancel.await.unwrap();
    let output = exec.await.unwrap();
    assert!(
        output.is_err(),
        "cancel returned {cancelled:?}; execution committed {output:?}"
    );
}
