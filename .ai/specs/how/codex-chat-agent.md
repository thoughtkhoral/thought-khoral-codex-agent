# Headless Codex chat agent — technical design

## Status

Draft — review before contract or runtime changes. Follows
[the What specification](../what/codex-chat-agent.md) and
[Decision 002](../decisions/002-independent-codex-agent.md).

## Data flow and boundaries

Human chat composer → authenticated room gateway → authorized durable task →
agent gateway → pinned local Codex A2A worker → Codex app-server → validated task
result → persisted room event → chat UI.

Implement the Codex worker in this independent repository,
`thought-khoral-codex-agent`, with its own specifications, tests, image, and
release version. Deploy it as a dedicated platform service. Keep Reference
Agent routing and exact deterministic result validation intact. The gateway
admits a pinned Codex registration and validates its versioned conversational
result contract; it does not launch Codex or understand its session files.
Do not make the existing registration accept arbitrary endpoints or relax its
validation for all agents.

## Bring-your-own-agent integration boundary

The contracts repository owns the versioned ThoughtKhoral integration profile
used alongside A2A task transport. Define conversation new/continue semantics,
capability discovery, optional model/effort choices, effective settings, optional
usage reports, and normalized failures. Do not assume those are all standard
A2A fields: document the profile's extension/control interfaces explicitly.

An admitted capability manifest declares support for conversations, model
selection, reasoning effort, and context telemetry. Dynamic choices come from
the agent through an authenticated, bounded gateway-mediated query. The UI
renders supported controls; capability absence does not require another agent
to imitate Codex. Use declared option identifiers rather than a globally fixed
Codex effort enumeration. Runtime-native IDs remain opaque to the platform.
Pin and review capabilities with the registration; self-advertisement does not
grant authority, and capability drift fails closed until admitted.

The independently operated agent owns its provider credentials, native history,
runtime upgrades, and mapping to platform conversation IDs. ThoughtKhoral owns
who may invoke it, disclosed inputs, conversation ownership, task lifecycle,
and accepted results. Keep the invocation credential separate from provider
authentication. Agents have no direct room database access.

The initial platform configuration installs one reviewed local Codex service.
User-hosted endpoints and self-service onboarding are future work requiring
explicit enrollment, endpoint/network trust, workload authentication, tenant
isolation, version negotiation, and revocation specifications. Repository
separation alone does not enable arbitrary agent admission.

## Conversation state

The room gateway persists a platform conversation UUID, room, requester,
agent, generation, lifecycle state, active task, resolved model and effort,
latest usage snapshot with thread/turn/model binding, and timestamps. Enforce one
active conversation for each owner/room/agent. The worker persists the mapping
from platform conversation UUID to Codex thread ID and per-task execution
receipts alongside its persistent Codex session storage. Browser clients use
only the platform ID. Mapping requests are task-authorized and bound to the
conversation generation; client-provided Codex thread IDs are never accepted.

A task reserves its conversation atomically in the room database before
dispatch. A second concurrent turn receives `conversation_busy`; requests are
not automatically reordered. Starting fresh is also rejected while a turn is
active. Each accepted request has a stable idempotency fingerprint containing
prompt, mode, conversation, generation, model, and effort. Replaying the request
returns the same task rather than launching another process.

## Proposed contract changes

Extend the versioned agent-task request with an optional `conversation`
object for Codex: `mode` is `new` or `continue`; continuing requires its
platform `id`. Absence remains valid for existing deterministic requests.
The gateway derives requester identity from authenticated claims.

Add an authenticated room-scoped query for the caller's active conversation,
so reload does not depend on browser storage or inferring ownership from chat.
Return platform conversation ID, generation, state, and active task ID.
Also return runtime-confirmed model/effort and the latest telemetry snapshot
with its freshness status. Add an authenticated model-catalog query through the
worker, restricted by the deployment's model allowlist. Add explicit `model`
and `reasoningEffort` fields for Codex turns; validate the pair server-side
against the current runtime catalog. Existing agents do not accept these fields.
Carry the conversation and settings binding in the internal task packet, canonical hash,
worker invocation, normalized update, and persisted task result. Unknown
agents, invalid mode combinations, and cross-owner IDs are rejected.

Define Codex success as a bounded result containing conversation ID and
assistant text. Optional citations must refer to disclosed inputs. Result
metadata includes confirmed model/effort and optional usage. Bind
settings and usage to the task's conversation generation and runtime turn ID.
Record safe settings/usage updates through the room gateway for live rendering
and replay; do not expose raw app-server messages to the browser. Model text
does not inherit the deterministic agent's exact-output guarantee. Preserve
the retained room protocol version using additive schemas and compatibility
fixtures. Contract owners must verify old requests still validate.

## Headless app-server process and memory

Pin and verify the CLI version in the runtime image. Launch it with a process
API and argument vector, never a shell command assembled from chat text.
Use newline-delimited protocol messages over the subprocess's stdin/stdout.
The headless command is:

```text
codex app-server --listen stdio://
```

Initialize each connection with `initialize` and `initialized`. Create via
`thread/start`, resume by the stored exact ID via `thread/resume`, and submit
the prompt via `turn/start` with explicit `model` and `effort`. Set read-only
sandbox policy and `approvalPolicy: "never"` at thread creation/resume and on
each turn. Never bypass sandboxing or execute model-requested client tools.
If the runtime requests an interactive approval, reject it or fail safely;
do not leave the turn waiting for an absent operator. Set an isolated working
directory and persistent worker-managed CODEX_HOME. Provision a minimal config
with no inherited operator MCP, hook, or plugin configuration. The runtime container exposes no
operator home, host repository, room database, or platform credentials.

Persist the returned `thread.id` from `thread/start` before submitting the first
turn, then bind the returned turn ID to its task receipt. Parse stdout as
bounded JSONL, correlate RPC replies by request ID and notifications by thread
and turn IDs, and consume only approved methods. A successful reply requires
a matching `turn/completed` with status `completed` and a nonempty final
assistant message. The long-lived process does not exit after a successful
turn. Failed/interrupted turns, protocol errors, unexpected process exit, or
missing completion fail the task. Drain bounded stderr for diagnostics;
do not publish raw stderr, reasoning, tool output, or credentials into chat.

Resume the exact thread in the same persistent store. Codex restores previous
turns; no second memory service reconstructs them. For a new conversation,
send only the current prompt and fixed agent instructions. Do not seed from
previous Codex replies in room history. Authorization remains checked for
every turn. Losing room access ends use of the old conversation; returning
after revocation requires a fresh generation.

## Model and reasoning-effort controls

Fetch all pages of `model/list` using the service's configured provider. Expose
picker-visible entries allowed by platform policy, their display names,
`defaultReasoningEffort`, and `supportedReasoningEfforts`. Treat this as a
catalog rather than proof of account entitlement. Refresh after worker restart
and reject removed/unsupported selections without fallback inference.

The UI displays resolved default selections before the first prompt. Changing
the model retains the selected effort only when supported by the new model;
otherwise display its catalog default and require acknowledgement before
submission. Controls are disabled while a task is queued or running. The human
may change both settings between turns without creating a new conversation.
Include the explicit pair in the next task fingerprint and invocation.

Confirm effective settings from thread/start or thread/resume responses and
the active-turn metadata exposed by the pinned protocol. Do not present a
browser selection as runtime-confirmed. Persist accepted defaults for future
turns and settings used for each reply. Where the runtime reports a model
reroute, show the reported model for that turn separately from the selected
default. If confirmation is missing, label the setting unconfirmed.

## Context-window indicator

Consume `thread/tokenUsage/updated` only for the active task's thread/turn and
generation. The local 0.160.0 generated schema contains `tokenUsage.last`,
`tokenUsage.total`, and nullable `tokenUsage.modelContextWindow`. Keep aggregate
`total` counters separate from window occupancy.

When the last report is valid and the window is positive, calculate an estimate
as `100 * tokenUsage.last.totalTokens / modelContextWindow`. Label it **Estimated
context used (last request)**; this is a runtime usage-based estimate, not an
exact count of the next prompt. Cached/reasoning token fields are breakdowns
and are not added again. Show the reported token count/window, timestamp,
and stale status in a tooltip. Bound the visual meter to 0–100%; if the raw
estimate exceeds 100%, preserve that information in its text rather than
silently presenting it as an ordinary 100% report.

New conversations start with no reading. On a model change, invalidate the old
denominator until a new bound report arrives. Compaction invalidates old usage
until a fresh report is available, so usage may decrease. On resume, saved
telemetry is labelled last reported rather than live unless refreshed. Missing
or invalid fields produce **Context usage unavailable**, not an invented zero
or a ratio based on advertised model limits or cumulative billing tokens.

Verify these field semantics with the pinned runtime during integration; the
schema establishes available fields, not exact next-turn occupancy. If the
runtime cannot supply a reliable numerator/window pair, retain the unavailable
state and ship model/effort controls independently.

## Timeouts and recovery

Use a Codex-specific configurable turn deadline, initially 180 seconds, inside
the existing five-minute lease with a submission margin. The Reference Agent
retains its ten-second execution budget. Limit concurrent workers, JSONL line
size, total output, and final reply size using explicit validated configuration.
Interrupt the bound turn on deadline, lost authorization, or lost lease. If
interruption is not acknowledged promptly, terminate and reap its dedicated
subprocess tree. Terminate and reap on worker shutdown. Do not allow terminating
one user's turn to disrupt another: each live conversation uses a separate
app-server process, bounded by a configurable worker concurrency limit. Late
results cannot mutate the room or conversation.

Persist a task receipt as reserved before launching, then running with thread
binding, then completed with normalized output. A recovered completed receipt
can replay its result without another CLI invocation. A recovered uncertain
running receipt must fail with `conversation_interrupted`, mark the conversation
unusable, and require a fresh session. Do not promise exactly-once provider
execution across process/database crashes. Prevent broker lease reclamation
from blindly reinvoking an uncertain task.

Mark a failed turn's conversation unusable when the worker cannot establish
whether its history was modified. A lost result acknowledgement retries only
the already recorded result. Failure codes distinguish runtime unavailable,
authentication required, session unavailable, busy, timeout, and interrupted;
messages remain safe for room display.

## Platform

Provide an opt-in local Codex service with dedicated persistent session storage.
Supply service-owned provider authentication from local secret configuration.
Use an API-key authentication path for the initial automated deployment;
personal ChatGPT session import is outside scope. The UI advertises Codex only
when its configured capability is enabled.

Give this runtime separately reviewed provider egress through a restricted
proxy or equivalent host allowlist. Model traffic can reach only the configured
provider HTTPS origin; arbitrary redirects and destinations are rejected.
Keep existing reference-agent network restrictions in place. A read-only
filesystem alone is insufficient to restrict network access. Review credential
injection and session-volume permissions before enabling the live service.

## Repository work

- Contracts: versioned agent integration profile, admitted capability shape,
  conversation/model queries, turn settings/usage, result schemas, and
  compatibility fixtures.
- Room gateway: conversation migration/store, authorization and idempotency,
  task reservation, agent-aware claiming, internal binding, and recovery.
- Agent gateway: pinned agent admission, authenticated capability queries,
  agent-aware dispatch, normalized result validation, and scoped invocation.
- New Codex agent repository: A2A worker, app-server adapter, model/effort
  discovery, usage normalization, native session storage, persistent receipts,
  runtime image, independent specifications, and tests.
- Workspace UI: explicit Codex target, fresh/continue controls, active-session
  restoration, model/effort selectors, confirmed settings, context indicator,
  busy state, and final assistant reply in the chat stream.
- Platform: opt-in service/image, secrets, persistent storage, restricted egress,
  configuration, and integration smoke test.

Before implementation, mirror approved scope into the affected child
specifications and record the owning issue for contribution traceability.
The approved repository foundation registers this independent child in the root
workspace. Runtime admission and compatibility remain separate approval gates.

## Verification

Use a fake app-server executable to verify startup/handshake, exact-ID resume,
captured thread/turn bindings, malformed/oversized JSONL, failure events,
timeout/process cleanup, session loss, completed-receipt replay, and uncertain
restart recovery.
Gateway integration tests verify owner/room boundaries, fresh generations,
duplicate requests, simultaneous turns, stale leases, and membership loss.
Adapter tests verify catalog pagination, model-specific efforts, confirmed
settings, setting changes on the same thread, and model-access errors without
fallback. Usage tests cover last versus cumulative counts, missing/zero windows,
cached/reasoning breakdowns without double counting, over-capacity readings,
out-of-order/wrong-thread updates, fresh-session reset, model switches,
compaction, and stale restoration. UI tests verify start/continue/new controls,
model/effort selectors, unsupported effort acknowledgement, visible settings,
meter/unavailable states, and restoration after reload.
Run existing contract, gateway, UI, and platform regression gates.
Verify capability negotiation with a non-Codex fixture that supports
conversation turns but omits model, effort, and usage controls. Build the Codex
agent independently against a pinned contract revision, and verify the gateway
can invoke its reviewed image without linking its runtime implementation.

An opt-in live smoke test sends a unique fact, asks for that fact on a resumed
turn, restarts the worker and repeats, then starts fresh and verifies a distinct
thread and no injected old turns. Verify freshness structurally; a model's
answer alone is not proof that history was excluded. Change model and effort
between turns and verify preserved thread identity, restored history, runtime
confirmation, and telemetry from that turn. A second model is exercised only
when accessible to the configured account; record coverage explicitly. Live verification needs
configured provider authentication and restricted egress; do not claim it has
passed based on fake app-server tests.
