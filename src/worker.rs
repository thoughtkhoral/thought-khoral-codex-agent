// SPDX-License-Identifier: Apache-2.0
use crate::{
    app_server::AppServer,
    config::Config,
    protocol::{RuntimeError, RuntimeRequest, validate_request},
    receipts::ReceiptStore,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex, Weak},
};
use tokio::sync::{Mutex as AsyncMutex, Semaphore};
use tokio_util::sync::CancellationToken;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerError {
    InvalidTaskInput,
    Forbidden,
    ConversationBusy,
    ConversationStale,
    ContextMismatch,
    ContextTooLarge,
    RuntimeUnavailable,
    AuthenticationRequired,
    SessionUnavailable,
    Timeout,
    ConversationInterrupted,
    ExecutionFailed,
    DuplicateConflict,
}
impl WorkerError {
    pub fn from_code(code: &str) -> Self {
        match code {
            "invalid_task_input" => Self::InvalidTaskInput,
            "forbidden" => Self::Forbidden,
            "conversation_busy" => Self::ConversationBusy,
            "conversation_stale" => Self::ConversationStale,
            "context_mismatch" => Self::ContextMismatch,
            "context_too_large" => Self::ContextTooLarge,
            "authentication_required" => Self::AuthenticationRequired,
            "session_unavailable" => Self::SessionUnavailable,
            "timeout" => Self::Timeout,
            "conversation_interrupted" => Self::ConversationInterrupted,
            "execution_failed" => Self::ExecutionFailed,
            "duplicate_conflict" => Self::DuplicateConflict,
            _ => Self::RuntimeUnavailable,
        }
    }
    pub fn runtime(self) -> RuntimeError {
        match self {
            Self::ContextMismatch => RuntimeError::ContextMismatch,
            Self::ConversationInterrupted => RuntimeError::ConversationInterrupted,
            _ => RuntimeError::RuntimeUnavailable,
        }
    }
}
impl std::fmt::Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidTaskInput => "invalid_task_input",
            Self::Forbidden => "forbidden",
            Self::ConversationBusy => "conversation_busy",
            Self::ConversationStale => "conversation_stale",
            Self::ContextMismatch => "context_mismatch",
            Self::ContextTooLarge => "context_too_large",
            Self::RuntimeUnavailable => "runtime_unavailable",
            Self::AuthenticationRequired => "authentication_required",
            Self::SessionUnavailable => "session_unavailable",
            Self::Timeout => "timeout",
            Self::ConversationInterrupted => "conversation_interrupted",
            Self::ExecutionFailed => "execution_failed",
            Self::DuplicateConflict => "duplicate_conflict",
        })
    }
}
impl std::error::Error for WorkerError {}
impl From<RuntimeError> for WorkerError {
    fn from(e: RuntimeError) -> Self {
        Self::from_code(&e.to_string())
    }
}
impl From<sqlx::Error> for WorkerError {
    fn from(_: sqlx::Error) -> Self {
        Self::RuntimeUnavailable
    }
}
impl From<std::io::Error> for WorkerError {
    fn from(_: std::io::Error) -> Self {
        Self::RuntimeUnavailable
    }
}
#[derive(Clone)]
pub struct Worker {
    inner: Arc<Inner>,
}
struct Inner {
    config: Config,
    store: ReceiptStore,
    slots: Arc<Semaphore>,
    locks: Mutex<HashMap<String, Weak<AsyncMutex<()>>>>,
    active: Mutex<HashMap<String, Active>>,
    shutdown: CancellationToken,
}
#[derive(Clone)]
struct Active {
    token: CancellationToken,
    terminal: Arc<AsyncMutex<()>>,
}
struct ActiveGuard {
    worker: Worker,
    task: String,
}
impl Drop for ActiveGuard {
    fn drop(&mut self) {
        self.worker.inner.active.lock().unwrap().remove(&self.task);
    }
}
impl Worker {
    pub async fn open(config: Config, path: &Path) -> Result<Self, WorkerError> {
        config.validate()?;
        let store = ReceiptStore::open(path).await?;
        store.recover().await?;
        Ok(Self {
            inner: Arc::new(Inner {
                config,
                store,
                slots: Arc::new(Semaphore::new(4)),
                locks: Mutex::new(HashMap::new()),
                active: Mutex::new(HashMap::new()),
                shutdown: CancellationToken::new(),
            }),
        })
    }
    fn validate(&self, p: &Value, thread: Option<String>) -> Result<(), WorkerError> {
        validate_request(
            &RuntimeRequest {
                packet: p.clone(),
                thread_id: thread,
            },
            &self.inner.config.guidance_revision,
        )?;
        Ok(())
    }
    pub async fn execute(&self, packet: Value) -> Result<Value, WorkerError> {
        let continuation = packet["conversation"]["mode"] == "continue";
        self.validate(
            &packet,
            continuation.then(|| "validated-mapping-required".into()),
        )?;
        if self.inner.shutdown.is_cancelled() {
            return Err(WorkerError::RuntimeUnavailable);
        }
        let scope = format!(
            "{}:{}",
            packet["roomId"].as_str().unwrap(),
            packet["agentId"].as_str().unwrap()
        );
        let lock = {
            let mut locks = self.inner.locks.lock().unwrap();
            locks.retain(|_, lock| lock.strong_count() > 0);
            let lock = locks
                .get(&scope)
                .and_then(Weak::upgrade)
                .unwrap_or_else(|| Arc::new(AsyncMutex::new(())));
            locks.insert(scope, Arc::downgrade(&lock));
            lock
        };
        let guard = lock
            .try_lock_owned()
            .map_err(|_| WorkerError::ConversationBusy)?;
        if let Some(result) = self.inner.store.existing(&packet).await? {
            self.validate(
                &packet,
                continuation.then(|| "validated-mapping-required".into()),
            )?;
            return Ok(result);
        }
        let slot = self
            .inner
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| WorkerError::ConversationBusy)?;
        let thread = self.inner.store.reserve(&packet).await?;
        let task = packet["taskId"].as_str().unwrap().to_owned();
        let token = self.inner.shutdown.child_token();
        let terminal = Arc::new(AsyncMutex::new(()));
        self.inner.active.lock().unwrap().insert(
            task.clone(),
            Active {
                token: token.clone(),
                terminal: terminal.clone(),
            },
        );
        // Owned execution survives a dropped HTTP waiter; explicit cancellation and authority deadlines still stop it.
        let worker = self.clone();
        let handle = tokio::spawn(async move {
            let _guard = guard;
            let _slot = slot;
            let _active = ActiveGuard {
                worker: worker.clone(),
                task: task.clone(),
            };
            let result = worker.run(packet, thread, token, terminal).await;
            match result {
                Err(error) => worker.inner.store.fail(&task, error).await.and(Err(error)),
                Ok(value) => Ok(value),
            }
        });
        handle.await.map_err(|_| WorkerError::RuntimeUnavailable)?
    }
    async fn run(
        &self,
        packet: Value,
        thread_id: Option<String>,
        token: CancellationToken,
        terminal: Arc<AsyncMutex<()>>,
    ) -> Result<Value, WorkerError> {
        let mut server = AppServer::spawn(self.inner.config.clone()).await?;
        let task = packet["taskId"].as_str().unwrap().to_owned();
        let store = self.inner.store.clone();
        let turn_store = store.clone();
        let turn_task = task.clone();
        let result = tokio::select! {
         biased;
         _=token.cancelled()=>Err(WorkerError::ConversationInterrupted),
         result=server.execute_with_callbacks(RuntimeRequest{packet:packet.clone(),thread_id:thread_id.clone()},move |thread|async move {store.submission_intent(&task,&thread).await.map_err(WorkerError::runtime)},move |thread,turn|async move{turn_store.bind_turn(&turn_task,&thread,&turn).await.map_err(WorkerError::runtime)})=>result.map_err(WorkerError::from),
        };
        server.close().await?;
        let outcome = result?;
        if token.is_cancelled() {
            return Err(WorkerError::ConversationInterrupted);
        }
        self.validate(&packet, thread_id)?;
        if self
            .inner
            .config
            .provider
            .as_ref()
            .is_some_and(|provider| provider.appears_in(&outcome.reply))
        {
            return Err(WorkerError::ExecutionFailed);
        }
        self.inner
            .store
            .complete(
                &packet,
                &outcome.reply,
                &token,
                &terminal,
                &self.inner.config.guidance_revision,
            )
            .await?;
        Ok(outcome.reply)
    }
    pub async fn acknowledge(&self, task: &str, ack: Value) -> Result<(), WorkerError> {
        self.inner.store.acknowledge(task, &ack).await
    }
    pub async fn receipt(&self, task: &str) -> Result<Value, WorkerError> {
        self.inner.store.receipt(task).await
    }
    pub async fn cancel(&self, task: &str) -> Result<Value, WorkerError> {
        let active = self.inner.active.lock().unwrap().get(task).cloned();
        if let Some(active) = active {
            let _terminal = active.terminal.lock().await;
            active.token.cancel();
        }
        for _ in 0..100 {
            if !self.inner.active.lock().unwrap().contains_key(task) {
                return self.receipt(task).await;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
        Err(WorkerError::RuntimeUnavailable)
    }
    pub async fn shutdown(&self) {
        self.inner.shutdown.cancel();
        loop {
            if self.inner.active.lock().unwrap().is_empty()
                && self.inner.slots.available_permits() == 4
            {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    }
    pub async fn catalog(&self) -> Result<Value, WorkerError> {
        let _slot = self
            .inner
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| WorkerError::ConversationBusy)?;
        let mut server = AppServer::spawn(self.inner.config.clone()).await?;
        let result = tokio::select! {
            _ = self.inner.shutdown.cancelled() => Err(RuntimeError::ConversationInterrupted),
            result = server.catalog() => result,
        };
        server.close().await?;
        let result = result?;
        if self
            .inner
            .config
            .provider
            .as_ref()
            .is_some_and(|provider| provider.appears_in(&result))
        {
            return Err(WorkerError::ExecutionFailed);
        }
        Ok(result)
    }
}
