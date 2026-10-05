// SPDX-License-Identifier: Apache-2.0
use std::{ffi::OsString, path::PathBuf, time::Duration};
#[derive(Clone)]
pub struct Config {
    pub executable: PathBuf,
    pub arguments: Vec<OsString>,
    pub working_directory: PathBuf,
    pub native_home: PathBuf,
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
            executable,
            arguments: vec![],
            working_directory,
            native_home,
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
    let names = [
        "shell_tool",
        "unified_exec",
        "view_image",
        "apps",
        "plugins",
        "hooks",
        "browser_use",
        "browser_use_external",
        "browser_use_full_cdp_access",
        "in_app_browser",
        "computer_use",
        "code_mode_host",
        "multi_agent",
        "multi_agent_v2",
        "image_generation",
        "remote_plugin",
        "plugin_sharing",
        "sleep_tool",
        "goals",
        "tool_suggest",
        "workspace_dependencies",
        "realtime_conversation",
    ];
    let features = names
        .into_iter()
        .map(|name| (name.to_owned(), serde_json::Value::Bool(false)))
        .collect::<serde_json::Map<_, _>>();
    serde_json::json!({"features":features,"web_search":"disabled","mcp_servers":{},"plugins":{},"project_doc_max_bytes":0,"shell_environment_policy":{"inherit":"none"}})
}
