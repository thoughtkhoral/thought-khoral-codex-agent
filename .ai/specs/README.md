# ThoughtKhoral Codex agent specifications

## Current workspace state — 2026-11-20

Native AMD64/ARM64 worker acceptance remains tied to tested source `9bcdbde` and its final lock. The 2026-11-20 provider-free ARM64 image from source `3f0318b` passed ten package/startup probes and exact native policy recapture: 44 model/effort cases, eight models, resume and six refusals. Published contract v1.1 and locally prepared released consumers/profile do not add provider, assembled-stack, browser, registry-image or production acceptance. Worker/mediator v1.0 pins remain compatible and unchanged.

Runtime source and contract pins are unchanged. Exact clean pinned-source
snapshots were retained before this documentation-only checkout advanced.

See the [root workspace authority](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/workspace-state.md) for exact source/release
identities, preserved source snapshots and remaining gates. Documentation-only
HEADs are not substitutions for frozen acceptance commits. Earlier dated
checkpoints retain their historical evidence. Required worker host checks were
run separately during this refresh; they do not extend native/package/live
acceptance or change runtime bytes.

Specifications are the source of truth for implementation and documentation.
Parent requirements in the [root specification index](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/README.md)
apply here. Overrides require an accepted local decision identifying the parent
rule, replacement, rationale, scope, approval, and consequences.

The repository foundation is approved by the user's setup request on
2026-10-02. Milestone-one runtime specifications and the coordinated plan were approved
on 2026-10-05 under [issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1). The provider-free Task 4 adapter and Task 5 worker transport, receipts and
packaging are merged into this repository's local `main` from their reviewed
branches; this repository now contains the provider-free agent/worker runtime.
The reviewed broker, mediator, UI, and opt-in platform candidates are merged
into their owning repositories' local `main` branches under explicit user
authorization on 2026-10-07. Exact source and merge commits are recorded in the
[coordinated plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md).
The integration was pushed to the owning GitHub `main` branches on 2026-10-07.
The Task 8 pinned-worker tool-policy correction and Task 9 worker recovery fix are
in this repository's pushed history. The F1 correction and visible defaults
discovery passed provider-free composed verification. The v1.1 contract release
artifact is now published; packaged-stack and separately authorized live
verification remain pending. This remains an experimental POC, not a production
readiness claim.

For the current goal, project ownership, evidence limits, and remaining
Task 9 gates across the workspace, see the [shared cross-project status guide](https://github.com/thoughtkhoral/thought-khoral/blob/main/docs/codex-conversation-status.md).
For this repository's worker implementation, see the [Codex component status
guide](../../docs/project-status.md).

## Approved foundation

- [What: repository foundation](what/repository-foundation.md)
- [What: public documentation](what/public-documentation.md)
- [How: repository foundation and documentation checks](how/repository-foundation.md)
- [Decision 001: governance, identity, and license](decisions/001-governance-and-license.md)

## Approved Linux acceptance extension

The [native Linux acceptance How](how/linux-worker-acceptance.md) was approved
by the maintainer on 2026-11-19 for local CI/helper implementation and offline
verification. It defines native AMD64/ARM64 Rust 1.85.0/1.93.1 tests, final-lock
image admission and isolated actual-CLI recapture. The maintainer subsequently
authorized publication and CI execution: all six native jobs passed at `9bcdbde`.
The How retains exact run/artifact evidence and scope. Local activation and
packaged-stack/browser/live acceptance remain separate gates.

## Approved milestone-one runtime

- [Rust 1.85 dependency-lock compatibility](how/rust-185-lock-compatibility-proposal.md)
  was approved on 2026-11-19 for the tested transitive pin, retained legal
  inventory refresh, and final-source verification. Later final-lock Linux
  minimum-version and package evidence is retained in the native acceptance How.
- [Architecture-specific package evidence](how/x86-package-evidence-proposal.md)
  was approved on 2026-11-19 for local implementation and verification. Later
  publication/native x86_64 CI passed under the Linux acceptance How; live and
  packaged room-stack activation remain separate gates.
- [What: Codex chat conversations](what/codex-chat-agent.md)
- [How: headless Codex chat agent](how/codex-chat-agent.md)
- [How: room conversation integration profile](how/conversation-integration-profile.md)
- [Decision 002: independent Codex agent boundary](decisions/002-independent-codex-agent.md)
- [How: visible server defaults](how/default-settings-discovery-proposal.md)
  was approved on 2026-10-07 for coordinated local implementation and synthetic
  verification; publication and live activation retain separate gates.
  Its [four-task execution plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-default-settings-implementation-plan.md)
  covers the candidate contract, broker, UI and composed acceptance evidence.

The approved design makes Codex an explicitly addressed room participant with
authorized room-wide history. Contracts Task 1 has published [thought-khoral-agent-conversation-v1.0.0](https://github.com/thoughtkhoral/thought-khoral-contracts/releases/tag/thought-khoral-agent-conversation-v1.0.0).
Consumer tasks must vendor its verified immutable artifact before implementation.

## Future guidance

- [What: specification and memory guided workspace](what/guided-workspace.md)

Directory guidance is a later extension. The first release retains fixed
instructions in an isolated working directory; it does not require this extension.

The [coordinated implementation plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md)
records repository ownership, dependency order, tests, and execution gates.

The [contribution review packet](how/conversation-contribution-review.md) records
accepted public issue links and the 2026-10-05 maintainer approval.

[Documentation](../../docs/README.md) remains derived context rather than an
independent source of product requirements.
