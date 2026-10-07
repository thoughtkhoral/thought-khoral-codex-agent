# Documentation workflow

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
record the exact commits and evidence. The v1.1 contract release artifact remains
unpublished; packaged-stack and separately authorized live checks remain pending.

Update the applicable What, How, and decision first. Once approved, implement
the change and update its derived docs. Draft descriptions remain explicitly
labelled as proposals. Pull requests link the accepted issue and specification.
Specs and public docs require maintainer review through CODEOWNERS routing.

Run `python3 scripts/check_docs.py` from the repository root. It checks local
links and source references without a network or provider key. It does not
prove remote-link availability or runtime compatibility.

Source: [public documentation requirements](../.ai/specs/what/public-documentation.md)
and [foundation verification design](../.ai/specs/how/repository-foundation.md).
