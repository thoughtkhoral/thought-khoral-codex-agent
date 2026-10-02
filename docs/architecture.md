# Proposed Codex agent architecture

**Draft runtime design; no runtime is implemented.** This document derives from
the local specifications and does not authorize implementation.

## Responsibilities

| Component | Responsibility |
| --- | --- |
| Room gateway | Human authorization, platform conversation ownership, durable tasks and room events |
| Agent gateway | Reviewed admission, authenticated invocation, context mediation, validated updates |
| This Codex agent | Codex subprocesses, native history, provider credentials, model/effort discovery, usage normalization |
| Contracts | Versioned integration profile and compatibility fixtures |
| Workspace UI | New/continue controls and agent-supported settings and telemetry |
| Platform | Optional deployment, persistent storage, local credentials, restricted egress |

The proposed agent wraps `codex app-server` over stdio, creates or resumes an
explicit thread, and submits one reserved turn at a time. Native thread IDs
remain internal. Provider credentials and history belong to the independently
operated agent, while ThoughtKhoral controls invocation and disclosed context.

## Planned user experience

Humans can start fresh or continue their conversation. Model and reasoning
effort can change between turns without losing its history. Effective settings
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
uncertain interrupted turns are not blindly executed again. Missing history
produces a visible error rather than silently substituting a fresh thread.

Source: draft [chat requirements](../.ai/specs/what/codex-chat-agent.md),
[runtime design](../.ai/specs/how/codex-chat-agent.md), and
[independent-agent decision](../.ai/specs/decisions/002-independent-codex-agent.md).
