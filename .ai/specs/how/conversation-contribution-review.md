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

Task 4 local source revision: `e61a5f79568ce24e411f559efc5290000638395a`; implementation remains in the isolated
`codex-app-server-adapter` branch and has not been merged into this checkout.

## Task 5 durable worker checkpoint

The maintainer authorized the next task after the adapter checkpoint. On
2026-10-06 the isolated `codex-worker-receipts` branch implements the worker-owned
SQLite/A2A/package unit under issue 1. The root coordinated plan records its
exact commit/image identity, 37 provider-free tests and independent review with
no remaining Critical or Important findings. Branch/worktree remain local and
unpublished. Mediation, UI, opt-in isolation and end-to-end verification remain
Tasks 6–9. Guided workspace/memory remains milestone two.

Task 5 local source revision: `b23cf7cd002270de16a7b572a0e810e2fd9ff15d`; source remains in the isolated
`codex-worker-receipts` branch and has not been merged into this checkout.

## Task 6 mediation checkpoint

Gateway mediation is committed locally at `1900f8d127d744ffb996021fdc2f3fb34858fbda` on
`codex-conversation-mediation`. The broker catalog adapter is committed at
`50d293491ae65250e0603d26645d4bcc4e692b90` on `codex-broker-catalog-bridge`, extending Task 3
`53d8fb78e27b2d9f71f99ae47fd013008d131dd7`. Corrected worker source is
`9dfc90189526bdc63bd1be8c189da2b665f3387f`, with image evidence at
`bd4d70c4d0ddf3940eb0efabcfb79507c1ac9069` on `codex-worker-transport-contract`. These independent worktrees
remain local, unmerged and unpublished; original source checkouts are preserved.

- Gateway: 66 provider-free tests pass, including all 44 deterministic baseline
  tests. Exact card/profile/endpoint admission, authenticated bounded controls,
  integer-preserving A2A packet envelopes, durable private transport records,
  authority polling/cancellation, artifact-to-receipt native binding comparison,
  prior disclosed citation IDs and normalized result/settings/usage validation
  have real local HTTP fixture coverage. Native IDs never enter broker updates.
- Completed or uncertain worker receipts never cause another provider turn.
  Both lost committed acknowledgement and uncommitted terminal POST recover
  from exact stored output. Verified failed/revoked/expired recovery is durably
  quarantined; transient per-record failures do not starve unrelated rooms.
- The production broker CatalogQuery connects to the fixed authenticated
  mediator catalog bridge at `thought-khoral-agent-gateway:9092`,
  `GET /internal/agent-conversations/v1/models`. Its separate bridge credential
  is distinct from worker invocation and workload credentials. The adapter
  locally paginates the bounded catalog using content/revision-bound cursors;
  browser cursors remain broker-scoped. Both services remain disabled by default.
- Worker: 39 tests pass. Its closed `{profileVersion, packet}` input and private
  `{reply, runtimeBinding}` completed artifact now conform to the existing
  approved profile. Completed-only receipt bindings come from SQLite. The
  deadline cannot exceed its lease, with rejection before any runtime request.
  The rebuilt ARM64 image is
  `sha256:95145520f4249ac1e843c0f13a817e0c42cfc578735abef5fa7b760333596a1b`.
  Archive/CLI/both schema checks and network-none/read-only package verification
  pass; UID/GID is 10003 and unconfigured startup exits 1 before inference.
- Formatting, Clippy with warnings denied, contract pins, documentation links,
  whole-change whitespace and root specification/reference/identity gates pass.
  The mediator preserves the existing 299-package lock graph; broker's eight
  added exact HTTP dependency archives/checksums/licenses were independently
  reviewed. Worker retains 139 protocol files and 477 legal-file hashes.
  Identity exceptions add only the two exact published mediator fixture fields,
  with a RED/GREEN regression rejecting other legacy occurrences.
- Independent reviews reproduced and corrected terminal-recovery starvation,
  lease inequality, catalog pagination and null-window freshness validation.
  Final scoped review has no remaining Critical or Important findings.

Broker full offline all-target suite: 148 passed, one existing ignored. Detailed
review evidence is recorded in its local How and independent Task 6 packet. An existing live Reference Agent test
remains ignored; no live coverage is inferred from that omission. All runtime
execution here used synthetic fixtures. x86_64 packaging, Rust 1.85 execution,
provider inference, tool/egress isolation, activation and full-stack verification
remain later gates. Task 7 is the room UI; Tasks 8–9 own opt-in packaging and
end-to-end evidence. Guided workspace/memory remains milestone two.


## Task 7 UI checkpoint

Workspace UI `e4afe0562306d7996f1ed232bc7499ee64d6bc1c` on `codex-room-conversation-ui` passed independent review.
The [owning How](https://github.com/thoughtkhoral/thought-khoral-workspace-ui/blob/main/.ai/specs/how/codex-room-participation.md)
and coordinated plan record provider-free controls, restoration, transcript and
verification evidence. Changes remain local and unmerged; platform activation
and full-stack/live evidence remain Tasks 8–9.

## Task 8 local packaging checkpoint — 2026-10-07

Platform `9626bc46f8b74c2a58b2578c4f744996f3d41317` on `codex-opt-in-platform` and worker `d40e4a8cd5efc77c7161742aec7ade289efb357a` on
`codex-worker-tool-policy` passed independent task review. The coordinated plan
records immutable image identities and synthetic kernel/TLS/startup/native-tool
capture evidence. Feature flags alone were insufficient for initial tool
restriction; the corrected pinned worker binds sanitized catalog, controls and
actual CLI evidence. Linux ARM64 is verified locally; x86_64 remains blocked on
equivalent capture. Cold bootstrap admission expires within five seconds after
health renewal stops; no instantaneous withdrawal from already-open UI is claimed.
Original checkouts retain their runtimes/scaffold. No provider inference,
activation, merge or publication occurred. Task 9 retains end-to-end integration
and separately authorized live verification; directory guidance remains later.

## Task 9 synthetic verification and correction checkpoint — 2026-10-07

Provider-free checkpoint only; Task 9 and the milestone remain open.

Task-scoped verification review: Approved. Broad implementation review: Partial
spec compliance; quality Needs follow-up. B1–B3 (pending-ack recovery, omitted
shared settings and receipt-correlated safe failures) are addressed. B4 is
partial: explicit initial/reset selection works for full-capability admission,
but automatic server-default display requires an approved interface amendment.
F1: unresolved Important reasoning-only UI deadlock. Optional capabilities are
independent; an effort-only admission cannot establish the guard-required model
through its hidden selector. This prevents initial/reset invocation and blocks
whole-milestone/merge readiness. No second broad fix wave or waiver is implied.

| Owner | Final reviewed local revision |
|---|---|
| contracts | `85baf86e574276fcd036e53e23641af6aad602f9` |
| broker | `fd05cb48b8508e7939f9cdf9df275742a06fc4f8` |
| mediator | `6c3d96b4763871b9addc9bc7223e71ee7d38abd9` |
| worker | `b0d43ec2b5b0c8da035d4ccff754545132b978d4` |
| ui | `e51d67e9e1a986601df6b5e1acf68aaf7ae0870d` |
| platform | `637a69279f0fe5019560b1e54d28f48c1c715897` |

All six reviewed worktrees were clean when this checkpoint was prepared.
Runtime is committed only on isolated local branches; originals retain their
runtime/scaffold and unrelated edits. Contracts v1.0.0 and dependency lockfiles
remain unchanged.

Controller final verification on platform revision above: `node
scripts/smoke-codex-conversation.mjs --fake` (session85023) exit0, six original
crash/commit boundaries, 11 fake native turns, exact baseline/delta/source IDs,
targeted/cross-room exclusion, duplicate=one logical turn, worker restart and
fresh reset, shared omitted settings, rejected-completion recovery and exact
execution_failed/session_unavailable/runtime_unavailable projections. `node
--test scripts/tests/codex-conversation-smoke.test.mjs` (session23304) exit0,
13 passed, zero failed. This is synthetic native/identity/private-DNS adapter
coverage, not whole packaged Compose, real Keycloak/browser or provider proof.

Inspected owner logs and independent review record broker150 passed + one
pre-existing ignored live test; mediator65 passed, zero failed/ignored (correcting
the earlier reported68); worker46; UI121 + pin/tamper checks and production
build. Owner fixture tests3, actual assertion-failure/SIGTERM/SIGINT cleanup3,
package/startup checks10 passed. Earlier contracts/regression/legal/pin evidence
is retained with original attribution, not presented as rerun here.

New ARM64 worker image:
`sha256:2d8bfade27802f910cf68e832722c93b4a2acc2addb825711e1223617a4cd385`.
Compiled runtime revision `b418a76e0e7ca047b5fe995eb17519aced369a06`; worker
head above adds evidence documentation. Immutable image readiness checks used
network-none/read-only/cap-drop-all, both admission markers; default invocation
refused as expected. Actual native 44-setting/eight-model/resume/six unsolicited
tool refusal evidence remains attributed to its earlier source/image, not this
new image. CLI/catalog/control hashes are unchanged. x86_64 native admission and
Rust1.85 minimum-version checks remain unrun.

Default discovery: pending specification approval. The local proposed How is
`thought-khoral-codex-agent/.ai/specs/how/default-settings-discovery-proposal.md`.
It proposes a read-only authenticated defaults query in a new immutable v1.1.0
artifact and independent mixed-capability controls, covering absent conversation
and explicit New/reset. It authorizes no runtime or published contract changes.
Live provider verification: pending. Account/model availability, actual native
history, live tool/egress/key isolation, packaged deployment/private DNS and real
browser/identity evidence remain separately gated. No merge, push, publication,
service activation or provider inference occurred.

Independent review artifacts are retained outside Git at
`/private/tmp/codex-conversation-task9/final-fix-review.md`,
`final-implementation-review.md`, and `task9-fix-review.md`; owner evidence at
`/private/tmp/Task9-final-fix-evidence/`. Final root/documentation/source-reference
and identity gate results will be recorded in the controller checkpoint after
these source-derived record updates. The aggregate release checklist remains
unchecked; passing synthetic checks do not resolve F1 or default discovery.

## Defaults discovery local synthetic checkpoint — 2026-10-07

All four defaults-amendment tasks passed their independent reviews. The final
whole-branch review passed. F1 (initial/New reasoning-only settings deadlock) and
visible defaults discovery are accepted for this local synthetic candidate.

| Source | Exact local revision | Retained worktree |
| --- | --- | --- |
| contracts | `1ea828f28725ddaaefa21d083473f9abbd777975` | `/private/tmp/codex-conversation-defaults/contracts` |
| broker | `2e7d23b467c572819f498c3b9bf14d74a62dc821` | `/private/tmp/codex-conversation-defaults/room-gateway` |
| mediator | `6c3d96b4763871b9addc9bc7223e71ee7d38abd9` | `/private/tmp/codex-conversation-final-fix/agent-gateway` |
| worker | `b0d43ec2b5b0c8da035d4ccff754545132b978d4` | `/private/tmp/codex-conversation-final-fix/worker` |
| ui | `79e5e7310a450efea561548cd87871446c1939aa` | `/private/tmp/codex-conversation-defaults/workspace-ui` |
| platform | `2d856773078a7caa542d719e539b55a5ab2dafaa` | `/private/tmp/codex-conversation-defaults/platform` |

The composed run was executed at `f9afeb20b746200daa9cdef0406c03f88a422b68`.
The subsequent path-provenance correction was tested and scoped-reviewed at
`220f6e0c74a29c000d7de81c0cb77823de0bd15c`;
the final platform revision above adds completion metadata only. The original
repositories retain their runtime; local implementation branches remain unmerged.

The unreleased candidate contract source is
`1ea828f28725ddaaefa21d083473f9abbd777975`, proposed release
`thought-khoral-agent-conversation-v1.1.0`. Its archive SHA-256 is
`fab59a486f6498b843467202debcb0768403bd57ba7dda41be2a01e5f23fdda8`
and externally anchored lock SHA-256 is
`7914d32eae2487879a68405b5095a6b9aa91355f87529c43f4055844821902a9`.
All 156 candidate payload files match in broker/UI; published v1.0 bytes remain
unchanged. The profile and API namespace stay v1. This is not a published release.

Evidence: retained 125 contract fixtures plus 16 additive cases and 6 candidate
integrity tests; broker serial suite 158 passed with 1 existing live-only test
ignored; UI full suite 190 passed with 1 intentional composed skip, followed by
scoped harness/TypeScript checks; 26 source-pin and 13 retained runner guards;
5 fixture unit tests; 3 actual failure/SIGTERM/SIGINT cleanup cases. The composed
candidate passed 12 capability/lifecycle cases, 3 display/send mutation cases,
15 actual HTTP UI children with 90 test passes, and 24 synthetic native turns
(11 retained baseline plus 13 added). Stale/removed pairs allocate no task/event
or native turn, retain the prompt and require explicit Refresh. Default-only
changes preserve the displayed explicit pair; unavailable replay is immutable.
Paused-publisher regressions verified RED before and GREEN after atomic exclusive
handshake publication. Owned processes, containers and staging files were cleaned.

The current composed state is `/var/folders/70/5kxy5kys3bj0252chp3ck8900000gn/T/Task9-codex-conversation-j9py6w`. Full provenance,
task/thread bindings, raw log references, limitations and review reports remain in
`/private/tmp/codex-conversation-defaults/defaults-reviewed-checkpoint.json` and
`/private/tmp/codex-conversation-defaults/task-4-logs/`. Root hierarchy/reference/
identity, scaffold documentation and whitespace results are recorded separately
in `/private/tmp/codex-conversation-defaults/final-gates.json` after synchronization.
The old parallel broker fixture port collision and Vite chunk advisory are
retained limitations; no passing parallel broker-suite claim is made.

Publication: pending

Packaged-stack verification: pending

Live provider verification: pending

The composed gate uses jsdom, a synthetic room socket, actual conversation HTTP
and storage, and a fake native executable. It does not establish packaged Compose,
real browser/Keycloak, provider, architecture-minimum or new-image acceptance.
Task 9 and milestone aggregate gates remain open. Specification/memory-guided
working directories remain the separately scoped future extension.
