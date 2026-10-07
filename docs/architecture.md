# Codex agent architecture

**Approved milestone-one design; this checkout remains the scaffold.**
Reviewed worker and mediation implementations live on isolated local branches;
UI and opt-in packaging are also committed locally; Task 9 synthetic verification
and runtime corrections are also reviewed locally. Important UI finding F1
and resolved-default discovery are accepted on reviewed local synthetic candidate
branches. Contract publication, whole packaged-stack and authorized live checks
remain pending. This document derives from
the local specifications and does not authorize implementation.

## Responsibilities

| Component | Responsibility |
| --- | --- |
| Room gateway | Human authorization, shared room conversation identity, authorized transcript snapshots/deltas, durable tasks and room events |
| Agent gateway | Reviewed admission, authenticated invocation, context mediation, validated updates |
| This Codex agent | Codex subprocesses, native history, provider credentials, model/effort discovery, usage normalization |
| Contracts | Versioned integration profile and compatibility fixtures |
| Workspace UI | New/continue controls and agent-supported settings and telemetry |
| Platform | Optional deployment, persistent storage, local credentials, restricted egress |

The proposed agent wraps `codex app-server` over stdio, creates or resumes an
explicit room thread, and submits one reserved turn at a time. Native thread IDs
remain internal. Provider credentials and history belong to the independently
operated agent, while ThoughtKhoral controls invocation and disclosed context.

## Planned user experience

Codex responds when a human directly addresses it by canonical mention or composer
selection. It receives authorized room-wide discussion, including messages from
other humans and messages that did not invoke it. Aliases and agent replies do
not invoke it, and targeted messages are excluded from the shared thread.

The proposed session is shared by authorized humans in the room. Starting fresh
replaces its native thread and supplies a new room-history baseline; it does
not erase the room transcript. Continuation supplies ordered updates with source
IDs and a revision boundary. Model and reasoning effort can change between turns
without losing native history. Effective settings
are visible and restored after reload. Context usage is a labelled estimate
from the last bound runtime report, or unavailable when telemetry is missing.

## Bring your own agent

The Codex implementation is released separately from the gateway. Other agents
may implement the shared profile and declare different optional capabilities.
The initial integration admits one reviewed agent; arbitrary user-hosted URLs
and self-service enrollment need their own trust and authorization design.

## Memory and failures

Sessions require durable native files and conversation mappings. Turn execution
is serialized and bounded. Completed receipts can replay accepted results;
uncertain interrupted turns are not blindly executed again. The room gateway
commits the context cursor with the accepted reply; execution receipts reconcile
that reply before another turn. Policy changes that make saved history forbidden
invalidate the thread. Missing history
produces a visible error rather than silently substituting a fresh thread.

## Future working directory

A later extension can supply a reviewed, immutable bundle of instructions,
specifications, and curated memory in the agent's working directory. Its revision
is bound to the room session; changing it requires a fresh native thread. Initial
chat uses fixed instructions and does not write memory or mount a host repository.

## Approval boundary

The exact integration profile now lives in the contracts repository as a
approved coordinated design; the local profile records worker-specific obligations.
The maintainer accepted the six contribution issues and approved root/local
specifications and the plan on 2026-10-05. Consumer runtime work follows
publication of a verified immutable contracts artifact and the plan dependency
gates. No runtime implementation or published interoperability is claimed.

Source: approved [chat requirements](../.ai/specs/what/codex-chat-agent.md),
[runtime design](../.ai/specs/how/codex-chat-agent.md), and
[independent-agent decision](../.ai/specs/decisions/002-independent-codex-agent.md),
[integration profile](../.ai/specs/how/conversation-integration-profile.md), and
[future guidance requirements](../.ai/specs/what/guided-workspace.md).
