// SPDX-License-Identifier: Apache-2.0
//! Immutable tool-selection metadata from the pinned CLI, never a model allowlist.
use crate::protocol::RuntimeError;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::Path;
pub const CATALOG_PATH: &str = "/opt/thought-khoral-codex/restricted-models.json";
pub const CATALOG_HASH: &str = "493372f14bbd5db95ee05d997d052990aff0c787d9336fa9d95ac5049f9cc9d1";
pub const CONTROLS: &str = include_str!("../contracts/codex-app-server-0.160.0/tool-controls.json");
const CATALOG: &str = include_str!("../contracts/codex-app-server-0.160.0/restricted-models.json");
pub fn verify_catalog(path: &Path) -> Result<(), RuntimeError> {
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.len() > 2_097_152 {
        return Err(RuntimeError::RuntimeUnavailable);
    }
    let bytes = std::fs::read(path)?;
    if format!("{:x}", Sha256::digest(&bytes)) != CATALOG_HASH {
        return Err(RuntimeError::RuntimeUnavailable);
    }
    Ok(())
}
pub fn validate_selection(model: &str, effort: Option<&str>) -> Result<(), RuntimeError> {
    let catalog: Value =
        serde_json::from_str(CATALOG).map_err(|_| RuntimeError::RuntimeUnavailable)?;
    let entry = catalog["models"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["slug"] == model)
        .ok_or(RuntimeError::InvalidTaskInput)?;
    if effort.is_some_and(|effort| {
        !entry["supported_reasoning_levels"]
            .as_array()
            .unwrap()
            .iter()
            .any(|level| level["effort"] == effort)
    }) {
        return Err(RuntimeError::InvalidTaskInput);
    }
    let proof: Value = serde_json::from_str(PROOF).map_err(|_| RuntimeError::RuntimeUnavailable)?;
    if !proof["cases"]
        .as_array()
        .unwrap()
        .iter()
        .any(|case| case["model"] == model && effort.is_none_or(|effort| case["effort"] == effort))
    {
        return Err(RuntimeError::InvalidTaskInput);
    }
    Ok(())
}

const PROOF: &str =
    include_str!("../contracts/codex-app-server-0.160.0/tool-policy-proof-aarch64.json");
pub const CONTROLS_PATH: &str = "/opt/thought-khoral-codex/tool-controls.json";
pub const PROOF_PATH: &str = "/opt/thought-khoral-codex/tool-policy-proof-aarch64.json";

pub fn verify_evidence(
    catalog: &Path,
    controls: &Path,
    proof: &Path,
    cli_digest: &str,
) -> Result<(), RuntimeError> {
    verify_catalog(catalog)?;
    if std::fs::read(controls)? != CONTROLS.as_bytes() || std::fs::read(proof)? != PROOF.as_bytes()
    {
        return Err(RuntimeError::RuntimeUnavailable);
    }
    let evidence: Value =
        serde_json::from_str(PROOF).map_err(|_| RuntimeError::RuntimeUnavailable)?;
    if evidence["cliVersion"] != "0.160.0"
        || evidence["cliSha256"] != cli_digest
        || evidence["catalogSha256"] != CATALOG_HASH
        || evidence["controlsSha256"] != format!("{:x}", Sha256::digest(CONTROLS.as_bytes()))
        || evidence["providerInference"] != false
        || evidence["cases"].as_array().is_none_or(|cases| {
            cases.is_empty()
                || cases
                    .iter()
                    .any(|case| case["tools"] != serde_json::json!([]))
        })
        || evidence["resume"]["tools"] != serde_json::json!([])
        || evidence["toolCallRejections"]
            != serde_json::json!([
                "apply_patch",
                "exec_command",
                "shell",
                "exec",
                "client_tool",
                "mcp__task8__canary"
            ])
    {
        return Err(RuntimeError::RuntimeUnavailable);
    }
    Ok(())
}

pub fn verify_installed_package() -> Result<(), RuntimeError> {
    use std::io::Read;
    let mut file = std::fs::File::open("/usr/local/bin/codex")?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    verify_evidence(
        Path::new(CATALOG_PATH),
        Path::new(CONTROLS_PATH),
        Path::new(PROOF_PATH),
        &format!("{:x}", hash.finalize()),
    )
}
