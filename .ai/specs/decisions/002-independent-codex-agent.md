# 002 — Independent headless Codex agent

## Status

Accepted by the project maintainer on 2026-10-05 under [issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1).
Scope: milestone-one design and coordinated implementation plan, including
the room-wide history filter. Release publication and live activation retain
separate gates; acceptance does not claim implemented runtime behavior.

## Context

A human needs to send a chat request to Codex, start a fresh conversation,
and continue an existing conversation with its previous turns available.
The human also needs to select the model and reasoning effort, see the current
settings, and see context-window usage when runtime telemetry supports it.
The approved A2A foundation admits only the deterministic Reference Agent.
Its fixed result validation, ten-second execution limit, and local-only agent
egress do not support a model-backed conversation.

## Decision

Admit one locally operated Codex Agent through the agent gateway. A dedicated
A2A worker launches the pinned Codex CLI as a headless app-server. Existing
reference-agent admission, validation, and egress remain independently enforced.

Implement the worker in this independent repository,
`thought-khoral-codex-agent`. Bring-your-own-agent is the architectural direction:
an agent implementation can be developed, released, deployed, and configured
independently of the platform. The gateway remains the platform's admission,
authorization, protocol mediation, and policy boundary. Its implementation
does not link Codex-specific runtime code or depend on Codex session-file formats.

Use `codex app-server --listen stdio://` and its explicit `thread/start`,
`thread/resume`, and `turn/start` APIs. Discover models and supported efforts
through `model/list`, and receive `thread/tokenUsage/updated` notifications.
The added model, effort, and usage requirements make this preferable to the
original exec-only proposal. Do not inspect the operator's personal sessions
or run with ephemeral persistence.

The room gateway owns conversation identity, human authorization, turn
reservation, and durable room events. The Codex worker owns CLI subprocesses,
runtime thread IDs, and persistent Codex session files. The agent gateway
mediates authenticated invocation and normalized results. Codex has no direct
room database access or active-decision authority.

The agreed participation direction is a room participant that responds only
when explicitly addressed by a human. A canonical direct mention or explicit
composer selection invokes Codex; ordinary messages, aliases, quoted mentions,
and agent messages never invoke it. Inputs and replies use room-wide delivery.

Proposed session design: one active conversation per room and admitted agent,
shared by humans authorized to participate in that room. Requester identity
belongs to each task, rather than to a private session owner. Any human with
room chat and agent-invocation permission may continue or explicitly reset it.
The UI must disclose that reset affects the room's shared Codex conversation.
This session design was approved on 2026-10-05.

Codex receives the room gateway's full authorized room-wide transcript on a
fresh thread, then ordered updates on later turns, alongside native Codex
history. Targeted messages are excluded even when visible to the requester:
their contents must not leak through shared history or room-wide replies.
Room history remains authoritative; native history is a derived execution
record. Context includes provenance, a revision boundary, and the current
invoking message exactly once. Every turn rechecks authorization.

This replaces the previous prompt-only proposal. Root Decision 009 and the five affected child specification sets now mirror
this design; the approved plan supplies the remaining runtime dependency gates. Decision 007's full-authorized-history
baseline remains in force for the Reference Agent. The Codex profile narrows
that baseline to room-wide transcript material for shared sessions; its
delivery-filter override, scope, and consequences are recorded below.

The first version provides conversational assistance with a read-only sandbox
inside a dedicated container. Repository editing, arbitrary host mounts,
additional MCP servers, and interactive tool approvals are outside this scope.
Provider authentication is supplied locally; credentials are never committed.

The longer-term direction is an agent working directory containing approved
specifications, instructions, and curated memory. The first release uses fixed
instructions in an isolated directory. Future guidance uses an operator-reviewed,
versioned bundle bound to the room and conversation generation; chat cannot
choose host paths, install instructions, or write durable memory. See the
[guided workspace requirements](../what/guided-workspace.md).

Model and effort changes apply to the next accepted turn in the same thread.
Persist resolved settings and distinguish the next-turn selection from the
settings of an active turn. Show only efforts supported by the selected model.
Context usage is an explicitly labelled estimate based on the last runtime
usage report and its reported window, not cumulative token consumption.

## Alternatives

- `codex exec --json` with explicit resume IDs supports a smaller process-per-
  turn adapter, but does not establish the model catalog and context telemetry
  required for this UX without an additional interface.
- Reconstructing conversation history for every new execution adds a second
  memory implementation and loses native Codex session semantics.
- Private per-human sessions retain individual Codex exchanges but do not
  provide a common conversation across humans. The proposed room session
  better matches the agreed participant experience.
- Autonomous participation could respond to every relevant message, but
  requires separate trigger, loop-prevention, and attention-budget policies.
  Explicit addressing is the initial behavior.

## Consequences

Contracts, room gateway persistence and authorization, agent registration and
dispatch, chat UI, and local platform configuration need coordinated changes.
Session files and database mappings must both survive ordinary restarts.
Uncertain execution after a crash must not be replayed as a second Codex turn.

Agent conversation controls, optional model/effort selection, and optional usage
reporting are declared capabilities in a versioned integration contract. The
UI renders only capabilities admitted for the agent; non-Codex agents are not
required to implement Codex model names, effort levels, or context telemetry.
The agent owner manages provider credentials and runtime session storage. The
gateway holds only the scoped credential needed to invoke the admitted agent.

This proposal admits the first pinned Codex agent, not arbitrary user-supplied
URLs or open remote admission. General bring-your-own-agent onboarding still
needs an explicit design for enrollment, endpoint trust, workload identity,
tenant isolation, context disclosure, compatibility, and revocation. A separate
repository is a boundary for ownership, not proof of safe agent admission.

## Parent override

- Parent rule: root Decision 007 supplies full history authorized for the
  invoking human and registered agent, including visible targeted material.
- Replacement: this shared Codex conversation receives only authorized
  room-wide transcript entries and room-wide active decisions.
- Rationale: a later human can continue the native thread and replies are
  room-wide, so requester-specific visibility cannot safely define its memory.
- Scope: the admitted Codex `chat` capability only; Reference Agent behavior
  remains governed by Decision 007.
- Approval: accepted by the project maintainer on 2026-10-05 with root
  Decision 009 and the coordinated child specifications.
- Consequences: private requests are rejected; visibility-policy changes that
  invalidate previously disclosed history invalidate the thread. A fresh
  thread receives the newly authorized room-wide baseline.

## Sources

- [OpenAI non-interactive mode](https://learn.chatgpt.com/docs/non-interactive-mode)
- [Codex app-server](https://learn.chatgpt.com/docs/app-server)
- Local CLI inspected on 2026-10-02: `codex-cli 0.160.0`, `codex exec --help`,
  `codex exec resume --help`, and generated app-server JSON schemas. The schema
  exposes supported reasoning efforts and nullable `modelContextWindow` with
  `last` and `total` usage breakdowns. Live runtime behavior still requires
  verification.

## Coordinated execution boundary

The user authorized specification advancement on 2026-10-02 and approved the
coordinated specifications and plan with accepted issues on 2026-10-05. The
contracts-owned profile selects a separate versioned HTTP API; public requests
and replies remain ordinary room messages. The written plan and exact profile are approved with accepted issue
traceability; artifact publication remains the consumer execution gate.
Directory guidance remains the later milestone, with no filesystem tools or
memory writes enabled by this pass.
