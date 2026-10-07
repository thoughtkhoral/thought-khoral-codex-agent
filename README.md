# ThoughtKhoral Codex agent

An independent Codex agent for the ThoughtKhoral collaborative workspace.
The proposed runtime wraps headless Codex and communicates through the agent
gateway; it does not access room storage directly.

## Status

**Scaffold checkout; Tasks 4–8 implemented on isolated local branches.**
The provider-free adapter is committed at `e61a5f79568ce24e411f559efc5290000638395a`
on `codex-app-server-adapter`. See the [execution checkpoint](.ai/specs/how/codex-chat-agent.md).
Repository governance, documentation checks, and
Apache 2.0 licensing are established. The adapter and durable A2A/SQLite worker use provider-free tests. Task 5 is
committed at `b23cf7cd002270de16a7b572a0e810e2fd9ff15d` on `codex-worker-receipts`;
see its [execution checkpoint](.ai/specs/how/codex-chat-agent.md). Task 6 gateway mediation and its
broker catalog adapter are committed on isolated local branches. The worker
transport correction and verified image are recorded in the same checkpoint.
Tasks 7–8 UI and opt-in platform packaging are committed locally on independent
branches, including the reviewed pinned-worker tool-policy correction. Task 9
synthetic verification and runtime corrections are reviewed locally. Important UI
finding F1 and resolved-default discovery are accepted on reviewed local
synthetic candidate branches. Contract publication, whole packaged-stack and
authorized live evidence remain pending. The milestone-one
specifications and coordinated plan were approved on 2026-10-05 under
[issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1).
The [conversation profile v1.0.0](https://github.com/thoughtkhoral/thought-khoral-contracts/releases/tag/thought-khoral-agent-conversation-v1.0.0)
is published and verified. Consumer runtime tasks must vendor that actual
artifact and record its immutable pin before implementation.

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
[public documentation](.ai/specs/what/public-documentation.md), and the approved
[Codex chat requirements](.ai/specs/what/codex-chat-agent.md).
