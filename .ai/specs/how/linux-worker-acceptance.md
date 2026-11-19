# Native Linux worker acceptance automation

Status: approved by the maintainer on 2026-11-19 with “approved” after review of
this concrete How. Local workflow/helper implementation, offline verification,
independent review and specification-first commits are authorized. Actual native
Linux acceptance awaits published jobs; local containers remain stopped.

## Authority and baseline

This continues accepted [worker issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1),
the independent-build/release criterion in the [existing What](../what/codex-chat-agent.md),
and Linux portability gates in the [coordinated plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md).
It implements the approved [architecture evidence](x86-package-evidence-proposal.md)
and [Rust 1.85 correction](rust-185-lock-compatibility-proposal.md). No new What,
decision, parent override, wire contract, model authority or runtime tool is added.

Worker baseline `429f6be` contains final lock SHA-256
`1c70ff10992b000c93232155ca0032cab9b7a6c27a13ca4ac46eba862e864a34`.
Current runtime CI tests only Rust 1.93.1 on an x86 runner and verifies a package;
it does not recapture native CLI evidence or test the Linux minimum. Earlier
Linux images predate the corrected lock and cannot close these gates.

## Changed files and boundaries

| File | Responsibility |
| --- | --- |
| `.github/workflows/runtime.yml` | Native test/package matrices, mandatory checks, evidence upload and cleanup. |
| `scripts/verify-linux-package.py` | Docker-only CI orchestration, architecture/source admission, actual package/capture checks, result manifest and owned-resource cleanup. |
| `scripts/test_linux_package.py` | Offline admission, evidence-validation, failure and cleanup tests at the external-engine boundary; no local engine/container required. |
| `scripts/fixtures/linux-acceptance-pins.json` | Reviewed test-only Python image/index/platform pins. |
| `docs/linux-acceptance.md` | Spec-derived instructions and evidence interpretation. |
| Owning spec index/main How, this amendment and root plan | Approval, discovery, traceability and attributed results. |

The production `Containerfile`, Rust source, Cargo manifest/lock, CLI version,
catalog, controls and committed proofs remain unchanged. Reuse the existing
capture harness's `--inside` path without changing its legacy Podman flow/pins.

## Native runtime matrix

| Runner | Required host / Docker / image architecture | Rust versions |
| --- | --- | --- |
| `ubuntu-24.04` | Linux `x86_64` / `amd64` / `linux/amd64` | 1.85.0 and 1.93.1 |
| `ubuntu-24.04-arm` | Linux `aarch64` / `arm64` / `linux/arm64` | 1.85.0 and 1.93.1 |

All four cells run `cargo test --locked -- --test-threads=1` and record/assert
actual Rust/Cargo versions and native targets. Keep `fail-fast: false`; every
cell is required. Emulation, QEMU, cross-compilation and alternate-runner
fallback cannot satisfy a native cell. Keep existing fmt, denied-warning clippy,
artifact/legal/docs/catalog/capture-helper checks on the 1.93.1 AMD64 cell and
add the new offline helper tests there. No `continue-on-error` pass is allowed.

## Native package and capture jobs

Two native package jobs depend on the entire runtime matrix. Each job must:

1. Check out exactly the event commit with persisted credentials disabled.
   Require a clean tree and actual HEAD equal to the validated event SHA before
   writing evidence outside the checkout. Require the actual lock digest to
   match the reviewed dependency inventory; record it rather than hardcoding
   an old digest into the workflow.
2. Admit host, runner and Docker architecture before any container allocation.
   Build the unchanged production `Containerfile` from that checkout natively.
   Resolve the result to an immutable image ID and verify Linux/architecture.
   No old image tag, cross-build recipe, TLS bypass or synthetic worker qualifies.
3. Run that exact image ID's `--verify-package` with network disabled, read-only
   root, all capabilities dropped and packaged UID/GID 10003. Require both
   existing success markers. Other-architecture proof, modified controls and
   modified catalog must each refuse admission without success markers.
4. Create a uniquely named stopped container from that worker ID, copy out only
   `/usr/local/bin/codex`, then remove it. Verify the executable digest against
   the committed target proof. Do not activate the production worker service.
5. Build a disposable capture image from the pinned Python target manifest and
   exact extracted executable, using a context containing only that executable
   and generated test Containerfile. Inspect and run its immutable image ID.
6. Execute existing `test-native-tool-policy.py --inside` as UID/GID 10003,
   with `--network none`, `--read-only`, `--cap-drop ALL`, temporary `/tmp`,
   read-only source and an empty owned output directory. The only provider is
   its loopback synthetic HTTP/SSE server; no credentials or personal config.
7. Require byte-identical committed target proof plus 44 unique model/effort
   cases across eight models, empty tools, fresh-process resume and all six
   recorded unsolicited-tool refusals. Never overwrite a proof to make CI pass.
8. Return success and a passed manifest only after every required check and
   cleanup passes. Missing/unavailable/failed checks remain failures, with
   bounded partial evidence and nonzero exit status.

The image builder remains the pinned Rust 1.93.1 image. Minimum-version support
comes from actual 1.85.0 native test cells; do not call a 1.93-built release image
a Rust 1.85 build. The existing mandatory package gate is retained and expanded.

## Exact reviewed CI/test pins

Use these full action commits with readable version comments. Rust selection
is the pinned action's explicit `toolchain` input, not an expression in `uses:`.

| Input | Exact reference |
| --- | --- |
| Checkout v6 | `actions/checkout@d23441a48e516b6c34aea4fa41551a30e30af803` |
| Rust setup, master-history commit | `dtolnay/rust-toolchain@df2a88e9fe4098bad09cb42c3dec7187f15861fc` |
| Artifact upload v7.0.2 | `actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9` |
| Test-only Python index | `docker.io/library/python@sha256:a83ba18aef0f865b1a4a611c2cf460fe446f2795673bbb9fe43be570439e3b39` |
| Python AMD64 manifest | `sha256:e8c69d86735f6df3efa06b324694fe51f0422785f83a3bd463052636c2421944` |
| Python ARM64 manifest | `sha256:464bee9ac10a34b0c86031f12d94d7ccc4643725773b23eff4076305fafab420` |

Python is test infrastructure only, never copied into the production image or
Rust legal inventory. Existing production image, CLI/archive/schema and policy
pins are preserved. No new production dependency or Dockerfile edit is approved.

## Evidence and cleanup

Record the checked-out commit/clean-tree result; Cargo.lock/Containerfile hashes;
requested/actual platforms; runner/Docker versions; Rust test versions; immutable
worker/capture IDs; Python pins; CLI/catalog/control/proof hashes; case/refusal
counts; named outcomes; and final cleanup status. This is trusted CI execution
evidence, not a cryptographic build-provenance claim.

Upload only allowlisted JSON manifests/proofs and bounded version/verification
summaries, with unique commit/run-attempt/architecture/toolchain names and 14-day
retention. Missing expected evidence fails upload; hidden files and overwrite
remain disabled. Partial evidence never overrides a failed job/upload. Record
actual accepted run URLs/artifact digests in reviewed evidence later; expiring
artifacts are not permanent acceptance records.

Retain only `contents: read` permissions. No `pull_request_target`, repository
secrets, provider calls, image push, release, deployment or automatic source
writes. Never upload homes, receipts/sessions, credentials, environment dumps,
checkout contents or private room data.

Use unique owned IDs and always-run cleanup for verification/capture containers,
the stopped extraction container and temporary tags. Handle partial allocation,
errors and signals; leftover owned containers fail visibly. Never globally prune,
delete unrelated resources or remove database volumes. Existing local stacks
remain stopped during implementation and offline validation.

## Acceptance criteria

- [ ] Four native Linux/toolchain cells pass on the same source/lock, without
  emulation, cross-build or platform fallback.
- [ ] Both actual production-package builds, admission/tamper probes and exact
  native CLI recaptures pass at that revision and lock digest.
- [x] Offline tests prove refusal for wrong host/engine/image architecture,
  dirty/mismatched source, stale lock, failed verifier, wrong CLI, changed/partial
  proof and cleanup failure. Assert real validation/outcomes, not only mock calls.
- [x] Missing/failing steps cannot create a passed manifest; failure tests retain
  bounded partial evidence and clean only their own resources.
- [x] Existing worker suites, fmt/clippy, pins/legal/docs and root hierarchy,
  reference/identity checks pass after implementation.
- [x] Local no-engine tests are labelled as such. Native acceptance is recorded
  only after actual Linux jobs pass, without closing live-provider, rendered-
  browser, full Codex packaged-stack or production gates by implication.

## Approval and execution

Approval is recorded in the owning index/main How and root plan before code.
Add failing helper cases, implement
the narrow orchestration, run offline checks and obtain independent review.
Commit specifications before implementation with descriptive issue/spec comments.

Pushing, dispatching published CI, provider activation and release remain
separate actions. The prior six local maintenance commits are still unpushed.
This approval grants no local container activation.

## Local implementation checkpoint — 2026-11-19

The approved workflow/helper, reviewed test-only pins, offline tests and derived
Linux acceptance guide are implemented locally. The specification approval was
committed first as `5fd884e`; implementation has not been pushed or executed on
published Linux runners. Both native acceptance checkboxes above remain open.

Verification at the unchanged lock digest recorded above:

- `python3 scripts/test_linux_package.py`: 15 tests pass using a Docker boundary
  double; no engine or container is used. These include both architecture paths,
  source/lock refusal, runtime-version/native-target refusal, incomplete proofs,
  partial allocation, cleanup failure/retry and stale passed-manifest revocation.
- Review identified that any nonzero tamper exit could mask an infrastructure
  failure. Regression tests first reproduced 24 failed subcases. The helper now
  inspects completed, started, non-OOM container state and requires the actual
  worker's exact policy result: exit 1 and `runtime_unavailable`. Launch failures,
  kills and unrelated diagnostics at each of the three probes fail acceptance.
  Final independent static review reported no remaining material finding.
- All 46 Rust tests pass on both 1.85.0 and 1.93.1 on **macOS ARM64**. The first
  1.85 run hit a sandbox permission denial in a descendant-process cleanup test;
  the authorized rerun outside that restriction passed the full suite. This is
  host evidence, not native Linux or Docker evidence.
- Rust 1.93.1 fmt/clippy with denied warnings, 144 contract pins, 478 retained
  legal files for 276 crates, catalog/tool policy, three existing capture-helper
  tests, 120 local documentation links and whitespace checks pass.
- Upstream-checksum-verified actionlint 1.7.12 accepts the workflow. Root spec
  hierarchy, repository references and identity checks pass. No running Podman
  containers were present; local stacks stayed stopped throughout.

Next acceptance requires authorized publication followed by the four actual
native runtime cells and both final-lock package/capture jobs, with reviewed run
URLs and retained artifact digests. Local tests and review do not close those
gates or any Task 9, browser, provider, release or production gate.

## Primary references checked on 2026-11-19

Repository metadata confirms the worker repository is public. Native runner
labels, action inputs/commits and Python index/platform digests were verified
from their owners without running containers.

- [GitHub native runner labels](https://docs.github.com/en/actions/how-tos/write-workflows/choose-where-workflows-run/choose-the-runner-for-a-job).
- [Rust action inputs and master-history SHA guidance](https://github.com/dtolnay/rust-toolchain/blob/df2a88e9fe4098bad09cb42c3dec7187f15861fc/README.md).
- [Artifact action inputs](https://github.com/actions/upload-artifact/blob/cf430e030ddbb5b0abf93d22962f4752f3646cd9/action.yml).
- [Docker copying from stopped containers](https://docs.docker.com/reference/cli/docker/container/cp/).
