# Native Linux acceptance

Governing specification: [approved Linux acceptance How](../.ai/specs/how/linux-worker-acceptance.md).

Verified checkpoint: all six native jobs passed at `9bcdbde` in
[run 44599536091](https://github.com/thoughtkhoral/thought-khoral-codex-agent/actions/runs/44599536091).
The [retained evidence](evidence/linux-worker-acceptance-44599536091.json) records
the exact source/lock, platforms/toolchains, image/proof hashes, artifact digests
and cleanup. This establishes native worker/package/policy evidence for that
revision; room-stack, browser, provider, release and production gates remain open.

The runtime workflow has four required native cells: Rust 1.85.0 and 1.93.1 on
AMD64 (`ubuntu-24.04`) and ARM64 (`ubuntu-24.04-arm`). Each records the actual
compiler, Cargo, native target, checked-out commit and reviewed lock hash before
running locked worker tests. The 1.93.1 AMD64 cell also runs existing quality
gates and offline helper tests. Both package jobs wait for the full matrix.

Each native package job builds the unchanged production Containerfile at that
same commit. The release builder remains Rust 1.93.1. It verifies the exact
image ID with isolated `--verify-package`, probes refusal of cross-architecture
evidence and changed controls/catalog, then extracts only the packaged CLI from
a stopped container. A disposable, pinned Python image runs the existing capture
harness with no external network, personal configuration or provider credentials.
Its 44 model/effort cases, eight models, resume and six unsolicited-tool refusals
must reproduce the committed target proof byte for byte.

The helper requires a clean event checkout and an empty evidence directory
outside the repository. Its Docker commands allocate resources only after native
host/runner/engine admission. All containers use a unique ownership label; cleanup
also runs after failures and in an always-run workflow step. Cleanup errors fail
the job. No global pruning or database volume removal is performed.

For local development, run the offline tests without starting any containers:

```sh
python3 scripts/test_linux_package.py
```

Those tests exercise admission, evidence validation, failure handling and cleanup
with a Docker boundary double. They do not establish actual native Linux acceptance.
Use the workflow for actual Linux checks after source publication is authorized.
The helper's `--help` documents its runtime, package and cleanup commands. Running
`package` starts test containers; existing local stacks should remain stopped.

Each CI cell uploads only allowlisted evidence with 14-day retention: JSON
manifests, successful target proof, cleanup result and bounded runtime-test summary.
The resource ledger, executable, source tree, native home and sessions are excluded.
Failed jobs may retain partial evidence, which never counts as acceptance. A passed
manifest covers its named checks; it does not replace a successful overall job,
artifact upload or the complete matrix. Retain reviewed run URLs and artifact
digests in acceptance records before artifacts expire.

Local implementation and offline checks do not close Task 9 packaged-stack,
rendered-browser, live-provider, contract publication or production acceptance.
