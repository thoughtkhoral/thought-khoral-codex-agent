# Contributing

Follow the [organization contribution policy](https://github.com/thoughtkhoral/.github/blob/main/CONTRIBUTING.md).
Future changes start with an issue in this repository. A maintainer accepts the
request and approves the applicable specification before implementation.
Pull requests link the accepted issue and governing What, How, or decision.

## Source of truth

Read the [specification index](.ai/specs/README.md). Product requirements live
in `what/`, technical designs in `how/`, and durable choices in `decisions/`.
Milestone-one runtime specifications are approved and the worker is implemented.
Future guidance/memory specifications remain drafts. The original foundation
alone did not authorize runtime work; applicable approved runtime specs govern
current changes. Public documentation follows those specifications.

## Verification

Use Python 3.10 or later for documentation checks:

```sh
python3 scripts/check_docs.py
git diff --check
```

Runtime changes also require locked Rust tests, fmt/clippy and the contract,
legal and policy checkers specified in [AGENTS.md](AGENTS.md). See the
[worker guide](docs/runtime.md) and [native Linux verification](docs/linux-acceptance.md)
for the supported toolchain/architecture matrix and exact evidence. Documentation
is maintained from approved specs; CI validates it without generating prose.

Explain behavior, compatibility, and documentation impact. Keep changes focused;
never commit provider keys, local environment files, session histories, or
private room content. Preserve [Apache 2.0 licensing](LICENSE).

## Security and support

Use the [private security-reporting policy](https://github.com/thoughtkhoral/.github/blob/main/SECURITY.md)
for vulnerabilities. Other Codex agent questions belong in this repository's
issues; platform deployment questions belong in the platform repository.

Governing specifications: [governance decision](.ai/specs/decisions/001-governance-and-license.md)
and [repository foundation](.ai/specs/how/repository-foundation.md).
