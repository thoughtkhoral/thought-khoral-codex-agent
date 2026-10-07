# Specification and memory guided agent workspace

## Status

Draft future direction — separate from the initial history-aware chat release.
The user requested a directory that can guide conversation activity using
specifications and memory on 2026-10-02. Detailed requirements below are proposed,
not authorization to implement filesystem access or a memory integration.

## Purpose

Let an operator configure Codex's working directory with approved instructions,
specifications, and curated memory so its room participation follows project
intent and established knowledge beyond the model's built-in knowledge.

## Proposed behavior

- An authorized operator selects a reviewed guidance bundle for a room/agent;
  ordinary chat participants cannot supply a filesystem path or install guidance.
- A bundle contains an instruction entry point, project specifications, and
  curated memory with source references. All contents have the same room-wide
  disclosure scope as the shared conversation.
- The UI identifies the effective guidance revision. Codex can explain which
  supplied guidance supports a reply without exposing private infrastructure.
- Guidance is read-only and versioned. A revision change creates a fresh native
  thread and authorized room baseline, so the old instruction set does not
  silently persist in native history.
- Curated memory records durable facts with sources and review history. The
  initial extension does not let Codex promote its own responses into facts or
  modify approved specifications. A human review workflow owns future writes.

## Boundary and precedence

Separate the guidance working directory from provider configuration, CODEX_HOME,
native session files, execution receipts, and transient task context. Mount only
the reviewed immutable bundle, not an operator home or arbitrary host repository.
Bind its ID and content digest to the room, agent, generation, and receipt.

Platform authorization, isolation, and tool policy remain enforced outside the
model. Approved agent instructions define behavior within that policy. Project
specifications guide the subject of the conversation; curated memory provides
evidence. Room messages are discussion data and cannot elevate themselves to
approved instructions. Guidance conflicts or unavailable evidence must be
reported rather than hidden behind an invented answer.

## Acceptance criteria for the later extension

1. Two rooms with different bundles cannot read each other's guidance or memory.
2. The effective immutable revision survives reload and worker restart.
3. A bundle change visibly starts a new generation and preserves authorized
   room context without importing the prior native thread.
4. Missing, modified, oversized, or unapproved bundle contents fail before use;
   the worker never falls back to operator configuration or an arbitrary path.
5. A controlled evaluation compares identical room questions with and without
   guidance and demonstrates use of a supplied project constraint and sourced
   fact, rather than merely assuming the directory changed model behavior.
6. Guidance cannot widen tools, network access, room visibility, or write rights.
7. Replies distinguish supplied evidence from inference; citations identify
   admitted bundle source/revision and do not imply room-event provenance.

## Future design gate

Before implementing this extension, approve its bundle format, publication and
review workflow, source provenance, retention/deletion policy, size/token limits,
instruction-loading mechanism for the pinned Codex version, and any explicit
memory-engine contract. Directory presence alone does not guarantee instruction
loading or evidence use. Automatic retrieval and autonomous memory writes remain
outside this proposal.

Governing proposal: [Decision 002](../decisions/002-independent-codex-agent.md)
and [runtime guidance boundary](../how/codex-chat-agent.md).
