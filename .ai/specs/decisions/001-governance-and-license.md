# 001 — Governance, identity, and Apache licensing

## Status

Accepted for repository foundation — user authorization on 2026-10-02.

## Decision

Name the independent repository `thought-khoral-codex-agent` in the
`thoughtkhoral` organization. Use Apache License, Version 2.0 for repository
source and documentation. Third-party runtime distributions retain their own
licenses; repository licensing does not relicense Codex or grant API access.

Inherit the root [workspace governance decision](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/001-workspace-governance.md)
and [issue-first contribution decision](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/decisions/004-contribution-and-change-management.md).
Specifications govern implementation and maintained documentation. Applicable
specifications require approval before runtime code or behavior changes.

The user's repository setup request authorizes this initial foundation. Future
public changes use accepted issues and governing specification references.
The root repository registers this child without tracking its source files.

## Consequences

The agent owns its release, provider credentials, native session storage, and
runtime implementation. Platform admission and room authority remain outside
this repository. Documentation distinguishes approved foundation from proposed
runtime behavior. Contributions must not contain provider keys, personal Codex
configuration, native session histories, or room content.

## Overrides

None. This decision inherits root governance and does not authorize general
remote agent admission or override room authorization.
