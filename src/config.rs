// SPDX-License-Identifier: Apache-2.0
use std::{ffi::OsString, path::PathBuf, time::Duration};
#[derive(Clone)]
pub struct Config {
    pub provider: Option<std::sync::Arc<ProviderCredentials>>,
    pub executable: PathBuf,
    pub arguments: Vec<OsString>,
    pub working_directory: PathBuf,
    pub native_home: PathBuf,
    pub tool_catalog: PathBuf,
    pub deadline: Duration,
    pub interrupt_grace: Duration,
    pub max_line_bytes: usize,
    pub max_output_bytes: usize,
    pub catalog_revision: String,
    pub model_allowlist: Vec<String>,
    pub guidance_revision: String,
}
impl Config {
    pub fn new(executable: PathBuf, working_directory: PathBuf, native_home: PathBuf) -> Self {
        Self {
            provider: None,
            executable,
            arguments: vec![],
            working_directory,
            native_home,
            tool_catalog: crate::tool_policy::CATALOG_PATH.into(),
            deadline: Duration::from_secs(180),
            interrupt_grace: Duration::from_secs(5),
            max_line_bytes: 1_048_576,
            max_output_bytes: 4_194_304,
            catalog_revision: "catalog-1".into(),
            model_allowlist: vec![],
            guidance_revision: "fixed-1".into(),
        }
    }
}
impl Config {
    pub(crate) fn validate(&self) -> Result<(), crate::protocol::RuntimeError> {
        use crate::protocol::RuntimeError;
        if self.deadline.is_zero()
            || self.deadline > Duration::from_secs(180)
            || self.interrupt_grace.is_zero()
            || self.interrupt_grace > Duration::from_secs(5)
            || self.max_line_bytes == 0
            || self.max_line_bytes > 1_048_576
            || self.max_output_bytes < self.max_line_bytes
            || self.max_output_bytes > 4_194_304
            || self.catalog_revision.is_empty()
            || self.catalog_revision.chars().count() > 128
            || self.guidance_revision.is_empty()
            || self.guidance_revision.chars().count() > 128
            || self.model_allowlist.is_empty()
            || self.model_allowlist.len() > 100
        {
            return Err(RuntimeError::InvalidTaskInput);
        }
        crate::tool_policy::verify_catalog(&self.tool_catalog)?;
        let cwd = std::fs::canonicalize(&self.working_directory)?;
        let home = std::fs::canonicalize(&self.native_home)?;
        if !cwd.is_dir() || !home.is_dir() || cwd.starts_with(&home) || home.starts_with(&cwd) {
            return Err(RuntimeError::InvalidTaskInput);
        }
        Ok(())
    }
}
/// Explicit pinned controls; live verification remains a deployment gate.
pub(crate) fn tool_overrides() -> serde_json::Value {
    serde_json::from_str(crate::tool_policy::CONTROLS).expect("pinned tool controls")
}

pub struct ProviderCredentials {
    key: String,
}
impl ProviderCredentials {
    pub fn from_file(path: &std::path::Path) -> Result<Self, crate::protocol::RuntimeError> {
        let key = read_credential_file(path)?;
        Ok(Self { key })
    }
    pub fn matches(&self, other: &str) -> bool {
        self.key == other
    }
    pub(crate) fn configure(&self, command: &mut tokio::process::Command) {
        command
            .env("OPENAI_API_KEY", &self.key)
            .env(
                "HTTPS_PROXY",
                "http://thought-khoral-codex-provider-proxy:3128",
            )
            .env(
                "HTTP_PROXY",
                "http://thought-khoral-codex-provider-proxy:3128",
            );
    }
    pub(crate) fn appears_in(&self, value: &serde_json::Value) -> bool {
        match value {
            serde_json::Value::String(text) => text.contains(&self.key),
            serde_json::Value::Array(values) => values.iter().any(|value| self.appears_in(value)),
            serde_json::Value::Object(values) => values
                .iter()
                .any(|(key, value)| key.contains(&self.key) || self.appears_in(value)),
            _ => false,
        }
    }
}

pub fn read_credential_file(
    path: &std::path::Path,
) -> Result<String, crate::protocol::RuntimeError> {
    use crate::protocol::RuntimeError;
    use std::io::Read;
    let file = std::fs::File::open(path)?;
    if !file.metadata()?.is_file() {
        return Err(RuntimeError::InvalidTaskInput);
    }
    let mut bytes = Vec::new();
    file.take(4097).read_to_end(&mut bytes)?;
    if bytes.len() > 4096 {
        return Err(RuntimeError::InvalidTaskInput);
    }
    let key = String::from_utf8(bytes).map_err(|_| RuntimeError::InvalidTaskInput)?;
    let key = key.trim_end_matches(['\r', '\n']).to_owned();
    if key.len() < 16 || !key.bytes().all(|byte| byte.is_ascii_graphic()) {
        return Err(RuntimeError::InvalidTaskInput);
    }
    Ok(key)
}
