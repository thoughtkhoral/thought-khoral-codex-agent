// SPDX-License-Identifier: Apache-2.0
use std::path::Path;
use thought_khoral_codex_agent::app_server::AppServer;
#[allow(dead_code)]
mod support;
#[tokio::test]
async fn unknown_runtime_model_change_is_rejected_instead_of_reported_unconfirmed() {
    let fixture = support::Fixture::new("unknown_reroute");
    let mut server = AppServer::spawn(fixture.config.clone()).await.unwrap();
    assert!(
        server
            .execute(support::request(), |_| async { Ok(()) })
            .await
            .is_err()
    );
    server.close().await.unwrap();
}
#[test]
fn pinned_tool_catalog_is_present_before_runtime_admission() {
    assert!(
        Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/contracts/codex-app-server-0.160.0/restricted-models.json"
        ))
        .is_file(),
        "missing immutable restricted tool catalog"
    );
}

#[test]
fn catalog_tampering_and_unsanitized_metadata_are_rejected() {
    use thought_khoral_codex_agent::tool_policy::verify_catalog;
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("catalog.json");
    assert!(verify_catalog(&path).is_err());
    std::fs::write(
        &path,
        include_str!("../contracts/codex-app-server-0.160.0/upstream-models.json"),
    )
    .unwrap();
    assert!(verify_catalog(&path).is_err());
    let bytes = include_str!("../contracts/codex-app-server-0.160.0/restricted-models.json");
    std::fs::write(&path, bytes).unwrap();
    verify_catalog(&path).unwrap();
    let mut mutated: serde_json::Value = serde_json::from_str(bytes).unwrap();
    mutated["models"][0]["tool_mode"] = serde_json::json!("code_mode");
    std::fs::write(&path, mutated.to_string()).unwrap();
    assert!(verify_catalog(&path).is_err());
}
#[test]
fn model_metadata_requires_exact_reviewed_id_and_effort() {
    use thought_khoral_codex_agent::tool_policy::validate_selection;
    validate_selection("gpt-6.1-sol", Some("medium")).unwrap();
    for model in ["unknown", "provider/gpt-6.1-sol", "gpt-6.1-sol-unknown"] {
        assert!(validate_selection(model, Some("medium")).is_err());
    }
    assert!(validate_selection("gpt-6.1-sol", Some("unreviewed")).is_err());
}
#[test]
fn package_evidence_binds_cli_catalog_controls_and_actual_capture() {
    use thought_khoral_codex_agent::tool_policy::verify_evidence;
    let base = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/contracts/codex-app-server-0.160.0"
    ));
    let (digest, proof_name, other_proof) = match std::env::consts::ARCH {
        "aarch64" => (
            "50b06603bdcdac39b714f5c3e68583c002b8ad8779ebfdaaf4932ff016b379c0",
            "tool-policy-proof-aarch64.json",
            "tool-policy-proof-x86_64.json",
        ),
        "x86_64" => (
            "12eb3e81114588aca3b7998f4f19e8997b056aca08e57a7ca7c8a3ec8c652aad",
            "tool-policy-proof-x86_64.json",
            "tool-policy-proof-aarch64.json",
        ),
        architecture => panic!("unsupported package architecture: {architecture}"),
    };
    let catalog = base.join("restricted-models.json");
    let controls = base.join("tool-controls.json");
    let proof = base.join(proof_name);
    verify_evidence(&catalog, &controls, &proof, digest).unwrap();
    assert!(verify_evidence(&catalog, &controls, &proof, "wrong-cli-digest").is_err());
    assert!(verify_evidence(&catalog, &controls, &base.join(other_proof), digest).is_err());
    let directory = tempfile::tempdir().unwrap();
    let tampered = directory.path().join("controls.json");
    std::fs::write(&tampered, "{}").unwrap();
    assert!(verify_evidence(&catalog, &tampered, &proof, digest).is_err());
    assert!(verify_evidence(&catalog, &controls, &tampered, digest).is_err());
    let tampered_catalog = directory.path().join("catalog.json");
    std::fs::copy(&catalog, &tampered_catalog).unwrap();
    let mut bytes = std::fs::read(&tampered_catalog).unwrap();
    bytes.push(b' ');
    std::fs::write(&tampered_catalog, bytes).unwrap();
    assert!(verify_evidence(&tampered_catalog, &controls, &proof, digest).is_err());
}
