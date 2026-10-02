# Codex agent repository foundation

## Status

Approved for repository setup — user authorization on 2026-10-02.

## Purpose

Establish `thoughtkhoral/thought-khoral-codex-agent` as an independent component
that can implement the first model-backed agent without embedding its runtime
in the ThoughtKhoral agent gateway.

## Acceptance criteria

1. The repository uses `.ai/specs/what`, `.ai/specs/how`, and
   `.ai/specs/decisions`, with an index linking root governance.
2. The foundation and documentation requirements have explicit approved status;
   planned runtime behavior remains clearly labelled draft.
3. Source and documentation are licensed under Apache License, Version 2.0.
4. README and local documents cite governing specifications, identify maturity
   as specification-only, and give a runnable documentation verification command.
5. Contribution guidance inherits organization issue-first and security policy.
6. CI validates the specification hierarchy, relative documentation links,
   required public files, and documentation source references.
7. Git ignores authentication, local environment files, and runtime sessions.
8. The initial commit is published to the public organization repository with
   `main` as its default branch. Runtime packaging and deployment are excluded.
