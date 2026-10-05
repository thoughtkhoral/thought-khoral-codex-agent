# ThoughtKhoral Codex agent

An independent Codex agent for the ThoughtKhoral collaborative workspace.
The approved runtime wraps headless Codex and communicates through the agent
gateway; it does not access room storage directly.

## Status

**Provider-free adapter library implemented (Task 4).** Governance, approved
specifications and Apache 2.0 licensing are established. The independent library
wraps the pinned CLI protocol, validates authorized context, discovers model/effort
options and normalizes replies/settings/usage. Deterministic external-process tests
exercise its submission barrier and lifecycle. See [adapter verification](docs/adapter.md).
Durable receipts, authenticated worker transport and platform activation remain
later tasks. No provider access or live isolation verification has occurred.

The milestone-one specifications and coordinated plan were approved on 2026-10-05
under [issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1).
The published [conversation profile v1.0.0](https://github.com/thoughtkhoral/thought-khoral-contracts/releases/tag/thought-khoral-agent-conversation-v1.0.0)
is vendored with its immutable commit/archive/file hashes. Native generated
Codex CLI 0.160.0 schemas are independently pinned.

The proposed chat experience makes Codex a room participant that responds when
explicitly addressed, using authorized room-wide discussion as context. Its
proposed shared room session supports new and continued native threads, model
and reasoning-effort selection, visible effective settings, and an estimated
context-window indicator when runtime telemetry is available. A future extension
adds a reviewed directory of specifications, instructions, and curated memory.

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

CI runs the documentation check on pushes and pull requests.
The [adapter guide](docs/adapter.md) documents local Cargo/fake-process checks.
Runtime CI and independent worker packaging belong to Task 5.

## Compatibility boundary

The [contracts repository](https://github.com/thoughtkhoral/thought-khoral-contracts)
owns cross-project compatibility. The library pins the published conversation
profile; end-to-end interoperability and production support remain unverified. The existing Reference
Agent remains independent; general bring-your-own-agent onboarding is future work.

## License and sources

Source and documentation are licensed under [Apache License 2.0](LICENSE).
See [NOTICE](NOTICE). Codex and provider services retain their own terms.

Governing specifications: [repository foundation](.ai/specs/what/repository-foundation.md),
[public documentation](.ai/specs/what/public-documentation.md), and the approved
[Codex chat requirements](.ai/specs/what/codex-chat-agent.md).
