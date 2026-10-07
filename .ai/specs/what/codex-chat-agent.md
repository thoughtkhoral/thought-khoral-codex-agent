# Codex chat conversations

## Status

Approved by the project maintainer in the Codex working session on 2026-10-05,
including this milestone-one specification and the coordinated implementation
plan. Accepted contribution: [issue 1](https://github.com/thoughtkhoral/thought-khoral-codex-agent/issues/1).
Implementation follows the [plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md) and its dependency gates.
Release/tag publication, provider use and service activation require their
separate later authorization. No completed runtime or live verification is claimed.

## Purpose

Let humans include a real Codex participant in ThoughtKhoral room conversations.
When explicitly addressed, Codex understands the authorized conversation among
humans and agents, including messages exchanged while Codex was not responding.
Humans can explicitly start a new Codex thread or continue its saved history.
Use this first integration to establish an independent agent implementation
boundary suitable for future bring-your-own-agent support.

## User behavior

1. A human selects or directly mentions the registered Codex Agent in the room
   composer. Submission through the conversation API invokes one turn; ordinary
   retained chat.send does not initiate Codex inference. Ordinary messages,
   quoted mention text, aliases, and messages from agents do not invoke it.
2. Without an active conversation, the composer offers **Start Codex session**.
   The first submitted prompt creates the session and starts its first turn.
3. With an active room conversation, **Continue session** is the default for
   every authorized human. **New session** resets the room's shared Codex thread;
   its label explains the room-wide effect. It receives the authorized room
   transcript again, including prior public Codex replies. It clears native
   thread state, not the room's conversation history.
4. The chat shows the accepted prompt, queued/running state, and final Codex
   reply or a safe error. The initiating human, Codex identity, shared room
   session, and room-visible delivery are clear.
5. Reloading the browser or restarting services preserves the active
   conversation when both the mapping and Codex session storage are available.
6. Missing or unusable history produces a visible session error and an option
   to start fresh. The system never silently substitutes a new conversation.
7. The composer always shows the selected model and reasoning effort, including
   before the first turn. Model and effort selectors are available before a
   fresh session and between turns in an existing session. Changes preserve
   conversation history and apply on the next accepted turn.
8. The session header shows runtime-confirmed model and effort. A pending
   selection is labelled **Next turn** until accepted by the runtime; the
   active turn's settings remain visible. After reload, settings come from the
   server rather than browser defaults. Unsent choices are local to the
   composer; the next accepted turn atomically establishes the shared defaults.
9. When supported, show **Estimated context used: N%**, with a tooltip stating
   that it reflects the last reported model request, plus tokens/window and
   update time. Before the first report, or when window size is unavailable,
   show **Context usage unavailable**. It is not a billing or quota meter.
10. Codex can answer about authorized room messages from other humans even if
    none previously invoked Codex. Messages arriving after a turn's snapshot
    boundary become context on the next explicit invocation.
11. Targeted delivery to Codex is rejected in this initial shared mode. The UI
    explains that its responses and session are room-wide.
12. Context that exceeds the configured baseline/delta limit produces a visible
    context error. The system does not silently discard history or claim that
    a truncated input represents the complete room transcript.

## Scope

- One locally controlled, model-backed Codex Agent with a `chat` capability.
- An independently versioned `thought-khoral-codex-agent` repository and runtime
  consuming shared integration contracts through the agent gateway.
- One active conversation per room and agent, shared by authorized humans,
  approved for milestone one. Each task retains its authenticated requester identity.
- Authorized room-wide transcript and active decisions supplied by the room
  gateway, with ordered provenance and full-baseline/continuation semantics.
- Native Codex history: previous human prompts and Codex turns, including any
  native compaction. This does not promise verbatim recall indefinitely.
- Room-visible requests and replies; selecting Codex is explicit and ordinary
  room messages do not invoke it.
- Conversational assistance in a read-only, isolated runtime.
- Explicit fresh/continue choice, idempotent requests, serialized turns,
  durable history, observable failures, and local development deployment.
- Model selection, model-specific reasoning effort, visible confirmed settings,
  and context-window telemetry when available.

## Exclusions

Importing personal Codex sessions; browsing or switching among archived native
threads; private per-human Codex conversations; targeted inputs or outputs;
autonomous agent responses; automatic memory-engine retrieval; code modification;
interactive approvals; token-level streaming; cancellation UI; general external
agent admission; production rollout. Directory-based specifications and curated
memory are a [future extension](guided-workspace.md), not a release dependency.

## Acceptance criteria

1. A first chat request launches headless Codex and returns its final reply.
2. A follow-up by another authorized human uses the same recorded Codex thread
   and can answer a question requiring information from the room discussion,
   including intervening messages that did not invoke Codex.
3. **New session** produces a distinct thread, excludes the old native thread,
   and receives a new authorized room transcript baseline. Old room-visible
   replies remain room history; another room's material is never included.
4. The active conversation and follow-up behavior survive browser reload and
   an ordinary worker restart with its session volume retained.
5. A human outside the room or without invocation permission cannot resume its
   conversation, even with a known identifier. Targeted messages are absent
   from baseline, delta, native thread inputs, citations, and room-visible replies.
6. Duplicate client requests create one task and one logical Codex turn;
   concurrent requests cannot interleave writes to the same Codex thread.
7. Failed authentication, missing executable/history, malformed output,
   timeout, and uncertain crash recovery produce safe visible failures.
8. An incomplete or failed Codex turn is never displayed as a successful reply.
9. Existing deterministic agent tasks continue to pass their current gates.
10. Only runtime-catalog models allowed by deployment policy and their supported
    effort values can be selected. Catalog membership does not guarantee account
    access; a denied model produces a safe error without silently substituting
    a different model. Changing models requires an explicit effort selection or
    acknowledgement of the new model's displayed default if the previous effort
    is unsupported.
11. Changing model or effort between turns retains the same Codex thread and
    history. Accepted settings persist and are restored after reload/restart;
    each completed reply records which settings were used.
12. Context usage is derived from the latest reported request, never summed
    across turns. Fresh sessions reset it to unavailable, model switches mark
    it stale, and compaction refreshes or invalidates it. Missing, invalid,
    zero-window, or mismatched-thread telemetry cannot produce a percentage.
13. The Codex agent can be built and released separately from the gateway;
    gateway invocation uses a versioned protocol, with no dependency on Codex
    subprocess or session-file implementation. Other agents may omit model,
    effort, or context-usage capabilities, and the UI handles their absence.
14. Direct mention and composer selection of the same Codex participant create
    one task. Aliases, quoted mentions, targeted delivery, and agent-originated
    messages cannot cause unsolicited turns or recursive agent conversations.
15. Every disclosure is task-bound and revision-bound. The current invocation
    text occurs once; continuation includes new eligible entries once, including
    messages from multiple humans, while sequence gaps over hidden events are
    valid. The existing native assistant reply is bound to its accepted room
    event rather than reinjected as a second assistant turn.
16. A stale context base, lost execution receipt, invalid authorization binding,
    changed visibility policy, or uncertain runtime mutation fails visibly.
    A caller losing membership cannot continue; a room thread containing newly
    forbidden history cannot be reused even by a remaining member.
17. History-size limits are explicit and tested at the boundary. Failure occurs
    before model submission when a complete authorized baseline/delta cannot
    be represented. Later compaction or retrieval requires an approved design.

## Governing proposal

[Decision 002](../decisions/002-independent-codex-agent.md) and
[technical design](../how/codex-chat-agent.md), including the proposed
[conversation integration profile](../how/conversation-integration-profile.md).
