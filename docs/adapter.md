# Provider-free adapter library

## Current workspace context — 2026-11-20

Native AMD64/ARM64 worker acceptance remains tied to tested source `9bcdbde` and its final lock. The 2026-11-20 provider-free ARM64 image from source `3f0318b` passed ten package/startup probes and exact native policy recapture: 44 model/effort cases, eight models, resume and six refusals. Published contract v1.1 and locally prepared released consumers/profile do not add provider, assembled-stack, browser, registry-image or production acceptance. Worker/mediator v1.0 pins remain compatible and unchanged.

Runtime source and contract pins are unchanged. Exact clean pinned-source
snapshots were retained before this documentation-only checkout advanced.

Derived from the [local specification index](../.ai/specs/README.md) and
[root workspace authority](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/workspace-state.md).
The cited tests retain their original source identities; this documentation
refresh adds no activation, publication or runtime acceptance.

Task 4 implements the approved [runtime design](../.ai/specs/how/codex-chat-agent.md)
and [conversation integration profile](../.ai/specs/how/conversation-integration-profile.md).
This page covers the adapter library; the implemented server and durable receipt
boundary are documented in the [worker runtime](runtime.md).

`AppServer::spawn(Config)` checks the exact CLI version 0.160.0 and launches
`codex app-server --listen stdio://` with an argument vector and cleared
environment. Configuration supplies separate existing working/native directories,
model-ID allowlist and fixed guidance/catalog revisions. Bounds can only lower
the approved maxima: 180 seconds, 5-second interrupt grace, 1 MiB per protocol
line, 4 MiB aggregate output, 64 KiB final UTF-8 text and 2,000 context records.

`execute(RuntimeRequest, before_submit)` validates the closed packet, digest,
generation, authority and catalog selection. It starts or resumes exactly the
host-supplied thread and calls the async barrier with that thread ID. The host
must durably commit mapping/submission intent there before turn submission.
Barrier failure prevents submission. Task 5 owns durable receipts, native reply
reconciliation, provider-key injection and authenticated transport.

The library accepts one execution per process. A successful outcome contains
private thread/turn IDs and a schema-validated normalized InternalReply. The host
closes it after execution; broker acknowledgement is a separate durable worker
boundary required before continuation. Error/timeout paths terminate the process group
and await child reaping. Cancelling execution kills the group and schedules
reaping even while the adapter is retained; a cancelled close can be awaited
again. Dropping the adapter kills its group and aborts reader tasks.

Only completed final messages from the matching completed turn become public
assistant text. Commentary, unfinished deltas and reasoning are excluded.
Approval requests, client tool requests, unexpected activity and malformed or
oversized streams fail safely without exposing process diagnostics.

Catalog discovery consumes all bounded pages and admits deployment-listed,
visible options only. Requested model/effort remain explicit with no fallback.
Effective settings use runtime confirmation; absent effort stays unconfirmed.
Reroutes remain separately reported and bound to the active turn. Usage uses
last.totalTokens without adding cached/reasoning breakdowns; model change,
compaction and reset invalidate earlier readings. Missing windows remain
unavailable, and over-capacity readings retain their original numerator.

Explicit pinned configuration disables shell, execution, image, browser,
computer, apps/plugins/hooks, multi-agent and other tool features; web search,
MCP servers, plugins and project-document loading are also disabled. These
provider-free tests verify requests and process behavior. They cannot prove
live tool or network isolation. Task 8/9 must prove those controls before
activation. This unit never reads operator configuration or contacts a provider.

## Verify

Use Rust/Cargo and Python 3 for the deterministic external fake:

```sh
cargo test --locked --offline
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 scripts/check_contract_pins.py
python3 scripts/check_docs.py
git diff --check
```

Offline Cargo commands require cached dependencies. The corrected reviewed lock
passed all 46 worker tests on actual Rust 1.85.0 and 1.93.1 on macOS ARM64 and
native Linux AMD64/ARM64. The [approved compatibility correction](../.ai/specs/how/rust-185-lock-compatibility-proposal.md)
preserves the declared 1.85 minimum. See [native acceptance](linux-acceptance.md),
[dependency evidence](dependency-evidence.json) and [third-party notices](../THIRD_PARTY_NOTICES.md)
for the exact lock, target scope and retained legal evidence. These tests do not
establish a full room-stack or live-provider conversation.
