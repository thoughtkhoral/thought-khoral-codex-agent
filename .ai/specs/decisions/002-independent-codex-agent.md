# 002 — Independent headless Codex agent

## Status

Draft — proposed for maintainer review; does not authorize implementation.

## Context

A human needs to send a chat request to Codex, start a fresh conversation,
and continue an existing conversation with its previous turns available.
The human also needs to select the model and reasoning effort, see the current
settings, and see context-window usage when runtime telemetry supports it.
The approved A2A foundation admits only the deterministic Reference Agent.
Its fixed result validation, ten-second execution limit, and local-only agent
egress do not support a model-backed conversation.

## Proposed decision

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

Proposed default: one active conversation per human, room, and Codex Agent.
Ownership is distinct from delivery: the first version uses room-visible
requests and replies, consistent with existing external agent tasks. Shared
room sessions and targeted conversations require a separate delivery design.

Codex receives the invoking human's prompt and its own conversation history.
It does not automatically receive unrelated room history or old conversations.
This is a Codex-specific exception to Decision 007's full-history disclosure;
the Reference Agent still receives its existing authorized context packet.

The first version provides conversational assistance with a read-only sandbox
inside a dedicated container. Repository editing, arbitrary host mounts,
additional MCP servers, and interactive tool approvals are outside this scope.
Provider authentication is supplied locally; credentials are never committed.

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

## Sources

- [OpenAI non-interactive mode](https://learn.chatgpt.com/docs/non-interactive-mode)
- [Codex app-server](https://learn.chatgpt.com/docs/app-server)
- Local CLI inspected on 2026-10-02: `codex-cli 0.160.0`, `codex exec --help`,
  `codex exec resume --help`, and generated app-server JSON schemas. The schema
  exposes supported reasoning efforts and nullable `modelContextWindow` with
  `last` and `total` usage breakdowns. Live runtime behavior still requires
  verification.
