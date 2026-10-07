# Documentation workflow

These are maintained Markdown documents derived from specifications, following
the pattern used by the other ThoughtKhoral projects. CI checks them; it does
not generate them or rewrite their sources.

| Document | Governing specification |
| --- | --- |
| [Repository README](../README.md) | [Public documentation](../.ai/specs/what/public-documentation.md) |
| [Architecture](architecture.md) | Approved [runtime design](../.ai/specs/how/codex-chat-agent.md) |
| [Worker runtime](runtime.md) | Approved [runtime design](../.ai/specs/how/codex-chat-agent.md) |
| [Adapter library](adapter.md) | Approved [runtime design](../.ai/specs/how/codex-chat-agent.md) |
| [Third-party sources](../THIRD_PARTY_NOTICES.md) | Approved [runtime design](../.ai/specs/how/codex-chat-agent.md) |
| Shared conversation profile | Approved [integration profile](../.ai/specs/how/conversation-integration-profile.md), owned by contracts before publication |
| Future directory guidance | Draft [guided workspace requirements](../.ai/specs/what/guided-workspace.md) |
| [Contribution guidance](../CONTRIBUTING.md) | [Governance](../.ai/specs/decisions/001-governance-and-license.md) |
| [Agent instructions](../AGENTS.md) | [Foundation design](../.ai/specs/how/repository-foundation.md) |

The provider-free adapter/worker runtime is merged into this repository's local
`main`; the reviewed broker, mediator, UI and platform candidates are merged into
their owning repositories' local `main` branches. The [runtime checkpoint](../.ai/specs/how/codex-chat-agent.md)
and [coordinated plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md)
record the exact commits and evidence. No integration branch has been pushed.
The v1.1 contract remains unpublished; packaged-stack and separately authorized
live checks remain pending.

Update the applicable What, How, and decision first. Once approved, implement
the change and update its derived docs. Draft descriptions remain explicitly
labelled as proposals. Pull requests link the accepted issue and specification.
Specs and public docs require maintainer review through CODEOWNERS routing.

Run `python3 scripts/check_docs.py` from the repository root. It checks local
links and source references without a network or provider key. It does not
prove remote-link availability or runtime compatibility.

Source: [public documentation requirements](../.ai/specs/what/public-documentation.md)
and [foundation verification design](../.ai/specs/how/repository-foundation.md).
