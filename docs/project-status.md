# Codex worker component status

**Status as of 2026-10-07: experimental provider-free POC.** This page covers
the Codex worker owned by this repository. For the end-to-end goal, project
responsibilities, evidence limits, remaining milestone-one gates, and the later
specification/memory-guided milestone, read the [cross-project ThoughtKhoral
status guide](https://github.com/thoughtkhoral/thought-khoral/blob/main/docs/codex-conversation-status.md).
Approved What/How specifications and the [coordinated implementation
plan](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/codex-room-conversations-implementation-plan.md)
govern requirements and execution.

## Current component

The independent worker includes a provider-free Codex app-server adapter,
authenticated gateway transport, durable execution receipts, and native
thread/session handling. It does not access room storage directly. Provider
authentication and service activation are separately configured and gated.

Provider-free worker and package verification is recorded in this repository's
[runtime specification](../.ai/specs/how/codex-chat-agent.md) and the coordinated
plan. This evidence does not establish a live provider conversation, packaged
deployment, Linux x86_64 parity, or production readiness.

## Future guided workspace

The [guided-workspace What](../.ai/specs/what/guided-workspace.md) remains a
draft future direction. It proposes reviewed immutable guidance bundles and a
separately governed memory-engine interface; it does not authorize filesystem
access or memory integration today.
