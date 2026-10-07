# Project status and next development

**Status as of 2026-10-07: experimental POC.** The Codex agent and reviewed
room-integration changes are pushed to their owning repositories' `main`
branches. This page is a reader's overview derived from the approved [What](../.ai/specs/what/codex-chat-agent.md), [How](../.ai/specs/how/codex-chat-agent.md), and [coordinated implementation plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md). Those specifications govern requirements and implementation details; this page summarizes their current status.

## Goal

Let a human explicitly address Codex in a ThoughtKhoral room and have it respond
using authorized room-wide conversation history, including relevant discussion
that happened while Codex was not responding. Authorized room members can
continue the shared Codex thread or start a fresh one without deleting the room
transcript. Codex runs as an independently operated agent behind the agent
gateway and does not access room storage directly.

## What is implemented

| Component | Current implementation |
| --- | --- |
| [Contracts](https://github.com/thoughtkhoral/thought-khoral-contracts) | Published v1.0.0 conversation profile remains the immutable consumer baseline. The additive v1.1 defaults candidate is on `main`, without a v1.1.0 release or tag. |
| [Codex agent](https://github.com/thoughtkhoral/thought-khoral-codex-agent) | Provider-free app-server adapter, durable worker receipts, authenticated transport, and native thread/session handling. Provider authentication is separate from room storage. |
| [Room gateway](https://github.com/thoughtkhoral/thought-khoral-room-gateway) | Room-scoped conversation reservation and storage, bounded authorized history, and authenticated defaults discovery. |
| [Agent gateway](https://github.com/thoughtkhoral/thought-khoral-agent-gateway) | Pinned Codex admission and mediation for catalog, receipt, and normalized conversation updates. |
| [Workspace UI](https://github.com/thoughtkhoral/thought-khoral-workspace-ui) | Explicit Codex selection/mention, shared start/continue/reset controls, visible model and effort settings, and context-usage status. |
| [Platform](https://github.com/thoughtkhoral/thought-khoral-platform) | Opt-in local service packaging and restricted provider egress configuration. |

The project has provider-free component tests and a synthetic composed smoke
recorded against reviewed source revisions in the coordinated plan. Platform
fixtures exercised dual-stack, DNS, TLS, and proxy/policy-loss isolation paths.
These results do not demonstrate a live provider conversation, deployed
identity and browser behavior, or production readiness. The published v1.0.0
contract artifact is unchanged.

## Remaining milestone-one work

Tasks 1–8 are implemented and pushed. Task 9's provider-free history, recovery,
and synthetic integration cases are recorded as complete. The following Task 9
gates remain open:

- Run the opt-in multi-human live conversation check: verify intervening room
  messages, worker restart and continuation, then an explicit fresh session.
- Verify model/effort changes, restored settings, denied-model behavior, and
  context telemetry against the configured runtime.
- Verify live tool and egress isolation and complete the final cross-project
  acceptance/evidence review.

An additional milestone-one portability gap remains: equivalent Linux x86_64
native tool-policy capture/admission and execution on the Rust 1.85 minimum
version are unverified.

These checks require their stated configuration and separate authorization for
live provider use. Until the gates are closed, the POC has no v1.1 contract
release/tag, packaged-stack acceptance, service activation, or live-provider
verification claim. The [Task 9 checklist and evidence](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md#task-9--verify-end-to-end-history-restart-isolation-and-release-evidence)
are authoritative for individual checks and revisions.

## Later milestone: specification- and memory-guided work

The [guided-workspace What](../.ai/specs/what/guided-workspace.md) is a draft
future direction, not an implementation task. It proposes reviewed immutable
guidance bundles, visible revisions, fresh native threads when guidance changes,
and source-grounded replies. Before implementation, the project still needs an
approved bundle format and publication/review process, provenance and retention
rules, an instruction-loading design for the pinned Codex version, and a
separately governed memory-engine interface. Automatic memory writes and
arbitrary host paths remain outside the approved milestone-one scope.
