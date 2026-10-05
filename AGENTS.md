# Agent instructions

Read [.ai/specs/README.md](.ai/specs/README.md) before changing this repository.
Specifications are the source of truth for implementation and documentation.

- Follow approved What, How, and decision records. Draft runtime specifications
  do not authorize implementing runtime code. Update and obtain approval for
  applicable specifications before changing behavior.
- Inherit [root workspace governance](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/README.md).
  Parent overrides need an accepted local decision with rationale and scope.
- Keep this an independent repository. Do not embed Codex runtime code in the
  gateway or add direct room-database access.
- Maintain documentation from specifications and link each document to its
  governing local source. Never use documentation to override a specification.
- Keep provider credentials, local configuration, Codex session files, and
  private room contents outside version control and test fixtures.
- Preserve Apache 2.0 licensing and third-party notices.
- Run `python3 scripts/check_docs.py` and `git diff --check` before completion.
  Report verification accurately; Task 4 supplies a provider-free adapter library,
  with no worker server or live activation.

Governing specification: [repository foundation](.ai/specs/how/repository-foundation.md).
