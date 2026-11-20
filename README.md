# ThoughtKhoral Codex agent

[Project documentation](docs/README.md) · [Documentation requirements](.ai/specs/what/public-documentation.md)

## Current workspace context — 2026-11-20

Native AMD64/ARM64 worker acceptance remains tied to tested source `9bcdbde` and its final lock. The 2026-11-20 provider-free ARM64 image from source `3f0318b` passed ten package/startup probes and exact native policy recapture: 44 model/effort cases, eight models, resume and six refusals. Published contract v1.1 and locally prepared released consumers/profile do not add provider, assembled-stack, browser, registry-image or production acceptance. Worker/mediator v1.0 pins remain compatible and unchanged.

Runtime source and contract pins are unchanged. Exact clean pinned-source
snapshots were retained before this documentation-only checkout advanced.

Derived from the [local specification index](.ai/specs/README.md) and
[root workspace authority](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/workspace-state.md).
The cited tests retain their original source identities; this documentation
refresh adds no activation, publication or runtime acceptance.

An independent Codex agent for the ThoughtKhoral collaborative workspace.
The runtime wraps headless Codex and communicates through the agent
gateway; it does not access room storage directly.

## Status

**Experimental provider-free Codex POC and reviewed room-integration commits are pushed to the owning GitHub `main` branches.**
The adapter/worker, broker, mediator, UI, and opt-in platform candidates were
reviewed and pushed on 2026-10-07; exact commits and evidence are in the
[coordinated plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md).
The F1 UI correction and visible defaults discovery passed provider-free
synthetic verification. The v1.1 artifact is published at `f1c8621`, and the
released consumers/profile are verified locally. Packaged-stack and separately authorized live
verification remain pending. This is experimental POC code, not a production
readiness claim. The milestone-one specifications and coordinated plan were
approved on 2026-10-05 under
[issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1).
The [conversation profile v1.0.0](https://github.com/thoughtkhoral/thought-khoral-contracts/releases/tag/thought-khoral-agent-conversation-v1.0.0)
is published and verified. Consumer repositories record immutable pins. The
published v1.0.0 artifact remains unchanged.

As of 2026-11-19, [native Linux acceptance](docs/linux-acceptance.md) passed
all four AMD64/ARM64 Rust 1.85.0/1.93.1 test cells and both actual package/policy
captures at the reviewed final lock. Each compiler cell passed 46 tests; both
target captures matched the committed proofs and cleaned all owned containers.
This closes worker-native portability gates only, with exact retained evidence.

The milestone-one goal is to let humans explicitly address Codex in a room and
have it use authorized room-wide conversation history, with shared continue and
fresh-session controls. The cross-project implementation, evidence limits, and
remaining gates are summarized in the [shared ThoughtKhoral status guide](https://github.com/thoughtkhoral/thought-khoral/blob/main/docs/codex-conversation-status.md).
This repository's [component status guide](docs/project-status.md) focuses on
the worker boundary. A later, separately scoped extension may add reviewed
specifications, instructions, and curated memory.

## Start here

- [Specification index](.ai/specs/README.md) — source of truth and approval status.
- [Cross-project Codex conversation status](https://github.com/thoughtkhoral/thought-khoral/blob/main/docs/codex-conversation-status.md) — ownership, evidence, and remaining milestone gates.
- [Codex component status](docs/project-status.md) — this repository's worker implementation.
- [Architecture](docs/architecture.md) — runtime boundaries and data flow.
- [Native Linux acceptance](docs/linux-acceptance.md) — supported compiler/architecture matrix and retained package-policy evidence.
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
v1.1 defaults is published, with released consumers/profile verified locally. Synthetic tests do not
establish packaged deployment or live interoperability. The existing Reference
Agent remains independent; general bring-your-own-agent onboarding is future work.

## License and sources

Source and documentation are licensed under [Apache License 2.0](LICENSE).
See [NOTICE](NOTICE). Codex and provider services retain their own terms.

Governing specifications: [repository foundation](.ai/specs/what/repository-foundation.md),
[public documentation](.ai/specs/what/public-documentation.md), and the approved
[Codex chat requirements](.ai/specs/what/codex-chat-agent.md).
