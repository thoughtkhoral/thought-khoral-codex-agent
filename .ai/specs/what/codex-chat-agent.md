# Codex chat conversations

## Status

Draft — proposed for maintainer review. Session ownership and initial tool
scope are proposed defaults, pending human review.

## Purpose

Let a human invoke a real Codex agent from ThoughtKhoral chat, explicitly start
a new conversation, and continue previous turns using Codex's saved history.
Use this first integration to establish an independent agent implementation
boundary suitable for future bring-your-own-agent support.

## User behavior

1. A human selects or mentions the registered Codex Agent in the room composer.
2. Without an active conversation, the composer offers **Start Codex session**.
   The first submitted prompt creates the session and starts its first turn.
3. With an active conversation, **Continue session** is the default. The human
   can select **New session** to send the next prompt without old Codex turns.
4. The chat shows the accepted prompt, queued/running state, and final Codex
   reply or a safe error. Ownership and room-visible delivery are clear.
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
   server rather than browser defaults.
9. When supported, show **Estimated context used: N%**, with a tooltip stating
   that it reflects the last reported model request, plus tokens/window and
   update time. Before the first report, or when window size is unavailable,
   show **Context usage unavailable**. It is not a billing or quota meter.

## Scope

- One locally controlled, model-backed Codex Agent with a `chat` capability.
- An independently versioned `thought-khoral-codex-agent` repository and runtime
  consuming shared integration contracts through the agent gateway.
- One active conversation per human, room, and agent, pending review.
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

Importing personal Codex sessions; browsing or switching among archived
conversations; shared session ownership; targeted delivery; automatic room
memory retrieval; code modification; interactive approvals; token-level
streaming; cancellation UI; general external agent admission; production rollout.

## Acceptance criteria

1. A first chat request launches headless Codex and returns its final reply.
2. A follow-up uses the same recorded Codex thread and can answer a question
   requiring information supplied in the first turn.
3. **New session** produces a distinct thread and does not inject old Codex
   turns or unrelated room history into the new prompt.
4. The active conversation and follow-up behavior survive browser reload and
   an ordinary worker restart with its session volume retained.
5. Another human or room cannot resume the owner's conversation, even by
   submitting a known conversation identifier.
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

## Governing proposal

[Decision 002](../decisions/002-independent-codex-agent.md) and
[technical design](../how/codex-chat-agent.md).
