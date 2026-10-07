# ThoughtKhoral Codex agent

An independent Codex agent for the ThoughtKhoral collaborative workspace.
The runtime wraps headless Codex and communicates through the agent
gateway; it does not access room storage directly.

## Status

**Provider-free Codex agent and worker runtime is merged into local `main`.**
The provider-free adapter is committed at `e61a5f79568ce24e411f559efc5290000638395a`
and merged from `codex-app-server-adapter`. See the [execution checkpoint](.ai/specs/how/codex-chat-agent.md).
Repository governance, documentation checks, and Apache 2.0 licensing are
established. The provider-free adapter/worker, broker, mediator, UI, and opt-in
platform candidates are merged into their owning repositories' local `main`
branches from the reviewed commits. This local integration was authorized on
2026-10-07 and is recorded in the [coordinated plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md).
The branches have not been pushed. The F1 UI correction and visible defaults
discovery are accepted for this local synthetic candidate; the v1.1 contract
remains unpublished. Packaged-stack verification and separately authorized live
evidence remain pending. The milestone-one
specifications and coordinated plan were approved on 2026-10-05 under
[issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1).
The [conversation profile v1.0.0](https://github.com/thoughtkhoral/thought-khoral-contracts/releases/tag/thought-khoral-agent-conversation-v1.0.0)
is published and verified. Consumer repositories record immutable pins; the
v1.1 defaults candidate remains local and unpublished.

The proposed chat experience makes Codex a room participant that responds when
explicitly addressed, using authorized room-wide discussion as context. Its
proposed shared room session supports new and continued native threads, model
and reasoning-effort selection, visible effective settings, and an estimated
context-window indicator when runtime telemetry is available. A future extension
adds a reviewed directory of specifications, instructions, and curated memory.

## Start here

- [Specification index](.ai/specs/README.md) — source of truth and approval status.
- [Architecture](docs/architecture.md) — runtime boundaries and data flow.
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

Run the provider-free runtime suite with `cargo test --locked --offline`.
CI runs the runtime and documentation checks on pushes and pull requests.

## Compatibility boundary

The [contracts repository](https://github.com/thoughtkhoral/thought-khoral-contracts)
owns cross-project compatibility. This runtime pins the published v1.0 profile;
the reviewed v1.1 defaults candidate remains unreleased. Synthetic tests do not
establish packaged deployment or live interoperability. The existing Reference
Agent remains independent; general bring-your-own-agent onboarding is future work.

## License and sources

Source and documentation are licensed under [Apache License 2.0](LICENSE).
See [NOTICE](NOTICE). Codex and provider services retain their own terms.

Governing specifications: [repository foundation](.ai/specs/what/repository-foundation.md),
[public documentation](.ai/specs/what/public-documentation.md), and the approved
[Codex chat requirements](.ai/specs/what/codex-chat-agent.md).
