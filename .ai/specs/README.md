# ThoughtKhoral Codex agent specifications

Specifications are the source of truth for implementation and documentation.
Parent requirements in the [root specification index](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/README.md)
apply here. Overrides require an accepted local decision identifying the parent
rule, replacement, rationale, scope, approval, and consequences.

The repository foundation is approved by the user's setup request on
2026-10-02. Runtime specifications are drafts; no Codex runtime, gateway
integration, or production deployment is implemented or approved by this setup.

## Approved foundation

- [What: repository foundation](what/repository-foundation.md)
- [What: public documentation](what/public-documentation.md)
- [How: repository foundation and documentation checks](how/repository-foundation.md)
- [Decision 001: governance, identity, and license](decisions/001-governance-and-license.md)

## Proposed runtime

- [What: Codex chat conversations](what/codex-chat-agent.md)
- [How: headless Codex chat agent](how/codex-chat-agent.md)
- [Decision 002: independent Codex agent boundary](decisions/002-independent-codex-agent.md)

Approve the applicable runtime What, How, contract profile, and implementation
plan before creating runtime code. [Documentation](../../docs/README.md) remains
derived context rather than an independent source of product requirements.
