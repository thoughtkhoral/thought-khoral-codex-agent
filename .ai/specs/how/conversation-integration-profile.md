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

## Approved defaults-discovery amendment — 2026-10-07

The maintainer approved the [visible server defaults design](https://github.com/thoughtkhoral/thought-khoral-codex-agent/blob/main/.ai/specs/how/default-settings-discovery-proposal.md) in
this conversation on 2026-10-07 after an explicit specification approval request.
It authorizes coordinated local implementation and synthetic verification of
the additive authenticated defaults query and independently optional model/effort
controls, including the F1 initial/reset effort-only deadlock. The accepted
design is the governing amendment to earlier default-visibility wording.

The contracts owner defines `ResolvedSettingsView` at
`GET /api/agent-conversations/v1/rooms/{roomId}/agents/{agentId}/defaults` in
new immutable artifact `thought-khoral-agent-conversation-v1.1.0`, retaining the
v1 profile/namespace and all existing published v1.0 schema/fixture bytes.
The broker validates authenticated room/agent authority, current admission,
catalog revision, policy-default pair and five-second bound before responding.
The read has no task/event/conversation/lease/native-state mutation, exposes no
effective-settings confirmation, credentials or private/native identifiers,
uses the existing safe ProfileError/HTTP mapping and `Cache-Control: no-store`.
There is no inferred catalog-order model or inference fallback.

The UI resolves and displays the concrete explicit next-turn pair when absent
or explicitly New/reset; restored continuation uses accepted shared settings.
Both capabilities allow both controls; effort-only keeps the resolved model
read-only; model-only keeps the displayed model-specific catalog default effort
read-only; neither capability retains the settings-free path. Unsupported
controls stay uneditable and no hidden control blocks a valid required choice.
Catalog/pair mismatch requires bounded refresh or an explicit unavailable state.
A still-valid explicit pair is not replaced after a deployment-default-only change.

As a scoped exception to the earlier published-artifact-first execution order,
isolated consumers may pin a reproducible local candidate from an exact committed
contracts revision, verified archive and per-file SHA-256 values, clearly marked
unreleased. This exception is only for this amendment's local pre-publication
development and synthetic testing. Published v1.0 provenance/bytes remain intact.
No release publication, shipped interoperability, merge, push, provider use or
service activation is authorized. Whole milestone/Task9 acceptance remains open.

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
