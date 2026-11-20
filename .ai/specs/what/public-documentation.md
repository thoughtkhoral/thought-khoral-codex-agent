# Codex worker documentation requirements

## Status and authority

Approved documentation scope, refreshed by the maintainer's 2026-11-20 request
to update documentation specifications before derived pages at workspace and
individual-project level. This supersedes stale scaffold/MVP-only documentation
criteria, without changing approved runtime requirements or acceptance gates.

Read the [local specification index](../README.md), [component requirements](codex-chat-agent.md)
and [implementation design](../how/codex-chat-agent.md) first. Root
[documentation requirements](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/what/open-source-documentation.md)
and [current workspace authority](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/workspace-state.md)
apply. Documentation cannot approve implementation, publication or activation.

## Maintained documentation set

| Page | Required role |
| --- | --- |
| [Repository README](../../../README.md) | Purpose, maturity, prerequisites, valid commands, compatibility and source links. |
| [Project documentation index](../../../docs/README.md) | Local documentation entrypoint mapping maintained pages to their governing specs and checkout applicability. |
| [Component status](../../../docs/project-status.md) | Independent worker implementation and remaining cross-project gates. |
| [Architecture](../../../docs/architecture.md) | Worker/gateway boundaries and approved data flow. |
| [Worker runtime](../../../docs/runtime.md) | Authenticated transport, durable receipts and native session responsibilities. |
| [Adapter](../../../docs/adapter.md) | Provider-free adapter boundary and pinned protocol. |
| [Native Linux acceptance](../../../docs/linux-acceptance.md) | Exact native/compiler/package evidence and portability limits. |

## Acceptance criteria

- Update applicable documentation specs and accepted evidence first, then
  maintain these authored Markdown pages. CI validates; it does not generate
  prose or rewrite specifications.
- Map each maintained page to its governing local What/How, and link from the
  repository README to this documentation specification and the project index.
- Identify canonical runtime versus prepared-branch applicability. Documentation
  HEADs, published artifacts and frozen runtime acceptance inputs are distinct;
  do not repin a tested source because a prose-only followup advanced HEAD.
- Describe the implemented experimental provider-free worker; original scaffold-only wording is historical, and draft future guidance is not runtime authority.
- Explain native sessions, durable receipts, model/effort and context reporting under the approved runtime How; keep room storage outside the worker.
- Give actual Rust/Python and documentation verification prerequisites and point to AGENTS.md-required checks, licensing and third-party notices.
- Distinguish published conversation contracts from unchanged compatible worker pins, and native/package evidence from assembled-stack, browser, account and provider acceptance.
- Preserve historical evidence and existing anchors. Attribute results to exact
  source/image/contract inputs and distinguish implemented, verified, published,
  enabled and production-ready states.
- Keep private configuration, provider keys, human tokens, native sessions and
  room contents outside docs and fixtures. Preserve licenses and legal notices.
- Direct contributions through the issue/specification workflow and private
  security-reporting policy. Validate links, source references and whitespace;
  run any additional checks required by this project's agent instructions.

## Verification boundary

Documentation checks establish discoverability and consistency, not runtime,
network, browser or provider acceptance. Fresh verification must name what was
actually run; retained results keep their original source identity. Current
state comes from approved component/root evidence, not the age of this file.
