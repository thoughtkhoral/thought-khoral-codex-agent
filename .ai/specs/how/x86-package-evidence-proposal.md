# Architecture-specific package evidence

Status: accepted by the maintainer in this working session on 2026-11-19.
Implementation and local verification are authorized; publication and live
activation retain their separate gates.

Approval record: the maintainer replied "approved" to the two concrete
architecture-evidence and platform-build amendments before implementation.
This continues existing accepted [worker contribution issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1).
It implements the existing [independent build/release acceptance criterion](../what/codex-chat-agent.md)
and approved no-tools runtime boundary; it changes How evidence is established,
not the product What or agent authority. No new architectural decision or
contract revision is required.

Governing sources: [approved runtime design](codex-chat-agent.md), particularly
the Task 8 tool-policy correction, and the [root specification-first rule](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/README.md).
This proposal completes the existing x86_64 capture/admission gate. It changes
no room contract, model catalog, tool controls, provider origin, or activation
policy.

## Observed problem

The latest GitHub [provider-free runtime run](https://github.com/thoughtkhoral/thought-khoral-codex-agent/actions/runs/37699754057)
builds its Linux x86_64 worker image, then fails `--verify-package` with
`runtime_unavailable`. The image includes only `tool-policy-proof-aarch64.json`;
the verifier compares the installed CLI digest with that ARM64 capture. The
approved design deliberately rejects x86_64 until equivalent actual-binary
evidence is added. The retained ARM64 image still passes its package check.

## Approved bounded change

1. Capture the pinned Codex CLI 0.160.0 x86_64 binary against the existing
   loopback synthetic HTTP/SSE harness, inside a network-disabled container
   with no provider credentials. Verify the existing immutable release archive
   checksum before execution. Preserve all 44 model/effort cases, fresh-process
   resume, recursively empty tool arrays, and all six unsolicited-tool refusals.
   Record execution architecture and whether CPU emulation was used in the
   accompanying evidence report; emulation is not native-host parity evidence.
2. Retain the exact existing ARM64 proof and add a separately pinned
   `tool-policy-proof-x86_64.json`. Select the embedded and installed proof by
   the worker build target architecture. Each proof continues binding the
   actual installed CLI digest, exact catalog, exact controls, and captured
   behaviors. Unsupported architectures and absent, cross-architecture,
   modified, or unreviewed evidence fail closed.
3. Package both proof files, pin their hashes in the existing CLI artifact
   lock, and update provenance. Keep the existing no-network, read-only CI
   package check mandatory. A passing native x86_64 CI run remains required
   before claiming that CI failure resolved or native-host package parity.
4. Make capture reproducible from an explicitly pinned source image and target
   platform, preserving the existing default ARM64 source-image pin. Run the
   real capture before accepting a new proof; do not derive a proof by changing
   the architecture or digest in another platform's JSON.

## Validation and review

Before implementation, add a failing regression demonstrating that the actual
x86_64 capture is rejected by the existing ARM64-only verifier. After the change,
verify each platform's matching evidence is accepted and a wrong CLI digest,
the other platform's proof, and modified catalog/controls/proof are rejected.
Run the required Rust tests, formatting, clippy, contract-pin, policy,
dependency-notice and documentation checks. Rebuild and verify both package
targets where available; attribute emulated execution separately. Rerun the
existing pinned provider-free composed conversation checks independently.

## Boundaries retained

No service activation, real provider calls, release/tag publication, remote
push/merge, contract upgrade, new model entitlement, filesystem tools, or memory
integration is authorized by this amendment. Packaged-stack, real browser/login,
live-provider isolation, and Rust 1.85 acceptance retain their existing gates.

## Local execution checkpoint — 2026-11-19

The exact x86_64 capture is vendored and pinned, and the verifier selects the
proof using its compile target. Tests additionally reject another architecture's
proof and a modified catalog. The capture harness accepts immutable image IDs
and explicit platform selection while preserving its original default image pins.

All 46 current-toolchain worker tests passed outside the host sandbox; the
process-group inspection test cannot run under that sandbox's restrictions.
Formatting, clippy with denied warnings, all three Python capture-helper tests,
144 artifact pins, retained legal inventory, documentation, and tool-policy
checks passed. The updated ARM64 capture harness regenerated the existing
ARM64 proof byte for byte. Independent code review found no outstanding issues.

The locally cross-compiled x86_64 image
`sha256:586f436750780f040f896476c7ebe0e407cc05c4070e37c1d0e81dc210623072`
passed actual `--verify-package` execution under CPU emulation, with no network,
capabilities, or writable root filesystem. Substituting the ARM64 proof,
altering controls, and altering the catalog each refused admission.

The updated explicit-target capture used that worker image and Python harness
`sha256:cf2dad23925ed51a00735f57606b2998aedf4705a1634cb9ae1053b5c145d5ba`
with `--platform linux/amd64`. It regenerated the pinned x86_64 proof byte for
byte. The local invocation/output is recorded in `x86-capture-after.log` under
`/private/tmp/thought-khoral-resume-2026-11-19/`; both Python and CLI were emulated
for this run, unlike the initial mixed-architecture capture.

Native x86_64 compiler execution failed under local CPU emulation, including
`rustc -vV`. The local acceptance image instead used the working ARM64 compiler
to cross-compile `x86_64-unknown-linux-gnu`, with the required target C compiler
and headers. A temporary build recipe limited Cargo to one job after VM OOM
and consumed the previously downloaded CLI archive after validating its exact
existing checksum. The original archive/schema checks and runtime packaging
were preserved; no TLS verification was disabled. Temporary build recipes and
logs remain in the evidence directory. These environment accommodations do not
establish a successful native CI build. The original CI package gate remains
mandatory and unchanged.

The ARM64 rebuild, likewise limited to one Cargo job and using the exact
checksum-verified release archive, produced image
`sha256:3f46c52e8615df74247590bc40e4aa3c36c33263b095e9f770625be468db3d52`.
All ten real-image package/startup checks passed: missing/shared credentials,
imported config/auth, state ownership/mode, mutable image references, wrong CLI
version, stale admission removal, and positive readiness without inference.
Both local package targets therefore have fresh admission evidence, with the
x86_64 target explicitly attributed to cross-compilation and CPU emulation.

The unchanged pinned composed defaults integration was rerun after these
changes and passed all 24 synthetic turns, six recovery boundaries, and 12
rendered jsdom UI lifecycle phases. Retained evidence is at
`/var/folders/70/5kxy5kys3bj0252chp3ck8900000gn/T/Task9-codex-conversation-z86Gbw`.
It uses its original exact reviewed source pins; actual new-image acceptance
above is separate evidence and does not inherit a live-provider claim.
