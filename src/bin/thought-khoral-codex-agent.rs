// SPDX-License-Identifier: Apache-2.0
use std::{path::Path, sync::Arc};
use thought_khoral_codex_agent::{
    a2a_service::{ServiceConfig, service},
    config::{Config, ProviderCredentials},
    worker::{Worker, WorkerError},
};
fn env(name: &str) -> Result<String, WorkerError> {
    std::env::var(name).map_err(|_| WorkerError::InvalidTaskInput)
}
fn private_directory(path: &Path) -> Result<(), WorkerError> {
    use std::os::unix::fs::MetadataExt;
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.is_dir()
        || metadata.uid() != 10003
        || metadata.gid() != 10003
        || metadata.mode() & 0o777 != 0o700
    {
        return Err(WorkerError::Forbidden);
    }
    Ok(())
}
async fn run() -> Result<(), WorkerError> {
    // No provider access is needed to verify the package. Deployment must first meet Task 8's isolation gate.
    if std::env::args().nth(1).as_deref() == Some("--verify-package") {
        let output = tokio::process::Command::new("/usr/local/bin/codex")
            .env_clear()
            .env("HOME", "/var/lib/thought-khoral-codex/native")
            .env("CODEX_HOME", "/var/lib/thought-khoral-codex/native")
            .arg("--version")
            .output()
            .await?;
        if !output.status.success() || output.stdout != b"codex-cli 0.160.0\n" {
            return Err(WorkerError::RuntimeUnavailable);
        }
        if unsafe { libc::geteuid() } != 10003 || unsafe { libc::getegid() } != 10003 {
            return Err(WorkerError::Forbidden);
        }
        thought_khoral_codex_agent::tool_policy::verify_installed_package()?;
        println!("tool policy verified; exposed tools: []");
        println!("worker package verified; codex-cli 0.160.0; uid/gid 10003; no inference");
        return Ok(());
    }
    if std::env::args().len() != 1 || env("THOUGHT_KHORAL_CODEX_ISOLATION_VERIFIED")? != "1" {
        return Err(WorkerError::Forbidden);
    }
    if unsafe { libc::geteuid() } != 10003 || unsafe { libc::getegid() } != 10003 {
        return Err(WorkerError::Forbidden);
    }
    thought_khoral_codex_agent::tool_policy::verify_installed_package()?;
    unsafe {
        libc::umask(0o077);
    }
    let native = Path::new("/var/lib/thought-khoral-codex/native");
    let receipts = Path::new("/var/lib/thought-khoral-codex/receipts");
    private_directory(native)?;
    private_directory(receipts)?;
    if native.join("config.toml").exists() || native.join("auth.json").exists() {
        return Err(WorkerError::Forbidden);
    }
    let cwd = std::fs::symlink_metadata("/opt/thought-khoral-codex/workspace")?;
    use std::os::unix::fs::MetadataExt;
    if !cwd.is_dir() || cwd.uid() != 0 || cwd.mode() & 0o777 != 0o555 {
        return Err(WorkerError::Forbidden);
    }
    let mut config = Config::new(
        "/usr/local/bin/codex".into(),
        "/opt/thought-khoral-codex/workspace".into(),
        native.into(),
    );
    config.model_allowlist = env("THOUGHT_KHORAL_CODEX_MODELS")?
        .split(',')
        .map(str::to_owned)
        .collect();
    let provider =
        ProviderCredentials::from_file(Path::new(&env("THOUGHT_KHORAL_CODEX_PROVIDER_KEY_FILE")?))?;
    // Reuse bounded credential-file parsing but never pass invocation credentials to the subprocess.
    let invocation_path = env("THOUGHT_KHORAL_CODEX_INVOCATION_KEY_FILE")?;
    let invocation =
        thought_khoral_codex_agent::config::read_credential_file(Path::new(&invocation_path))?;
    if provider.matches(&invocation) {
        return Err(WorkerError::InvalidTaskInput);
    }
    config.provider = Some(Arc::new(provider));
    let expires =
        chrono::DateTime::parse_from_rfc3339(&env("THOUGHT_KHORAL_CODEX_ADMISSION_EXPIRES_AT")?)
            .map_err(|_| WorkerError::InvalidTaskInput)?
            .with_timezone(&chrono::Utc);
    if expires <= chrono::Utc::now() {
        return Err(WorkerError::AuthenticationRequired);
    }
    let worker = Worker::open(config, &receipts.join("worker.sqlite")).await?;
    let router = service(worker.clone(), ServiceConfig::new(invocation, expires))?;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9091").await?;
    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            let mut term =
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .expect("signal setup failed");
            tokio::select! {_=tokio::signal::ctrl_c()=>{},_=term.recv()=>{}}
            worker.shutdown().await;
        })
        .await?;
    Ok(())
}
#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
