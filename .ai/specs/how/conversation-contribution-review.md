# Codex conversation contribution review

## Status and authority

Prepared on 2026-10-05 following the user's instruction to proceed with the
[coordinated implementation plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md).
The maintainer explicitly approved publishing the six issues, their scope, and
the coordinated specifications/plan on 2026-10-05. The accepted issue links below
record that authorization; no schema release or runtime implementation is claimed.

Read-only GitHub checks on 2026-10-05 returned no issues in the root, Codex agent,
or contracts repositories. The authenticated GitHub identity is RichNasz;
authentication alone does not establish maintainer acceptance.

Root [Decision 004](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/004-contribution-and-change-management.md)
requires accepted issues and approved specifications before implementation.
The following six complete issue drafts implement the root
[contribution brief](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-conversation-issue-brief.md).
Specification paths below identify current workspace drafts; publication to the
repositories' default branches has not been verified. Actual published issue URLs and explicit maintainer acceptance are recorded
below. The issue body drafts are preserved as the review record.

## 1. Owning feature and independent worker

Repository: `thoughtkhoral/thought-khoral-codex-agent`

Title: **Add an explicitly addressed Codex room participant with authorized history**

### Issue body

Humans should be able to address Codex in a ThoughtKhoral room and receive a
reply informed by the authorized public conversation, including discussion
exchanged while Codex was silent. Use one shared conversation per room and
admitted agent, with an explicit new-thread control that reseeds authorized room
history. Direct typed addressing invokes one turn; ordinary chat does not.

Keep the worker independent. It owns pinned Codex app-server processes, native
session files, and durable SQLite receipts. The room gateway owns authorization,
frozen context and public replies; the mediator owns authenticated invocation.
Uncertain provider submission fails interrupted and requires explicit new;
completed receipts replay stored output without another provider call.

Governing drafts: `.ai/specs/what/codex-chat-agent.md`,
`.ai/specs/how/codex-chat-agent.md`,
`.ai/specs/how/conversation-integration-profile.md`, and Decision 002.
Coordinating sources: root Decision 009 and
`.ai/specs/how/codex-room-conversations-implementation-plan.md`.

Acceptance: root What criteria and plan Tasks 4, 5 and 9, including provider-free
process/receipt tests, bounded output, restart recovery, model/effort confirmation,
and verified tool isolation before activation. Link the five contribution issues
below before recording implementation readiness.

Excluded: autonomous participation, targeted/private conversations, guidance
bundles or memory writes, arbitrary agent admission, and production deployment.

## 2. Contracts

Repository: `thoughtkhoral/thought-khoral-contracts`

Title: **Define and verify the separate v1 agent conversation contract**

### Issue body

Provide language-neutral closed JSON Schema and compatibility fixtures for
`thought-khoral.agent-conversation.v1`. The separate HTTP profile coordinates
turn requests, shared conversation state, catalogs, frozen input, updates,
normalized results and acknowledgements. Public prompts and replies remain
ordinary retained room messages.

Governing drafts: `.ai/specs/what/conversation-profile.md`,
`.ai/specs/how/agent-conversation-profile.md`, and Decision 003.
Implementation: root conversation plan Task 1. Owning feature issue: pending.

Acceptance: nine schema entry points; positive/negative fixtures; semantic
identity, binding, UTF-8 and canonical-digest checks; literal independently
derived digest vectors; unchanged retained fixtures passing. Consumers pin an
actual immutable release commit and archive digest. Tagging/publication needs
separate authorization after review and verification.

## 3. Room gateway

Repository: `thoughtkhoral/thought-khoral-room-gateway`

Title: **Reserve Codex turns atomically and disclose authorized room history**

### Issue body

Implement the separate conversation HTTP/workload API, shared generation/task
storage, atomic prompt reservation, complete public baseline and ordered deltas,
and atomic ordinary-message reply commit. Exclude targeted content; enforce
policy, requester expiry, lease, generation and digest binding on every result.
Idempotent requests persist one prompt/task; accepted replay persists one reply.

Governing drafts: `.ai/specs/{what,how}/codex-room-participation.md`.
Implementation: root conversation plan Tasks 2 and 3. Owning feature and released
contract references: pending.

Acceptance: real PostgreSQL transaction/race tests, exact context vectors,
authorization failures, idempotent terminal acknowledgement, no native IDs in
browser/broker projections, and unchanged retained room/Reference Agent behavior.
Browser Leave remains disconnection under the current authorization model.

## 4. Agent gateway

Repository: `thoughtkhoral/thought-khoral-agent-gateway`

Title: **Mediate pinned Codex conversations and reconcile execution receipts**

### Issue body

Add a separate Codex dispatcher with pinned identity/card/endpoint/capabilities,
authenticated catalog and receipt mediation, authority polling and cancellation.
Validate normalized results against the frozen platform task and private runtime
binding. Reconcile worker and broker receipts before transport retry; uncertain
running execution must never trigger another provider turn.

Governing drafts: `.ai/specs/{what,how}/codex-mediation.md`.
Implementation: root conversation plan Task 6. Owning feature and released
contract references: pending.

Acceptance: real local fake-worker/broker transport tests for lost acknowledgement,
expiry, late output, binding/citation failures and admission drift. Keep the
deterministic Reference Agent validator, loopback pin and ten-second budget.
Provider credentials remain outside both gateways.

## 5. Workspace UI

Repository: `thoughtkhoral/thought-khoral-workspace-ui`

Title: **Add explicit Codex addressing and shared conversation controls**

### Issue body

Expose explicit Codex selection/direct mention, shared new/continue controls,
model/effort selection, server-confirmed settings and last-request context usage.
Send one conversation HTTP request per invocation. Render assistant text once
from the ordinary room transcript; profile task state provides status metadata.

Governing drafts: `.ai/specs/{what,how}/codex-room-participation.md`.
Implementation: root conversation plan Task 7. Owning feature and released
contract references: pending.

Acceptance: accessible interaction tests for two humans sharing a session,
reload, busy draft preservation, model/effort changes, stale/out-of-order polling,
alias/quoted mentions, targeted rejection, and one reply bubble. Tokens stay in
the Authorization header; polling stops on terminal state, Leave or auth loss.

## 6. Platform

Repository: `thoughtkhoral/thought-khoral-platform`

Title: **Package opt-in Codex service with restricted provider egress**

### Issue body

Provide an opt-in Compose overlay for the independent worker, persistent native
and SQLite state, distinct invocation/provider secrets, and default-deny provider
egress. Pin reviewed image/source/CLI revisions and verify non-root, read-only,
no host port or host configuration, disabled tools and state ownership.

Governing drafts: `.ai/specs/{what,how}/codex-local-service.md`.
Implementation: root conversation plan Tasks 8 and 9. Owning feature and reviewed
worker/contract release references: pending.

Acceptance: base deployment works without Codex credentials; opt-in failures
close safely; controlled-listener egress tests reject direct IPv4/IPv6 and
arbitrary CONNECT destinations. Provider-free integration demonstrates history,
restart and replay behavior. Separately authorized live verification records
actual pins and isolation evidence before activation. Kubernetes/production
rollout is excluded.

## Approval and execution record

- User instruction to proceed: 2026-10-05.
- Permission to publish these six public issue bodies: granted on 2026-10-05.
- Maintainer acceptance of owning/contribution issues: recorded in each issue body on 2026-10-05.
- Explicit coordinated specification/plan approval: recorded in governing sources on 2026-10-05.
- First executable unit after those gates: contracts Task 1; no live inference.
- Release/tag, provider use and service activation: separate later gates.

## Published accepted issues

- [thought-khoral-codex-agent](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1)
- [thought-khoral-contracts](https://github.com/thoughtkhoral/thought-khoral-contracts/issues/1)
- [thought-khoral-room-gateway](https://github.com/thoughtkhoral/thought-khoral-room-gateway/issues/1)
- [thought-khoral-agent-gateway](https://github.com/thoughtkhoral/thought-khoral-agent-gateway/issues/1)
- [thought-khoral-workspace-ui](https://github.com/thoughtkhoral/thought-khoral-workspace-ui/issues/1)
- [thought-khoral-platform](https://github.com/thoughtkhoral/thought-khoral-platform/issues/1)

## Contract artifact preparation

Contracts Task 1 artifact preparation is verified on 2026-10-05: nine schema
entry points, 125 named fixtures, both fixture suites passing, retained schemas
and fixtures unchanged, independent review with no outstanding findings, and
zero reported npm audit vulnerabilities after the compatible fast-uri patch.

The isolated `codex-conversation-v1` branch resolves to `85baf86e574276fcd036e53e23641af6aad602f9`.
The root plan records the prepared TAR digest and verification evidence.
Publication was separately authorized on 2026-10-05. The [thought-khoral-agent-conversation-v1.0.0](https://github.com/thoughtkhoral/thought-khoral-contracts/releases/tag/thought-khoral-agent-conversation-v1.0.0)
release is published; its commit and downloaded archive digest were verified.
Contracts Task 1 is complete. Tasks 2 onward consume that actual artifact and
follow their separate implementation checkpoints; no Codex runtime is enabled.

## Task 2 gateway storage checkpoint

The maintainer authorized Task 2 on 2026-10-05 after contract publication.
Room-gateway storage/validation is committed locally at
`4f4d014194a7887c06722215181fd7451ca50f55` on `codex-conversation-storage`.
The [coordinated plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md)
records its exact source and verification evidence: 121 passing gateway tests,
one existing live test ignored, 34 passing final focused checks, formatting,
Clippy, contract hashes and root governance gates. Independent review findings
were corrected with reproducing regressions; no blocking findings remain.

The unit adds the independent contract pin, opt-in policy, strict turn validation,
and atomic prompt/reservation with frozen public context. Task 3 supplies the
profile routes, claims and accepted-result transitions. The independent Codex
worker and live activation remain later tasks; this repository is still a
runtime scaffold. Guided workspace/memory remains the separate second milestone.

## Task 3 gateway route checkpoint

The maintainer authorized Task 3 on 2026-10-05. The gateway implementation is
committed locally at `53d8fb78e27b2d9f71f99ae47fd013008d131dd7` on `codex-conversation-routes`, extending the
Task 2 storage checkpoint. The
[coordinated plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md)
and gateway [How](https://github.com/thoughtkhoral/thought-khoral-room-gateway/blob/main/.ai/specs/how/codex-room-participation.md)
record 142 passing tests, one existing live test ignored, literal digest vectors,
18 synthetic HTTP/database tests, formatting, Clippy and governance verification.
Independent review has no remaining blocking findings.

The unit supplies bounded public context, authenticated profile routes,
workload claims/authority/receipt recovery and atomic ordinary-message result
commit. Policy remains disabled by default; the executable has no catalog
bridge. A maximum nonterminal ordinal clarification is recorded for the
published profile before activation. No provider access, deployment or
publication occurred. Task 4 is the independent Codex app-server adapter;
this Codex repository remains a runtime scaffold. Guided workspace/memory is
still the separate second milestone.

## Task 4 independent adapter checkpoint

The maintainer authorized Task 4 on 2026-10-05. The independent adapter library,
its published-profile pin, stable CLI 0.160.0 schema pin and deterministic
external-process tests are prepared on `codex-app-server-adapter`. The root
[coordinated plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md)
records its exact local commit and verification evidence. The local
[runtime design](codex-chat-agent.md) records 21 passing tests and review fixes.

The branch remains isolated and unpublished. A2A/SQLite transport, native receipt
reconciliation, packaging and live isolation remain later tasks. The repository
now contains a provider-free adapter library rather than only a scaffold; no
Codex service is enabled. Guided workspace/memory remains milestone two.

## Task 5 durable worker checkpoint

The maintainer authorized the next task after the adapter checkpoint. On
2026-10-06 the isolated `codex-worker-receipts` branch implements the worker-owned
SQLite/A2A/package unit under issue 1. The root coordinated plan records its
exact commit/image identity, 37 provider-free tests and independent review with
no remaining Critical or Important findings. Branch/worktree remain local and
unpublished. Mediation, UI, opt-in isolation and end-to-end verification remain
Tasks 6–9. Guided workspace/memory remains milestone two.
