# Documentation workflow

## Current workspace context — 2026-11-20

Native AMD64/ARM64 worker acceptance remains tied to tested source `9bcdbde` and its final lock. The 2026-11-20 provider-free ARM64 image from source `3f0318b` passed ten package/startup probes and exact native policy recapture: 44 model/effort cases, eight models, resume and six refusals. Published contract v1.1 and locally prepared released consumers/profile do not add provider, assembled-stack, browser, registry-image or production acceptance. Worker/mediator v1.0 pins remain compatible and unchanged.

Runtime source and contract pins are unchanged. Exact clean pinned-source
snapshots were retained before this documentation-only checkout advanced.

Derived from the [local specification index](../.ai/specs/README.md) and
[root workspace authority](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/workspace-state.md).
The cited tests retain their original source identities; this documentation
refresh adds no activation, publication or runtime acceptance.

These are maintained Markdown documents derived from specifications, following
the pattern used by the other ThoughtKhoral projects. CI checks them; it does
not generate them or rewrite their sources.

| Document | Governing specification |
| --- | --- |
| [Repository README](../README.md) | [Public documentation](../.ai/specs/what/public-documentation.md) |
| [Cross-project Codex conversation status](https://github.com/thoughtkhoral/thought-khoral/blob/main/docs/codex-conversation-status.md) | Root and child Codex conversation specifications and coordinated plan |
| [Codex worker component status](project-status.md) | [Chat requirements](../.ai/specs/what/codex-chat-agent.md), [runtime design](../.ai/specs/how/codex-chat-agent.md), and [coordinated plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md) |
| [Architecture](architecture.md) | Approved [runtime design](../.ai/specs/how/codex-chat-agent.md) |
| [Worker runtime](runtime.md) | Approved [runtime design](../.ai/specs/how/codex-chat-agent.md) |
| [Native Linux acceptance](linux-acceptance.md) | Approved [Linux acceptance How](../.ai/specs/how/linux-worker-acceptance.md) |
| [Adapter library](adapter.md) | Approved [runtime design](../.ai/specs/how/codex-chat-agent.md) |
| [Third-party sources](../THIRD_PARTY_NOTICES.md) | Approved [runtime design](../.ai/specs/how/codex-chat-agent.md) |
| Shared conversation profile | Approved [integration profile](../.ai/specs/how/conversation-integration-profile.md), owned by contracts before publication |
| Future directory guidance | Draft [guided workspace requirements](../.ai/specs/what/guided-workspace.md) |
| [Contribution guidance](../CONTRIBUTING.md) | [Governance](../.ai/specs/decisions/001-governance-and-license.md) |
| [Agent instructions](../AGENTS.md) | [Foundation design](../.ai/specs/how/repository-foundation.md) |

The provider-free adapter/worker runtime and reviewed broker, mediator, UI, and
platform POC commits are pushed to their owning GitHub `main` branches. The
[runtime checkpoint](../.ai/specs/how/codex-chat-agent.md) and [coordinated
plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md)
record the exact commits and evidence. The v1.1 contract artifact is published; local released consumers/profile
are verified, while packaged-stack/browser and separately authorized live
checks remain pending.

Update the applicable What, How, and decision first. Once approved, implement
the change and update its derived docs. Draft descriptions remain explicitly
labelled as proposals. Pull requests link the accepted issue and specification.
Specs and public docs require maintainer review through CODEOWNERS routing.

Run `python3 scripts/check_docs.py` from the repository root. It checks local
links and source references without a network or provider key. It does not
prove remote-link availability or runtime compatibility.

Source: [public documentation requirements](../.ai/specs/what/public-documentation.md)
and [foundation verification design](../.ai/specs/how/repository-foundation.md).
