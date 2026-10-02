# ThoughtKhoral Codex agent

An independent Codex agent for the ThoughtKhoral collaborative workspace.
The proposed runtime wraps headless Codex and communicates through the agent
gateway; it does not access room storage directly.

## Status

**Specification-only.** Repository governance, documentation checks, and
Apache 2.0 licensing are established. Codex execution, A2A integration, session
persistence, and platform deployment are not implemented. Runtime specifications
remain drafts pending approval and a contract-backed implementation plan.

The proposed chat experience supports new and continued conversations, model
and reasoning-effort selection, visible effective settings, and an estimated
context-window indicator when runtime telemetry is available.

## Start here

- [Specification index](.ai/specs/README.md) — source of truth and approval status.
- [Architecture](docs/architecture.md) — proposed boundaries and data flow.
- [Documentation workflow](docs/README.md) — specification-to-document traceability.
- [Contributing](CONTRIBUTING.md) — issue-first contribution and verification.
- [ThoughtKhoral repository map](https://github.com/thoughtkhoral/thought-khoral/blob/main/docs/repository-map.md).

## Verification

The repository foundation requires Git and Python 3.10 or later. It does not
require Codex installation, provider authentication, or a running platform.

```sh
python3 scripts/check_docs.py
git diff --check
```

CI runs the documentation check on pushes and pull requests. Runtime build,
test, and deployment instructions will follow approved implementation work.

## Compatibility boundary

The [contracts repository](https://github.com/thoughtkhoral/thought-khoral-contracts)
owns cross-project compatibility. This scaffold pins no runtime contract and
makes no interoperability or production-support claim. The existing Reference
Agent remains independent; general bring-your-own-agent onboarding is future work.

## License and sources

Source and documentation are licensed under [Apache License 2.0](LICENSE).
See [NOTICE](NOTICE). Codex and provider services retain their own terms.

Governing specifications: [repository foundation](.ai/specs/what/repository-foundation.md),
[public documentation](.ai/specs/what/public-documentation.md), and the draft
[Codex chat requirements](.ai/specs/what/codex-chat-agent.md).
