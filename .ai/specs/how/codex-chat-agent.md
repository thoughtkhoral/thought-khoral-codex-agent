# Headless Codex chat agent — technical design

## Status

Approved by the project maintainer in the Codex working session on 2026-10-05,
including this milestone-one specification and the coordinated implementation
plan. Accepted contribution: [issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1).
Implementation follows the [plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md) and its dependency gates.
Release/tag publication, provider use and service activation require their
separate later authorization. No completed runtime or live verification is claimed.

## Data flow and boundaries

Human explicitly addresses Codex → authenticated room gateway → authorized
room transcript and durable task →
agent gateway → pinned local Codex A2A worker → Codex app-server → validated task
result → persisted room event → chat UI.

Implement the Codex worker in this independent repository,
`thought-khoral-codex-agent`, with its own specifications, tests, image, and
release version. Deploy it as a dedicated platform service. Keep Reference
Agent routing and exact deterministic result validation intact. The gateway
admits a pinned Codex registration and validates its versioned conversational
result contract; public replies are ordinary persisted room messages, while
profile task lifecycle and telemetry remain separate; it does not launch Codex or understand its session files.
Do not make the existing registration accept arbitrary endpoints or relax its
validation for all agents.

## Bring-your-own-agent integration boundary

The contracts repository owns the versioned ThoughtKhoral integration profile
used alongside A2A task transport. Define conversation new/continue semantics,
capability discovery, optional model/effort choices, effective settings, optional
usage reports, and normalized failures. Do not assume those are all standard
A2A fields: document the profile's extension/control interfaces explicitly.
The [proposed conversation integration profile](conversation-integration-profile.md)
links the contracts-owned exact API, field shapes, canonicalization, and fixtures.
The profile is separately versioned HTTP; the retained room stream carries only
existing message events for public prompts and replies. Published artifacts are
a prerequisite to consumer implementation.

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

The room gateway persists a platform conversation UUID, room,
agent, generation, lifecycle state, active task, visibility-policy revision,
consumed context revision, accepted native-reply bindings, resolved model and effort,
latest usage snapshot with opaque task/generation/model binding, and timestamps.
Native thread/turn IDs remain worker/mediator-private. Enforce one
active conversation for each room/agent. Requester identity is task-scoped.
Any currently authorized human may continue or explicitly reset the room session;
the gateway checks room chat and agent-invocation permission on each operation.
No possession of an ID grants authority. The worker persists the mapping
from platform conversation UUID to Codex thread ID and per-task execution
receipts alongside its persistent Codex session storage. Browser clients use
only the platform ID. Mapping requests are task-authorized and bound to the
conversation generation; client-provided Codex thread IDs are never accepted.

A task reserves its conversation atomically in the room database before
dispatch. A second concurrent turn receives `conversation_busy`; requests are
not automatically reordered. Starting fresh is also rejected while a turn is
active. Compare submitted intent using the retained room/request-ID idempotency
key, with authenticated requester included in the fingerprint and authorization
check, before resolving defaults or creating the trigger event.
The accepted task freezes authenticated requester, trigger event, mode,
conversation, generation, model, and effort, plus the server-selected context
base/revision, policy revision, and context digest. Replaying the same request
returns its original task and binding rather than resolving new defaults or
launching another process.

## Explicit invocation and room history

The room gateway resolves canonical participant identities before accepting a
turn. A room-wide human message submitted through the versioned conversation
API with a direct Codex mention or equivalent composer selection invokes one task.
A retained chat.send Codex mention remains ordinary chat. Selection and
mention normalize to the same target. Alias expansion, quoted text, ordinary
messages, and agent-authored events never trigger execution. Targeted delivery
is rejected for Codex chat. Persist the invoking room message and task reservation
in one transaction; triggerEventId identifies that message. A busy rejection
writes neither the invoking message nor a new task. Ordinary chat remains available.

Construct a complete authorized snapshot at reservation, ending at the invoking
message's sequence. Include ordered room-wide human messages and accepted
room-wide agent replies, with event ID, sequence, author identity, timestamp,
and text. Include the authorized room-wide active-decision projection with
source IDs. Apply requester and admitted-agent visibility checks. Exclude all
targeted messages, even if this requester can see them, task progress, credentials,
and other rooms. Source text is untrusted discussion data; it cannot change
worker configuration, tools, or policy.

On a fresh native thread send the complete eligible transcript through this
boundary. On continuation send eligible entries after the last committed
consumed revision. Gaps over excluded events are valid; the cursor tracks the
room sequence. The trigger is an entry in this packet; its text is not appended
again as a separate prompt. Fixed instructions identify the trigger entry as
the request to answer and other entries as context. Messages arriving after
the snapshot belong to the next explicit invocation.

Room events are authoritative. Bind the previous native assistant response to
its accepted room event using task, generation, event ID, and exact-text digest.
When that reply appears in the next delta, send its provenance binding rather
than another copy of its text if already present in this native thread. Other
agents' replies and prior replies on a new thread are ordinary transcript entries.
Receipts record delivered event IDs, packet digest, and base/end revisions.
Advance the room cursor only in the transaction accepting the task's completed
result and assistant event. Result retries reuse this exact binding. Reconcile
the previous receipt and acknowledgement before starting another turn. A mismatch
never falls back to appending guessed history.

Validate the complete packet before turn submission. Limits apply to serialized
UTF-8 bytes and eligible entries. Over-limit context returns context_too_large;
never silently drop the oldest entries. Native compaction may summarize already
delivered turns, so full baseline delivery does not promise indefinite verbatim
recall. Application-level compaction or retrieval is separate future work.

Before resume and result acceptance, verify current requester authority, agent
admission, generation, and visibility-policy revision. A caller losing authorization or reaching its validated token expiry ends
that task's authority. Current room lifecycle has no durable membership or kick
operation; a future ACL must use the same policy-invalidation port. Removal of agent access, narrowing of transcript
visibility, or inability to verify previously disclosed sources invalidates the
room thread: abort its task and require a fresh baseline. Browser disconnection
or another human leaving does not itself erase still-authorized room history.
No stale native history is reused after an invalidating policy change.

## Proposed contract changes

The browser entry point is POST /api/agent-conversations/v1/turns,
as defined in the contracts profile. The UI does not also send chat.send. The resulting internal versioned task request carries
a conversation object: mode is new or continue; continuation requires its
platform ID and generation. Existing deterministic requests remain valid under
their existing entry points. The gateway derives requester identity from
authenticated claims.

Add an authenticated room-scoped query for the caller's active conversation,
so reload does not depend on browser storage or inferring session identity from chat.
Return platform conversation ID, shared room scope, generation, state, context
revision, and active task ID.
Also return runtime-confirmed model/effort and the latest telemetry snapshot
with its freshness status. Add an authenticated model-catalog query through the
worker, restricted by the deployment's model allowlist. Add explicit `model`
and `reasoningEffort` fields for Codex turns; validate the pair server-side
against the current runtime catalog. Existing agents do not accept these fields.
Carry the conversation and settings binding in the internal task packet, canonical hash,
worker invocation, normalized update, and persisted task result, together with
the context binding. Unknown agents, invalid mode combinations, cross-room IDs,
and unauthorized humans are rejected.

Define Codex success as a bounded result containing conversation ID and
assistant text. Optional citations must refer to disclosed inputs. Result
metadata includes confirmed model/effort and optional usage. Bind
settings and usage to the task's conversation generation; the mediator verifies
private runtime thread/turn IDs and forwards only normalized task-bound metadata.
Record safe settings/usage updates in profile task storage for authenticated
polling and replay; do not expose raw app-server messages to the browser. Model text
does not inherit the deterministic agent's exact-output guarantee. Preserve
existing retained room protocol semantics and compatibility fixtures. Contract
owners must approve the extension/release strategy in the profile before adding
methods, agent identities, or result discriminators; compatibility is not assumed
merely because fields are added.

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
each turn. Disable built-in shell/filesystem tools as well as external MCP,
plugins, hooks, and client tools in the verified initial deployment. Never bypass
sandboxing or execute model-requested client tools.
If the runtime requests an interactive approval, reject it or fail safely;
do not leave the turn waiting for an absent operator. Set an isolated working
directory and persistent worker-managed CODEX_HOME. Provision a minimal config
with no inherited operator MCP, hook, or plugin configuration. The runtime container exposes no
operator home, host repository, room database, or platform credentials.

Persist the returned `thread.id` from `thread/start` before submitting the first
turn, then bind the returned turn ID to its task receipt. Parse stdout as
bounded JSONL, correlate RPC replies by request ID and notifications by thread
and turn IDs, and consume only approved methods. Accumulate completed agentMessage
items, not reasoning/tool items or unfinished deltas. Prefer phase final_answer;
join multiple final items in their observed order with a newline. If the pinned
runtime omits phase, use its last completed agentMessage item only after matching
turn completion; never use a message explicitly labelled commentary as final.
The fake protocol fixtures and live smoke test must verify this fallback for the
pinned version. A successful reply requires
a matching `turn/completed` with status `completed` and a nonempty final
assistant message. A successful turn does not itself cause app-server to exit;
after durable result reconciliation, the worker closes and reaps the now-idle
process as specified in the profile. Failed/interrupted turns, protocol errors,
unexpected process exit during a turn, or missing completion fail the task.
Drain bounded stderr for diagnostics;
do not publish raw stderr, reasoning, tool output, or credentials into chat.

Resume the exact thread in the same persistent store. Codex restores previous
turns and receives the validated room delta; no memory service reconstructs its
native history. A new conversation receives fixed instructions and the complete
authorized room baseline, including prior public Codex replies as room sources.
Never import an archived native thread. Authorization and policy checks follow
the room-history rules above.

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
default. If confirmation is missing, label the setting unconfirmed. The adapter
must specify the pinned schema's thread start/resume model and reasoningEffort
fields, thread/settings/updated correlation, and turn-bound model/rerouted
handling in its implementation plan. A settings notification without a turn ID
is correlated only within the single reserved turn on that dedicated process;
it is not proof of a model request's actual routing. A null effort remains
unconfirmed.

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
unusable, and require a fresh session. The
[profile's state and crash rules](conversation-integration-profile.md) specify
write ordering, first-turn failure, and replacement behavior. Do not promise
exactly-once provider execution across process/database crashes. Prevent broker lease reclamation
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

Give this runtime separately reviewed provider egress through a dedicated
restricted HTTPS proxy. Initially api.openai.com is the sole provider HTTPS
origin; arbitrary redirects and destinations are rejected.
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
workspace. The coordinated child specifications and written plan were approved on
2026-10-05 with accepted issue traceability. Published contract compatibility,
dependency order and runtime admission/isolation remain execution gates.

## Future directory guidance

The initial process uses an isolated working directory containing only fixed
instructions, separate from CODEX_HOME and its persistent session store.
No client-supplied path is accepted. Preserve a worker-owned guidance identity
in the internal conversation binding: initially the fixed instruction revision.
Future immutable specification and curated-memory bundles can replace that
identity only through reviewed configuration and a new native thread. See
[guided workspace requirements](../what/guided-workspace.md). This extension does
not add host mounts, memory writes, or extra tools to the initial release.

## Verification

Use a fake app-server executable to verify startup/handshake, exact-ID resume,
captured thread/turn bindings, malformed/oversized JSONL, failure events,
timeout/process cleanup, session loss, completed-receipt replay, and uncertain
restart recovery.
Gateway integration tests verify existing room participation permissions, fresh
generations, duplicate requests, simultaneous turns, stale leases, token expiry,
and agent/disclosure-policy revocation. Future membership controls use the same
authorization port; browser Leave remains disconnection, not revocation.
History fixtures verify first invocation after multi-human discussion, another
human continuing the thread, intervening ordinary messages, hidden sequence
gaps, excluded targeted messages, single trigger inclusion, accepted native-reply
binding, policy invalidation, wrong base/digest, context limits, and reset with a
new baseline. Trigger tests prove aliases, quoted mentions, and agent replies
do not invoke Codex.
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
thread and a fresh authorized room baseline without importing the old native
thread. Verify freshness structurally: old public facts can legitimately be
available from room history, so a model's answer cannot prove thread freshness.
Begin the smoke test with a unique fact in an ordinary human room message,
then have a different human address Codex about it. Continue after additional
ordinary discussion and verify the delivered delta structurally.
Change model and effort
between turns and verify preserved thread identity, restored history, runtime
confirmation, and telemetry from that turn. A second model is exercised only
when accessible to the configured account; record coverage explicitly. Live verification needs
configured provider authentication and restricted egress; do not claim it has
passed based on fake app-server tests.

## Task 4 adapter execution decomposition

Task 4 is authorized after the verified gateway route checkpoint. The independent
library consumes the published conversation release and the stable generated
Codex CLI 0.160.0 protocol. `contracts/codex-app-server-0.160.0/lock.json` hashes
its unmodified generated v2 schema and initialize response. Schema generation
uses an isolated temporary home and does not start inference.

The host config supplies an executable and argument vector, an isolated cwd and
native home, a deployment model-ID allowlist, catalog/guidance revisions and
bounds that may only lower the approved maxima. The child receives a cleared
environment with worker-owned HOME/CODEX_HOME and fixed nonsecret defaults.
Provider-key/proxy injection, authenticated transport and durable receipt
implementation belong to Task 5/8, not this provider-free library unit.

Validate the complete packet and native mapping before creating/resuming a
thread. Catalog model IDs are opaque options mapping to the native `model`
field. Effort IDs are generated from each native catalog effort and mapped
inside the adapter; callers cannot pass a native enum directly. Discover every
catalog page, reject duplicates/cycles or unsupported choices, and never use a
hidden or disallowed model. Refresh catalog for each new process.

An async host barrier receives the exact thread ID after creation/resume and
before turn/start. Task 5 must commit mapping/submission intent there. Barrier
failure prevents submission and reaps the process. The library does not claim
durable or exactly-once execution independently. One execute operation per
process avoids unacknowledged history reuse; the host closes successful idle
processes after result reconciliation. Error, timeout, cancellation and Drop
terminate the dedicated process group; explicit close awaits child reaping.

JSONL stdout and stderr are bounded independently and collectively. Parse
unique object keys, correlate replies by RPC ID, reject server requests for
approval/client tools, and reject activity from another thread/turn. Select
completed agentMessage items only after matching completed turn status; prefer
all final_answer items in observation order, or the last phase-less completed
item when no explicit final exists. Commentary and unfinished deltas cannot
produce success. Generated native schemas constrain consumed event structures;
the closed published profile constrains normalized packets and replies.

Thread start/resume use `config.model_reasoning_effort`; each turn also uses the
native effort field. Settings map runtime model/effort confirmation to public
options. Thread-only settings notifications describe configured settings only
within the dedicated active turn; turn-bound model reroutes remain separately
reported and grant no selection authority. Usage binds the exact thread/turn,
uses last.totalTokens without adding breakdowns, and never invents a denominator.
Missing/invalid reports remain unavailable; model switches/reset/compaction
invalidate earlier readings. Tests verify process requests and lifecycle but do
not prove live tool or network isolation.

## Task 4 provider-free verification checkpoint — 2026-10-05

The maintainer authorized this adapter unit after Task 3. The isolated branch
`codex-app-server-adapter` consumes the actual published profile and unmodified
stable CLI 0.160.0 schemas. The root coordinated plan records the exact local
commit. Source, tests, generated-schema provenance and dependency/license
evidence belong to this independent repository; no gateway code was copied.

The final provider-free suite has 21 passing tests: 15 external-process tests,
five catalog/usage tests and one close-cancellation regression. Reproducing
regressions addressed review findings for execution cancellation, generation
binding, model-change telemetry and cancelled close. Final independent review
has no remaining Critical or Important findings. Formatting, Clippy with
warnings denied, 139 vendored-file hashes, documentation and root governance
checks pass. Rust 1.93.1 was exercised; the declared 1.85 minimum was not directly
exercised. See docs/dependency-evidence.json for the inactive WASI dependency.

The test executable is synthetic and makes no inference calls. Explicit
no-tool/read-only/never-approve requests are verified at the process boundary;
no live isolation, end-to-end interoperability or production readiness is
claimed. Task 5 owns durable receipts, authenticated worker transport and
packaging. Provider access, publication and activation remain separate gates.

## Task 5 worker execution checkpoint — 2026-10-06

The authorized worker unit adds SQLite mapping/receipts and broker acknowledgements,
four process slots, scoped concurrency, authenticated A2A/card/control transport,
expiry/cancellation and independent pinned packaging. Its synthetic subprocess
and real-SQLite suite has 37 passing tests: the 21 adapter tests plus 12 receipt
and four HTTP tests. The root plan records the exact isolated local commit and
image identity. Rust/Cargo 1.93.1 was exercised; the declared 1.85 minimum was not.

Review regressions reproduced and fixed cancellation during a blocked completion
write, native replies duplicated as transcript text, missing native bindings and
policy rollback. Missing expected bindings and invalidating policy changes
persist invalidation and require explicit new. A terminal mutex is acquired after
SQLite writes and immediately before the final authority check/commit; cancellation
can win while persistence waits, and completion wins once final commit begins.
Processes can close after durable completion while broker acknowledgement is
pending; native history remains private and continuation stays blocked.

The admitted A2A server handler interface/native types are reused with an Axum
JSON-RPC wrapper that preserves canonical integer types. The stock protobuf
router coerces integers into floats and is unsuitable for exact profile packets.
This is a transport implementation detail, not a cross-repository contract change.

Formatting, Clippy, exact 139 profile/native files, 477 retained legal texts for
275 locked crates, documentation and root governance checks pass. Independent
review has no remaining Critical or Important findings. ARM64 image build and
network-disabled/read-only package check verify the pinned Codex release, schema
and UID/GID 10003; unconfigured service startup is rejected. x86_64 CI is defined
but has not been executed locally. No provider inference, deployment activation,
merge or publication occurred; Tasks 6–9 own the remaining integration gates.

## Task 6 approved-profile transport correction — 2026-10-06

The approved contracts profile requires a closed A2A input DataPart envelope
`{profileVersion, packet: TaskInput}` and completed artifact
`{reply: InternalReply, runtimeBinding: {threadId, turnId}}`. Matching task/context
IDs derive from the inner packet. Bare packets, extra envelope fields and wrong
profiles fail before inference. Worker.execute continues to return InternalReply.
The authenticated receipt projection adds runtimeBinding from committed native
thread/turn columns only for completed receipts, otherwise null. GetTask and
CancelTask replay the same bound completed artifact. The mediator compares this
private binding with its durable worker receipt; only InternalReply reaches the
broker/browser. Native identifiers are worker/mediator-private; no native session
files or credentials are returned. Provider-key echoes in identifiers are rejected
before persisting those identifiers. This corrects Task 5 transport implementation
to the already approved profile without adding routes or provider activation.

The corrected transport source is committed locally at
`6a0c03548053d6ad21958a1c2f8c442ae777d35c` on
`codex-worker-transport-contract`. Independent read-only review found no blockers;
38 existing tests and six service tests in an independent regression copy pass.
The rebuilt ARM64 image is
`sha256:c3951b12e58154f289c6554fe27768c315001c48b1dfdec13791423736966eae`.
Its pinned CLI/archive/schema checks and network-none/read-only package check
pass. Unconfigured startup exits 1 before inference. Original Task 5 source and
image remain historical checkpoints; no provider use, activation or publication
was performed. Detailed package evidence is in `docs/image-evidence.json`.

Task 6 profile review also enforces `expiresAt <= leaseExpiresAt`, alongside the
authorization deadline and 180-second packet bound, before any app-server protocol
request. A still-live lease ending before the packet deadline is invalid input;
equality remains accepted. This implements the approved published wire annex.

The final lease-guard source is `9dfc90189526bdc63bd1be8c189da2b665f3387f`.
Independent scoped review and all 39 provider-free tests pass. The final rebuilt
ARM64 image is
`sha256:95145520f4249ac1e843c0f13a817e0c42cfc578735abef5fa7b760333596a1b`,
superseding the earlier Task 6 transport image above. Archive/version/both schema
checks and network-none/read-only package verification pass; unconfigured startup
exits 1 before inference. Detailed evidence preserves both earlier image identities.
No provider use, service activation or publication occurred.
