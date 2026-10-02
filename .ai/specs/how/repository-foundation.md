# Repository foundation and documentation verification

## Status

Approved for repository setup — user authorization on 2026-10-02.

## Governing requirements

- [Repository foundation](../what/repository-foundation.md)
- [Public documentation](../what/public-documentation.md)
- [Governance and license](../decisions/001-governance-and-license.md)

## Structure

`.ai/specs` owns What, How, and decisions. `README.md`, `CONTRIBUTING.md`,
`AGENTS.md`, and `docs/` are derived maintained guidance. `LICENSE` holds the
full Apache 2.0 license, and `NOTICE` identifies project contributors.
`.github/CODEOWNERS` routes reviews to the repository maintainer.
Organization community defaults supply issue forms and security policy.

The initial commit contains no runtime package, image, contract pin, or service.
Runtime specs describe the proposed integration; they are not a release claim.

## Documentation workflow

Change the source What/How/decision first, obtain approval for applicable
behavior changes, then update code and derived documentation in that order.
Documents list their governing specification under a Source or Governing
specifications paragraph. Reviewers assess both requirements and derived text.

`python3 scripts/check_docs.py` uses only the Python 3 standard library. It
checks required foundation files, nonempty specification directories, the root
governance link, the Apache license identifier, documentation source references,
and repository-relative Markdown link targets outside fenced code blocks.
External HTTPS links are not a network/availability check. Runtime and
cross-repository compatibility are not established by this command.

GitHub Actions runs the same check on pushes, pull requests, and manual dispatch,
with `contents: read`. It does not execute Codex, access provider credentials,
or generate documentation. Cross-repository link validation belongs to the
organization's documentation workflow.

## Git and publication

Initialize an independent Git repository on `main`; root Git ignores this child.
Publish the reviewed foundation to the public ThoughtKhoral organization,
enable issues, disable the wiki, and apply organization labels used by inherited
issue forms. CODEOWNERS provides review routing; enforcing owner approval
requires server-side branch protection and is not claimed by this scaffold.

## Verification

Run the documentation check, demonstrate that a missing relative link is
rejected, run `git diff --check`, compare the full Apache license text to its
canonical source, and verify remote default branch, license recognition,
published commit, and CI status. Do not claim runtime tests have passed.
