# Codex worker component status

## Local runtime correction — 2026-11-23

The approved [Task 9 native startup correction](../.ai/specs/how/codex-chat-agent.md#approved-task-9-native-startup-compatibility--2026-11-23)
accepts optional pinned integer notification timestamps and only the exact
reviewed bundled-bubblewrap warning and disabled remote-control state. Catalog
and turn processing use the same schema-validated disposition; other warnings,
extra fields, requests and unsupported remote states fail closed. Diagnostics
and installation identities stay private. CLI, contract pins and tool controls
remain fixed.

Synthetic adapter tests cover these paths. Actual network-none production-image
adapter catalog, fresh package/policy checks, publication, changed-input
activation and Task 9 acceptance remain separate root execution gates. Earlier
native acceptance below remains bound to its original tested sources.

## Historical workspace context — 2026-11-20

Native AMD64/ARM64 worker acceptance remains tied to tested source `9bcdbde` and its final lock. The 2026-11-20 provider-free ARM64 image from source `3f0318b` passed ten package/startup probes and exact native policy recapture: 44 model/effort cases, eight models, resume and six refusals. Published contract v1.1 and locally prepared released consumers/profile do not add provider, assembled-stack, browser, registry-image or production acceptance. Worker/mediator v1.0 pins remain compatible and unchanged.

Runtime source and contract pins are unchanged. Exact clean pinned-source
snapshots were retained before this documentation-only checkout advanced.

Derived from the [local specification index](../.ai/specs/README.md) and
[root workspace authority](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/workspace-state.md).
The cited tests retain their original source identities; this documentation
refresh adds no activation, publication or runtime acceptance.

**Status as of 2026-11-20: experimental provider-free POC.** This page covers
the Codex worker owned by this repository. For the end-to-end goal, project
responsibilities, evidence limits, remaining milestone-one gates, and the later
specification/memory-guided milestone, read the [cross-project ThoughtKhoral
status guide](https://github.com/thoughtkhoral/thought-khoral/blob/main/docs/codex-conversation-status.md).
Approved What/How specifications and the [coordinated implementation
plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md)
govern requirements and execution.

## Current component

The independent worker includes a provider-free Codex app-server adapter,
authenticated gateway transport, durable execution receipts, and native
thread/session handling. It does not access room storage directly. Provider
authentication and service activation are separately configured and gated.

Provider-free worker and package verification is recorded in this repository's
[runtime specification](../.ai/specs/how/codex-chat-agent.md) and the coordinated
plan. The approved [native Linux acceptance](../.ai/specs/how/linux-worker-acceptance.md)
now records 46 tests per AMD64/ARM64 Rust 1.85.0/1.93.1 cell and both actual
final-lock package/policy-capture jobs. Downloaded target proofs match committed
bytes, and cleanup reports zero owned containers. See [retained evidence](evidence/linux-worker-acceptance-44599536091.json)
and the [acceptance guide](linux-acceptance.md) for exact source/run identities.

The final-lock native portability/package-policy gates are closed. This evidence
does not establish a live provider conversation, full Codex packaged room-stack,
rendered-browser behavior, contract/image release or production readiness.

## Future guided workspace

The [guided-workspace What](../.ai/specs/what/guided-workspace.md) remains a
draft future direction. It proposes reviewed immutable guidance bundles and a
separately governed memory-engine interface; it does not authorize filesystem
access or memory integration today.
