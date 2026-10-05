# ThoughtKhoral Codex agent specifications

Specifications are the source of truth for implementation and documentation.
Parent requirements in the [root specification index](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/README.md)
apply here. Overrides require an accepted local decision identifying the parent
rule, replacement, rationale, scope, approval, and consequences.

The repository foundation is approved by the user's setup request on
2026-10-02. Milestone-one runtime specifications and the coordinated plan were approved
on 2026-10-05 under [issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1). Task 4 implements the provider-free app-server adapter library. Durable worker
transport, end-to-end integration and production deployment remain unimplemented.

## Approved foundation

- [What: repository foundation](what/repository-foundation.md)
- [What: public documentation](what/public-documentation.md)
- [How: repository foundation and documentation checks](how/repository-foundation.md)
- [Decision 001: governance, identity, and license](decisions/001-governance-and-license.md)

## Approved milestone-one runtime

- [What: Codex chat conversations](what/codex-chat-agent.md)
- [How: headless Codex chat agent](how/codex-chat-agent.md)
- [How: room conversation integration profile](how/conversation-integration-profile.md)
- [Decision 002: independent Codex agent boundary](decisions/002-independent-codex-agent.md)

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
