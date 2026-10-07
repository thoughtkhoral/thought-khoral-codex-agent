# Durable Codex worker

Governing sources: approved [runtime design](../.ai/specs/how/codex-chat-agent.md)
and [conversation profile](../.ai/specs/how/conversation-integration-profile.md).
Contribution: [issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1).

Task 5 adds the independent worker, authenticated A2A v1 JSON-RPC service,
worker-owned SQLite receipts and image. Tests use a synthetic subprocess and
real temporary SQLite files. They do not call a provider. Deployment activation,
live provider execution and deployment activation remain later gates. Task8 now
verifies the initial tool policy for the reviewed Linuxaarch64 package.

## Durable boundaries

`Worker::execute` validates authority, context and fixed guidance before reserving
one room/agent scope. Four slots bound concurrent subprocesses. A reservation is
committed before startup; submission intent and exact native thread are committed
before `turn/start`; its exact turn ID is committed immediately after the response.
SQLite uses WAL, FULL synchronization and exclusive worker ownership.

Normalized output is committed before success is returned. Cancellation remains
admissible while completion waits on SQLite. A terminal mutex serializes the final
commit against cancellation; once final commit begins, completion wins a concurrent
cancel. Authority is rechecked immediately before that commit. Errors during uncertain
persistence never authorize another submission. Completed output is replayed only
under current live authority and the matching task/context/generation/policy.

Processes close and reap after durable completion. The conversation remains pending
acknowledgement until the broker's exact accepted reply binding is stored. Continuation
requires that acknowledgement, the committed base revision and retained native history.
Acknowledged replies already in the native thread appear as provenance bindings,
never duplicate transcript text. Missing sessions, uncertain recovered execution or
invalidated policy require a new baseline. Policy invalidation persists across old
packet retries. First-turn pre-submit failures and uncertain submitted turns have
separate receipt phases; restart never blindly launches an uncertain task again.

## Transport

All endpoints require the distinct invocation Bearer credential, including
`/.well-known/agent-card.json`, JSON-RPC `POST /`, model discovery and receipts.
Admission has a trusted expiry; a submitted packet may not outlive it. JSON bodies
are bounded and duplicate keys rejected. Native A2A types and the reviewed server
handler interface are reused through an integer-preserving Axum wrapper. The
published server's protobuf JSON conversion changes integer values to floats, so
its router cannot transport canonical profile packets unchanged.

Only `SendMessage`, `GetTask` and `CancelTask` are supported. A send carries exactly
one DataPart containing the closed `{profileVersion, packet: TaskInput}` envelope,
with task/context IDs matching the inner packet. Bare packets, unknown envelope
fields and mismatched profile versions are rejected before inference.
Files, artifact URLs, extra parts, tools, handoffs, unsupported configuration and
query parameters fail closed. Streaming and push notifications are not advertised.
A completed A2A task has one closed `{reply: InternalReply, runtimeBinding:
{threadId, turnId}}` artifact, derived from its committed receipt. Get and cancel
return the same bound completed artifact. Native identifiers are private to the
worker and mediator; the mediator compares them against the worker receipt and
forwards only the normalized reply to the broker/browser. Session files and
credentials are never returned. Errors expose stable safe codes.

Authenticated `GET /control/v1/models` returns the entire admitted catalog in one
bounded page (maximum 100 model options); `nextCursor` is null. The adapter still
validates every native catalog page. `GET /control/v1/receipts/{taskId}` returns a
closed durable receipt projection with taskId, conversationId, generation, phase,
result (normalized reply), acknowledgement, error and runtimeBinding. The binding
is `{threadId, turnId}` only when completed, otherwise null. `POST /control/v1/receipts/{taskId}/ack` validates
the closed acknowledgement profile and returns 204. HTTP waiter disconnection does
not cancel owned work; explicit cancellation, authority deadlines and shutdown do.

## Package and startup

The image pins the build/runtime base digests and Codex CLI 0.160.0 release hashes
for ARM64 and x86_64. Build checks the CLI version and generated stable v2 schema
hash without inference. UID/GID 10003 owns two distinct mode-0700 state directories:
`/var/lib/thought-khoral-codex/native` and `/var/lib/thought-khoral-codex/receipts`.
The fixed empty cwd is `/opt/thought-khoral-codex/workspace`, root-owned mode 0555;
fixed instructions are supplied by the adapter. Startup rejects imported native
`config.toml`/`auth.json`, invalid ownership and overlapping native/cwd paths.
The runtime listens on the admitted internal port 9091, with no publication step.

The executable refuses service startup unless the operator supplies
`THOUGHT_KHORAL_CODEX_ISOLATION_VERIFIED=1` after Task 8's isolation checks. Required
trusted environment configuration is:

- `THOUGHT_KHORAL_CODEX_MODELS`: comma-separated admitted native model IDs.
- `THOUGHT_KHORAL_CODEX_PROVIDER_KEY_FILE`: separately mounted provider credential.
- `THOUGHT_KHORAL_CODEX_INVOCATION_KEY_FILE`: separately mounted distinct invocation credential.
- `THOUGHT_KHORAL_CODEX_ADMISSION_EXPIRES_AT`: trusted RFC3339 admission expiry.

Credentials are bounded local files outside Git. Provider authentication uses a
fixed custom OpenAI Responses provider, environment key and the dedicated
`thought-khoral-codex-provider-proxy:3128`; arbitrary provider endpoints are not
accepted. See official [configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference).
The invocation credential is hashed in the HTTP service and never forwarded into
Codex. Subprocess environments are cleared. Replies/catalogs and private runtime bindings containing the exact provider key
are rejected before output or persistence. Shutdown cancels active
turns, closes their process groups and waits for reaping. Task 8 must verify real
network/tool isolation and read-only mounts before activation.

## Provider-free verification

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
python3 scripts/check_contract_pins.py
python3 scripts/check_dependency_notices.py
python3 scripts/check_docs.py
git diff --check
podman build -f Containerfile -t localhost/thought-khoral-codex-agent:task6-final .
podman run --rm --network none --read-only localhost/thought-khoral-codex-agent:task6-final --verify-package
```

The package check only prints the pinned CLI version and verifies UID/GID. CI runs
fake protocol/SQLite checks and a network-disabled package check. It never supplies
provider credentials or the activation flag. Dependency terms and upstream
attributions are retained under `/usr/share/licenses/thought-khoral-codex-agent`.
ARM64 was exercised locally; x86_64 packaging is covered by the CI definition and
still requires its own successful run. Rust 1.93.1 was exercised; the declared
1.85 minimum was not directly tested.

The 2026-10-06 checkpoint records 39 passing provider-free tests and independent
review with no remaining blocking findings. [Image evidence](image-evidence.json)
records the ARM64 local image identity and exact checked release/schema hashes.
Overlapping builds exposed overly short fixture deadlines; worker fixtures now
use ten seconds and the SQLite synchronization wait is bounded. The full suite
also needs process inspection for its descendant-reaping check.

## Task 8 package admission

The [governing runtime design](../.ai/specs/how/codex-chat-agent.md) records the
pinned metadata correction needed to remove all initial tools. A hashed static
catalog preserves upstream non-tool metadata while disabling model-selected tool
paths; explicit controls disable tools, agents, extensions and remote discovery.
Native model IDs/efforts still come from `model/list`. Exact admission and reviewed
capture coverage apply before execution and to reported model changes.

The actual CLI capture covered44 visible model/effort cases, a fresh-process
resume, and six unsolicited function calls. Those calls returned native
unknown/unsupported-tool outputs and did not run the canary. Package verification
binds the installed CLI binary, catalog, controls and this evidence. Its new
empty-tool marker is required by platform startup. Evidence covers Linuxaarch64;
Linuxx86_64 fails closed pending a reviewed equivalent capture. No provider
inference or account-access verification is implied.

The [Task8 image evidence](task8-image-evidence.json) records the reviewed local
image ID and runtime source commit; the earlier Task6 image record is retained
as historical evidence and does not pass the new tool-policy gate.
