# Pinned tool-selection correction

Source: [OpenAI Codex a956835d model catalog](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/models-manager/models.json),
release CLI0.160.0. Original bytes are `upstream-models.json`, SHA256
`fd219bd9f061278275f528939f82f54d2eb97df4b25c23b022adbe48813d920b`.
Upstream Apache2.0 license and notices are retained in this directory.

`restricted-models.json` changes exactly six tool-selection fields on each of the
11 descriptors: shell_type=disabled, apply_patch_tool_type=null,
experimental_supported_tools=[], tool_mode=direct, supports_search_tool=false,
multi_agent_version=null. All non-tool metadata is preserved.
`tool-controls.json` contains the exact startup controls used by the adapter and
actual-binary capture. `lock.json` pins all artifacts.

[Upstream tool registration](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/spec_plan.rs)
requires these model fields in addition to feature controls. The
[static manager](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/models-manager/src/manager.rs)
prevents refresh from adding unsanitized metadata. The
[effort resolver](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/protocol/src/openai_models/reasoning_effort.rs)
explains why a native Ultra setting has a different wire reasoning effort.

`tool-policy-proof-aarch64.json` is actual Linuxaarch64 CLI request-capture output,
44 native-visible model/effort cases plus one fresh-process resume, using only a
loopback synthetic HTTP/SSE server in a network-disabled container. Nested tool
arrays were empty; model and native effort were preserved; wire effort follows
the pinned resolver. It is not provider access or inference evidence. Only this
CLI binary digest is admitted by the current package verifier; x86_64 fails closed.

Run `python3 scripts/check_tool_policy.py` and
`python3 scripts/check_contract_pins.py` from the repository root. Regenerate
actual evidence with `python3 scripts/test-native-tool-policy.py OUTPUT.json` in
the approved isolated test environment. Its source CLI image must resolve to the
recorded Task6 image ID before it can start; it uses no real provider key.

The actual native harness also injected six unsolicited function calls: apply_patch,
exec_command, shell, exec, client_tool and an MCP canary. Each produced a native
unknown/unsupported tool result tied to its call ID and tool name; no client tool
request or filesystem canary occurred. Worker protocol regression tests separately
reject forbidden tool/approval events. This is representative negative handler
coverage, not a claim that every possible hostile native event was enumerated.
