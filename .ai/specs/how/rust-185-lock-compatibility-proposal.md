# Restore the declared Rust 1.85 compatibility

Status: accepted by the maintainer in this working session on 2026-11-19.
The exact dependency-lock correction, legal inventory refresh, and local
verification are authorized. Publication and provider activation remain separate.

Approval record: following the successful isolated candidate and explicit
amendment request, the maintainer replied "approval granted as requested".
This acceptance was recorded before changing the canonical dependency lock.
It continues existing accepted [worker contribution issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1)
and the [independent build/release requirement](../what/codex-chat-agent.md),
restoring the existing technical minimum rather than changing the product What.
No new decision, parent override, direct dependency requirement, or wire
contract change is introduced.

Governing sources: [runtime design](codex-chat-agent.md), the declared
`rust-version = "1.85"` in `Cargo.toml`, and the
[root specification-first rule](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/README.md).

## Reproduced failure and tested candidate

Actual Rust 1.85.0 cannot compile locked `yoke-derive 0.8.3`: its
`src/lib.rs:202` calls `str::from_utf8`, unavailable on that compiler. The
dependency reaches the worker through yoke, ICU, and idna. Its declared metadata
does not prevent Cargo from selecting it for the existing minimum compiler.

In an isolated disposable worker copy, the single explicit resolver change
`cargo +1.85.0 update --offline -p yoke-derive --precise 0.8.2` restores the
build. It preserves `yoke 0.8.3` and adds its derive dependency
`synstructure 0.13.2` alongside the existing `synstructure 0.14.0`.
The actual Rust 1.85 check and all 46 tests passed on macOS ARM64. The isolated
candidate lock SHA-256 is
`1c70ff10992b000c93232155ca0032cab9b7a6c27a13ca4ac46eba862e864a34`.
This is not Linux minimum-version evidence or final application acceptance.

## Approved bounded correction

Apply this exact tested transitive lockfile correction without changing direct
dependency requirements or raising the declared Rust minimum. Refresh the
worker-owned dependency-notice inventory and retained legal files for
`yoke-derive 0.8.2` and the additional `synstructure 0.13.2`, removing the replaced
derive version's obsolete inventory entry. Their license texts match versions
already retained; preserve original legal-file bytes and provenance. The locked
package inventory changes from 275 to 276 packages.

Require the actual Rust 1.85 locked tests and current-toolchain tests, formatting,
clippy, dependency/legal, contract-pin and documentation checks to pass against
the final worker source. Record Linux execution separately when available.
Do not patch shared Cargo registry sources, bypass compiler checks, alter
protocol schemas, change tools or model policy, or claim broader compatibility
from dependency metadata alone.

The already approved worker architecture-evidence and platform build-input
corrections remain independently scoped. No provider activation, release,
remote push/merge, or contract publication is included in this amendment.

## Final-source execution evidence — 2026-11-19

The exact approved lock is applied to the canonical worker source, including
the previously approved architecture-evidence correction. The refreshed
inventory matches all 276 registry package versions and archive checksums in
`Cargo.lock`. All 478 retained legal files verify. New crate metadata and legal
texts were read from checksum-verified cached registry archives; obsolete
`yoke-derive 0.8.3` legal inventory was replaced with the original 0.8.2 text.

On macOS ARM64, both actual Rust/Cargo 1.85.0 and 1.93.1 passed all 46 tests
with `cargo test --locked --offline -- --test-threads=1`. Minimum-version tests
used an isolated Cargo cache and target directory. Logs are retained under
`/private/tmp/thought-khoral-resume-2026-11-19/` as
`worker-rust185-final.log` and `worker-rust193-final-lock.log`.
Independent read-only review confirmed the exact lock hash, all package/checksum
entries, unchanged direct dependency requirements, and original legal bytes.
Formatting and current-toolchain clippy with warnings denied passed, along with
the contract-pin, legal-inventory, documentation, catalog-policy and three
capture-helper tests. Root specification/reference/identity checks passed.

No containers were started. The earlier Linux image and packaged-stack results
belong to the prior dependency lock; final-lock Linux and native x86_64 CI
evidence remain open. Existing runtime/provider activation and publication
gates remain unchanged.
