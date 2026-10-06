// SPDX-License-Identifier: Apache-2.0
use crate::{
    protocol::{canonical_bytes, validate_profile},
    worker::WorkerError,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::{
    Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use std::{
    fs::{File, OpenOptions},
    path::Path,
    sync::Arc,
    time::Duration,
};
#[derive(Clone)]
pub struct ReceiptStore {
    pool: SqlitePool,
    _owner: Arc<File>,
}
fn get(row: &sqlx::sqlite::SqliteRow, name: &str) -> Result<String, WorkerError> {
    Ok(row.try_get(name)?)
}
pub fn fingerprint(packet: &Value) -> Result<String, WorkerError> {
    let mut stable = packet.clone();
    for field in [
        "issuedAt",
        "expiresAt",
        "authorizationExpiresAt",
        "leaseExpiresAt",
        "leaseOwner",
    ] {
        stable
            .as_object_mut()
            .ok_or(WorkerError::InvalidTaskInput)?
            .remove(field);
    }
    Ok(format!("{:x}", Sha256::digest(canonical_bytes(&stable)?)))
}
impl ReceiptStore {
    pub async fn open(path: &Path) -> Result<Self, WorkerError> {
        let owner = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path.with_extension("owner"))?;
        #[cfg(unix)]
        {
            use std::os::fd::AsRawFd;
            if unsafe { libc::flock(owner.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
                return Err(WorkerError::ConversationBusy);
            }
        }
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Full)
            .busy_timeout(Duration::from_secs(5));
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;
        sqlx::raw_sql(include_str!("../migrations/0001_worker_receipts.sql"))
            .execute(&pool)
            .await?;
        Ok(Self {
            pool,
            _owner: Arc::new(owner),
        })
    }
    pub async fn recover(&self) -> Result<(), WorkerError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            "UPDATE conversations SET state='unusable' WHERE state IN ('reserved','running')",
        )
        .execute(&mut *tx)
        .await?;
        // Existing ready history is preserved only for a definitely pre-submit failure.
        sqlx::query("UPDATE conversations SET state='ready' WHERE state='unusable' AND EXISTS(SELECT 1 FROM receipts r WHERE r.conversation_id=conversations.conversation_id AND r.generation=conversations.generation AND r.phase='reserved' AND r.prior_ready=1)").execute(&mut *tx).await?;
        sqlx::query("UPDATE receipts SET phase='interrupted',error_code='conversation_interrupted' WHERE phase IN ('reserved','running')").execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
    pub async fn existing(&self, packet: &Value) -> Result<Option<Value>, WorkerError> {
        let Some(row) = sqlx::query("SELECT * FROM receipts WHERE task_id=?")
            .bind(packet["taskId"].as_str().unwrap())
            .fetch_optional(&self.pool)
            .await?
        else {
            return Ok(None);
        };
        if get(&row, "fingerprint")? != fingerprint(packet)? {
            return Err(WorkerError::DuplicateConflict);
        }
        self.current(packet).await?;
        match get(&row, "phase")?.as_str() {
            "completed" => Ok(Some(
                serde_json::from_str(&get(&row, "result")?)
                    .map_err(|_| WorkerError::RuntimeUnavailable)?,
            )),
            "reserved" | "running" => Err(WorkerError::ConversationBusy),
            _ => Err(WorkerError::from_code(&get(&row, "error_code")?)),
        }
    }
    async fn current(&self, p: &Value) -> Result<(), WorkerError> {
        let row = sqlx::query("SELECT * FROM conversations WHERE room_id=? AND agent_id=?")
            .bind(p["roomId"].as_str().unwrap())
            .bind(p["agentId"].as_str().unwrap())
            .fetch_optional(&self.pool)
            .await?
            .ok_or(WorkerError::SessionUnavailable)?;
        if row.try_get::<bool, _>("invalidated")?
            || get(&row, "conversation_id")? != p["conversation"]["id"]
            || row.try_get::<i64, _>("generation")?
                != p["conversation"]["generation"].as_i64().unwrap()
            || get(&row, "policy_revision")? != p["context"]["policyRevision"]
            || get(&row, "guidance_revision")? != p["guidanceRevision"]
        {
            return Err(WorkerError::ConversationStale);
        }
        Ok(())
    }
    pub async fn reserve(&self, p: &Value) -> Result<Option<String>, WorkerError> {
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query("SELECT * FROM conversations WHERE room_id=? AND agent_id=?")
            .bind(p["roomId"].as_str().unwrap())
            .bind(p["agentId"].as_str().unwrap())
            .fetch_optional(&mut *tx)
            .await?;
        let new = p["conversation"]["mode"] == "new";
        let mut thread = None;
        let mut prior_ready = false;
        if let Some(row) = &row {
            if matches!(
                get(row, "state")?.as_str(),
                "reserved" | "running" | "pending_ack"
            ) {
                return Err(WorkerError::ConversationBusy);
            }
            if new {
                if get(row, "conversation_id")? == p["conversation"]["id"]
                    && row.try_get::<i64, _>("generation")?
                        >= p["conversation"]["generation"].as_i64().unwrap()
                {
                    return Err(WorkerError::ConversationStale);
                }
            } else {
                if get(row, "conversation_id")? == p["conversation"]["id"]
                    && row.try_get::<i64, _>("generation")?
                        == p["conversation"]["generation"].as_i64().unwrap()
                    && (get(row, "policy_revision")? != p["context"]["policyRevision"]
                        || get(row, "guidance_revision")? != p["guidanceRevision"])
                {
                    sqlx::query("UPDATE conversations SET state='unusable',invalidated=1,policy_revision=?,guidance_revision=? WHERE room_id=? AND agent_id=?")
                        .bind(p["context"]["policyRevision"].as_str().unwrap()).bind(p["guidanceRevision"].as_str().unwrap())
                        .bind(p["roomId"].as_str().unwrap()).bind(p["agentId"].as_str().unwrap()).execute(&mut *tx).await?;
                    tx.commit().await?;
                    return Err(WorkerError::ConversationStale);
                }
                if get(row, "conversation_id")? != p["conversation"]["id"]
                    || row.try_get::<i64, _>("generation")?
                        != p["conversation"]["generation"].as_i64().unwrap()
                    || get(row, "policy_revision")? != p["context"]["policyRevision"]
                    || get(row, "guidance_revision")? != p["guidanceRevision"]
                {
                    return Err(WorkerError::ConversationStale);
                }
                if get(row, "state")? != "ready" {
                    return Err(WorkerError::SessionUnavailable);
                }
                if row.try_get::<i64, _>("consumed_revision")?
                    != p["context"]["baseRevision"].as_i64().unwrap()
                {
                    return Err(WorkerError::ContextMismatch);
                }
                thread = row.try_get::<Option<String>, _>("thread_id")?;
                if thread.is_none() {
                    return Err(WorkerError::SessionUnavailable);
                }
                prior_ready = true;
            }
        } else if !new {
            return Err(WorkerError::SessionUnavailable);
        }
        // Every substitution must be an acknowledged reply in this exact native thread.
        let mut prior = p["context"]["baseRevision"].as_u64().unwrap();
        for binding in p["context"]["nativeReplyBindings"].as_array().unwrap() {
            let receipt = sqlx::query("SELECT * FROM receipts WHERE task_id=?")
                .bind(binding["sourceTaskId"].as_str().unwrap())
                .fetch_optional(&mut *tx)
                .await?
                .ok_or(WorkerError::ContextMismatch)?;
            let ack: Value = serde_json::from_str(
                &get(&receipt, "ack").map_err(|_| WorkerError::ContextMismatch)?,
            )
            .map_err(|_| WorkerError::ContextMismatch)?;
            if get(&receipt, "phase")? != "completed"
                || get(&receipt, "conversation_id")? != p["conversation"]["id"]
                || receipt.try_get::<i64, _>("generation")?
                    != p["conversation"]["generation"].as_i64().unwrap()
                || receipt.try_get::<Option<String>, _>("thread_id")? != thread
                || ack["replyEventId"] != binding["eventId"]
                || ack["replySequence"] != binding["sequence"]
                || ack["textDigest"] != binding["textDigest"]
                || binding["sequence"].as_u64().unwrap() <= prior
            {
                return Err(WorkerError::ContextMismatch);
            }
            prior = binding["sequence"].as_u64().unwrap();
        }
        let ids: Vec<Value> = p["context"]["entries"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e["eventId"].clone())
            .collect();
        if !new {
            let old_receipts = sqlx::query("SELECT source_ids,ack,thread_id FROM receipts WHERE conversation_id=? AND generation=? AND phase='completed'")
                .bind(p["conversation"]["id"].as_str().unwrap())
                .bind(p["conversation"]["generation"].as_i64().unwrap())
                .fetch_all(&mut *tx).await?;
            for old in old_receipts {
                if old.try_get::<Option<String>, _>("thread_id")? == thread {
                    if let Some(ack) = old.try_get::<Option<String>, _>("ack")? {
                        let ack: Value = serde_json::from_str(&ack)
                            .map_err(|_| WorkerError::RuntimeUnavailable)?;
                        if ids.contains(&ack["replyEventId"]) {
                            return Err(WorkerError::ContextMismatch);
                        }
                        let sequence = ack["replySequence"]
                            .as_u64()
                            .ok_or(WorkerError::RuntimeUnavailable)?;
                        if sequence > p["context"]["baseRevision"].as_u64().unwrap()
                            && sequence <= p["context"]["revision"].as_u64().unwrap()
                            && !p["context"]["nativeReplyBindings"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .any(|binding| binding["eventId"] == ack["replyEventId"])
                        {
                            sqlx::query("UPDATE conversations SET state='unusable',invalidated=1 WHERE room_id=? AND agent_id=?")
                                .bind(p["roomId"].as_str().unwrap()).bind(p["agentId"].as_str().unwrap()).execute(&mut *tx).await?;
                            tx.commit().await?;
                            return Err(WorkerError::ContextMismatch);
                        }
                    }
                }
                let old: Vec<Value> = serde_json::from_str(&get(&old, "source_ids")?)
                    .map_err(|_| WorkerError::RuntimeUnavailable)?;
                if ids.iter().any(|id| old.contains(id)) {
                    return Err(WorkerError::ContextMismatch);
                }
            }
        }
        if new {
            sqlx::query("INSERT INTO conversations(room_id,agent_id,conversation_id,generation,policy_revision,guidance_revision,state) VALUES(?,?,?,?,?,?,'reserved') ON CONFLICT(room_id,agent_id) DO UPDATE SET conversation_id=excluded.conversation_id,generation=excluded.generation,policy_revision=excluded.policy_revision,guidance_revision=excluded.guidance_revision,state='reserved',invalidated=0,thread_id=NULL,consumed_revision=0").bind(p["roomId"].as_str().unwrap()).bind(p["agentId"].as_str().unwrap()).bind(p["conversation"]["id"].as_str().unwrap()).bind(p["conversation"]["generation"].as_i64().unwrap()).bind(p["context"]["policyRevision"].as_str().unwrap()).bind(p["guidanceRevision"].as_str().unwrap()).execute(&mut *tx).await?;
        } else {
            sqlx::query("UPDATE conversations SET state='reserved' WHERE conversation_id=?")
                .bind(p["conversation"]["id"].as_str().unwrap())
                .execute(&mut *tx)
                .await?;
        }
        sqlx::query("INSERT INTO receipts(task_id,room_id,agent_id,conversation_id,generation,fingerprint,context_digest,base_revision,end_revision,policy_revision,guidance_revision,source_ids,phase,prior_ready,thread_id) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,'reserved',?,?)").bind(p["taskId"].as_str().unwrap()).bind(p["roomId"].as_str().unwrap()).bind(p["agentId"].as_str().unwrap()).bind(p["conversation"]["id"].as_str().unwrap()).bind(p["conversation"]["generation"].as_i64().unwrap()).bind(fingerprint(p)?).bind(p["context"]["digest"].as_str().unwrap()).bind(p["context"]["baseRevision"].as_i64().unwrap()).bind(p["context"]["revision"].as_i64().unwrap()).bind(p["context"]["policyRevision"].as_str().unwrap()).bind(p["guidanceRevision"].as_str().unwrap()).bind(serde_json::to_string(&ids).unwrap()).bind(prior_ready).bind(&thread).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(thread)
    }
    pub async fn submission_intent(&self, task: &str, thread: &str) -> Result<(), WorkerError> {
        let mut tx = self.pool.begin().await?;
        let changed = sqlx::query(
            "UPDATE receipts SET phase='running',thread_id=? WHERE task_id=? AND phase='reserved'",
        )
        .bind(thread)
        .bind(task)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        if changed != 1 {
            return Err(WorkerError::ConversationInterrupted);
        }
        sqlx::query("UPDATE conversations SET state='running',thread_id=? WHERE conversation_id=(SELECT conversation_id FROM receipts WHERE task_id=?)").bind(thread).bind(task).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
    pub async fn bind_turn(&self, task: &str, thread: &str, turn: &str) -> Result<(), WorkerError> {
        if sqlx::query("UPDATE receipts SET turn_id=? WHERE task_id=? AND phase='running' AND thread_id=? AND turn_id IS NULL").bind(turn).bind(task).bind(thread).execute(&self.pool).await?.rows_affected()!=1{return Err(WorkerError::ConversationInterrupted)}
        Ok(())
    }
    pub async fn complete(
        &self,
        p: &Value,
        result: &Value,
        cancellation: &tokio_util::sync::CancellationToken,
        terminal: &tokio::sync::Mutex<()>,
        guidance: &str,
    ) -> Result<(), WorkerError> {
        validate_profile("result", result)?;
        self.current(p).await?;
        let mut tx = self.pool.begin().await?;
        if sqlx::query("UPDATE receipts SET phase='completed',result=? WHERE task_id=? AND phase='running' AND turn_id IS NOT NULL").bind(serde_json::to_string(result).unwrap()).bind(p["taskId"].as_str().unwrap()).execute(&mut *tx).await?.rows_affected()!=1{return Err(WorkerError::ConversationInterrupted)}
        sqlx::query("UPDATE conversations SET state='pending_ack' WHERE conversation_id=?")
            .bind(p["conversation"]["id"].as_str().unwrap())
            .execute(&mut *tx)
            .await?;
        // Cancellation remains admissible while SQLite is blocked. Once this gate
        // is held, the final commit wins concurrent cancellation; failures remain uncertain.
        let _terminal = terminal.lock().await;
        if cancellation.is_cancelled() {
            return Err(WorkerError::ConversationInterrupted);
        }
        crate::protocol::validate_request(
            &crate::protocol::RuntimeRequest {
                packet: p.clone(),
                thread_id: (p["conversation"]["mode"] == "continue")
                    .then(|| "validated-mapping-required".into()),
            },
            guidance,
        )?;
        tx.commit().await?;
        Ok(())
    }
    pub async fn fail(&self, task: &str, error: WorkerError) -> Result<(), WorkerError> {
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query("SELECT * FROM receipts WHERE task_id=?")
            .bind(task)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(WorkerError::InvalidTaskInput)?;
        let phase = get(&row, "phase")?;
        if matches!(phase.as_str(), "completed" | "failed" | "interrupted") {
            return Ok(());
        }
        let safe = phase == "reserved"
            && row.try_get::<bool, _>("prior_ready")?
            && error != WorkerError::SessionUnavailable;
        sqlx::query("UPDATE receipts SET phase=?,error_code=? WHERE task_id=?")
            .bind(if phase == "running" {
                "interrupted"
            } else {
                "failed"
            })
            .bind(error.to_string())
            .bind(task)
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE conversations SET state=? WHERE conversation_id=? AND generation=?")
            .bind(if safe { "ready" } else { "unusable" })
            .bind(get(&row, "conversation_id")?)
            .bind(row.try_get::<i64, _>("generation")?)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
    pub async fn receipt(&self, task: &str) -> Result<Value, WorkerError> {
        let r = sqlx::query("SELECT * FROM receipts WHERE task_id=?")
            .bind(task)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(WorkerError::InvalidTaskInput)?;
        let decode = |name| -> Result<Option<Value>, WorkerError> {
            r.try_get::<Option<String>, _>(name)?
                .map(|v| serde_json::from_str(&v).map_err(|_| WorkerError::RuntimeUnavailable))
                .transpose()
        };
        let phase = get(&r, "phase")?;
        let runtime_binding = if phase == "completed" {
            let thread = get(&r, "thread_id")?;
            let turn = get(&r, "turn_id")?;
            if thread.is_empty() || turn.is_empty() {
                return Err(WorkerError::RuntimeUnavailable);
            }
            json!({"threadId":thread,"turnId":turn})
        } else {
            Value::Null
        };
        Ok(
            json!({"runtimeBinding":runtime_binding,"taskId":task,"conversationId":get(&r,"conversation_id")?,"generation":r.try_get::<i64,_>("generation")?,"phase":get(&r,"phase")?,"result":decode("result")?,"acknowledgement":decode("ack")?,"error":r.try_get::<Option<String>,_>("error_code")?}),
        )
    }
    pub async fn acknowledge(&self, task: &str, ack: &Value) -> Result<(), WorkerError> {
        validate_profile("ack", ack)?;
        if ack["taskId"] != task {
            return Err(WorkerError::ContextMismatch);
        }
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query("SELECT * FROM receipts WHERE task_id=?")
            .bind(task)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(WorkerError::InvalidTaskInput)?;
        let result: Value = serde_json::from_str(
            &get(&row, "result").map_err(|_| WorkerError::ConversationInterrupted)?,
        )
        .map_err(|_| WorkerError::RuntimeUnavailable)?;
        if get(&row, "phase")? != "completed"
            || ack["conversationId"] != get(&row, "conversation_id")?
            || ack["generation"] != row.try_get::<i64, _>("generation")?
            || ack["consumedRevision"] != row.try_get::<i64, _>("end_revision")?
            || ack["contextDigest"] != get(&row, "context_digest")?
            || ack["textDigest"]
                != format!(
                    "{:x}",
                    Sha256::digest(result["assistantText"].as_str().unwrap().as_bytes())
                )
            || ack["replySequence"].as_u64().unwrap() <= ack["consumedRevision"].as_u64().unwrap()
        {
            return Err(WorkerError::ContextMismatch);
        }
        if let Some(old) = row.try_get::<Option<String>, _>("ack")? {
            if serde_json::from_str::<Value>(&old).map_err(|_| WorkerError::RuntimeUnavailable)?
                != *ack
            {
                return Err(WorkerError::DuplicateConflict);
            }
            return Ok(());
        }
        sqlx::query("UPDATE receipts SET ack=? WHERE task_id=?")
            .bind(serde_json::to_string(ack).unwrap())
            .bind(task)
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE conversations SET state='ready',consumed_revision=? WHERE conversation_id=? AND generation=? AND state='pending_ack'").bind(ack["consumedRevision"].as_i64().unwrap()).bind(get(&row,"conversation_id")?).bind(row.try_get::<i64,_>("generation")?).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
}
