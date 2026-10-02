# Documentation workflow

These are maintained Markdown documents derived from specifications, following
the pattern used by the other ThoughtKhoral projects. CI checks them; it does
not generate them or rewrite their sources.

| Document | Governing specification |
| --- | --- |
| [Repository README](../README.md) | [Public documentation](../.ai/specs/what/public-documentation.md) |
| [Architecture](architecture.md) | Draft [runtime design](../.ai/specs/how/codex-chat-agent.md) |
| [Contribution guidance](../CONTRIBUTING.md) | [Governance](../.ai/specs/decisions/001-governance-and-license.md) |
| [Agent instructions](../AGENTS.md) | [Foundation design](../.ai/specs/how/repository-foundation.md) |

Update the applicable What, How, and decision first. Once approved, implement
the change and update its derived docs. Draft descriptions remain explicitly
labelled as proposals. Pull requests link the accepted issue and specification.
Specs and public docs require maintainer review through CODEOWNERS routing.

Run `python3 scripts/check_docs.py` from the repository root. It checks local
links and source references without a network or provider key. It does not
prove remote-link availability or runtime compatibility.

Source: [public documentation requirements](../.ai/specs/what/public-documentation.md)
and [foundation verification design](../.ai/specs/how/repository-foundation.md).
