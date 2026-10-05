# Room conversation integration profile — local adapter obligations

## Status

Approved by the project maintainer on 2026-10-05 under [issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1),
with root/sibling specifications and the exact contracts profile/plan. Consumer
implementation requires the verified immutable contract release; no runtime
code or released interoperability is claimed.

## Contract authority

The [contracts profile](https://github.com/thoughtkhoral/thought-khoral-contracts/blob/thought-khoral-agent-conversation-v1.0.0/.ai/specs/how/agent-conversation-profile.md) owns the exact
browser/workload API, field shapes, canonicalization, versioning, and fixture
requirements. Do not independently widen a retained room schema or copy A2A
transport types into room domain models. Consumers pin its immutable release
before implementation can ship.

The chosen browser interface is a separate versioned HTTP API. The workspace UI
submits one explicit human turn there; the gateway stores the prompt and reply
as ordinary room-wide message events on the existing room stream. Existing
chat.send messages do not silently acquire Codex inference triggers. Task state,
model controls, and telemetry use profile queries; terminal output renders once.

## Worker binding and persistence

Validate task/room/requester/agent identity, generation, base/end revisions,
policy and guidance revision, digest, deadline, and invocation authentication.
A fresh thread gets a complete eligible room baseline; continuation gets ordered
new transcript entries and validated bindings for prior native assistant replies.
Reconcile the broker's accepted reply event with the durable worker receipt
before resuming. The trigger text occurs once. Hidden sequence gaps are valid;
targeted messages and another room's history are excluded.

Use Rust edition 2024 with minimum Rust 1.85, matching the mediated A2A stack.
Use the reviewed A2A versions already recorded by the agent gateway and SQLx
0.8.6 with SQLite for worker-owned mappings/receipts, alongside persistent
CODEX_HOME. Commit the worker lockfile after dependency/license review; never
link gateway implementations or use room-database credentials.

A SQLite transaction records reserved before thread creation and submission
intent before turn/start. Completed output is durable before returning to the
mediator. Uncertain running receipts become interrupted; they are not replayed.
Completed receipts replay only normalized output. Record the accepted room reply
ID, sequence, and digest before another turn. Close idle app-server processes
after acknowledgement while retaining native files and the mapping. Missing files
or bindings require explicit new, never an invented replacement thread.

## Initial controls and deployment constraints

Pin Codex CLI 0.160.0 and generated stable protocol bindings in the image;
verify its version and generated schema checksums during release. Discover catalog
pages and model-specific efforts. Never silently substitute a denied model.
Normalize only completed assistant output and verified settings/usage bindings.
Keep unavailable telemetry unavailable. A settings event without a turn ID is
not proof of request routing; use the single bound turn and report uncertainty.

Initial bounds: 8,000 prompt characters, 1 MiB canonical context, 2,000 transcript
records, 1 MiB JSONL line, 4 MiB parsed output, 64 KiB final text, 180-second turn
deadline, five-second interrupt grace, and four live subprocesses. Fail visibly
on overflow before submission where possible; never silently omit room history.

Use fixed instructions in an isolated directory, separate from native state.
The initial deployment disables shell execution, filesystem tools, external MCP,
plugins, hooks, and client tool execution using a verified pinned configuration.
Read-only sandbox and never-approve policy remain defense in depth. A live
isolation check must demonstrate the tool restriction before service activation;
a fake server cannot prove it. Runtime model output cannot expose provider env
or authentication files. Provider API-key injection belongs to the worker only.

The [future guided workspace](../what/guided-workspace.md) remains milestone two.
Its first design can preload reviewed files into instructions/context without
widening tools; filesystem access and memory writes require separate approval.

## Execution and verification

The [coordinated implementation plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md) owns dependency ordering and
repository tasks; the local worker task owns this adapter. Required acceptance
coverage is in [chat requirements](../what/codex-chat-agent.md) and
[runtime design](codex-chat-agent.md). No runtime work begins until accepted issue
traceability and the complete written plan are approved.

## Published contract pin

The maintainer authorized the contracts release on 2026-10-05. The actual
[thought-khoral-agent-conversation-v1.0.0](https://github.com/thoughtkhoral/thought-khoral-contracts/releases/tag/thought-khoral-agent-conversation-v1.0.0) artifact resolves to
`85baf86e574276fcd036e53e23641af6aad602f9`. Its attached TAR has SHA-256
`0038fdbf858db013c4a269aeedbca51a5db9128a2f1d6e39754f92ba60fd8f36`.
The remote tag, GitHub asset digest and downloaded archive were verified;
both fixture suites pass from the download. Task 4 vendors that exact release
and records its own artifact lock before worker implementation. This source
records the pin; it does not claim the worker already vendors or implements it.
