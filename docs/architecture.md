# Codex agent architecture

## Current workspace context — 2026-11-20

Native AMD64/ARM64 worker acceptance remains tied to tested source `9bcdbde` and its final lock. The 2026-11-20 provider-free ARM64 image from source `3f0318b` passed ten package/startup probes and exact native policy recapture: 44 model/effort cases, eight models, resume and six refusals. Published contract v1.1 and locally prepared released consumers/profile do not add provider, assembled-stack, browser, registry-image or production acceptance. Worker/mediator v1.0 pins remain compatible and unchanged.

Runtime source and contract pins are unchanged. Exact clean pinned-source
snapshots were retained before this documentation-only checkout advanced.

Derived from the [local specification index](../.ai/specs/README.md) and
[root workspace authority](https://github.com/thoughtkhoral/thought-khoral/blob/main/.ai/specs/how/workspace-state.md).
The cited tests retain their original source identities; this documentation
refresh adds no activation, publication or runtime acceptance.

**Approved milestone-one design; experimental provider-free agent/worker and reviewed room-integration POC commits are pushed to owning GitHub `main` branches.**
The reviewed integration was pushed on 2026-10-07. Exact commits and
provider-free evidence are recorded in the coordinated plan. The v1.1 contract
is now published; released consumers/profile are verified locally. Packaged-stack and separately authorized live checks
remain pending. This is not a production-readiness claim. This document derives
from the local specifications and records the separate integration/release gates.

The later approved [native Linux acceptance](../.ai/specs/how/linux-worker-acceptance.md)
records successful AMD64/ARM64 runtime tests on Rust 1.85.0/1.93.1 and exact
package-policy recaptures at the final lock. It closes worker portability gates;
full Codex room-stack, rendered-browser, provider and production gates remain open.

## Responsibilities

| Component | Responsibility |
| --- | --- |
| Room gateway | Human authorization, shared room conversation identity, authorized transcript snapshots/deltas, durable tasks and room events |
| Agent gateway | Reviewed admission, authenticated invocation, context mediation, validated updates |
| This Codex agent | Codex subprocesses, native history, provider credentials, model/effort discovery, usage normalization |
| Contracts | Versioned integration profile and compatibility fixtures |
| Workspace UI | New/continue controls and agent-supported settings and telemetry |
| Platform | Optional deployment, persistent storage, local credentials, restricted egress |

The agent wraps `codex app-server` over stdio, creates or resumes an
explicit room thread, and submits one reserved turn at a time. Native thread IDs
remain internal. Provider credentials and history belong to the independently
operated agent, while ThoughtKhoral controls invocation and disclosed context.

## Approved conversation behavior and verification boundary

Codex responds when a human directly addresses it by canonical mention or composer
selection. It receives authorized room-wide discussion, including messages from
other humans and messages that did not invoke it. Aliases and agent replies do
not invoke it, and targeted messages are excluded from the shared thread.

The room session is shared by authorized humans in the room. Starting fresh
replaces its native thread and supplies a new room-history baseline; it does
not erase the room transcript. Continuation supplies ordered updates with source
IDs and a revision boundary. Model and reasoning effort can change between turns
without losing native history. Effective settings
are visible and restored after reload. Context usage is a labelled estimate
from the last bound runtime report, or unavailable when telemetry is missing.

## Bring your own agent

The Codex implementation is independently versioned from the gateway. Other agents
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

The exact integration profile is an approved coordinated design; the local
profile records worker-specific obligations. The provider-free agent/worker,
gateway, broker, UI, and platform integration is pushed to each owning GitHub
`main` for experimental POC use. The v1.1 artifact is published at `f1c8621`;
released consumer/profile branches remain local and unmerged.
Packaged-stack activation and live-provider interoperability remain unverified.
The implemented controls have provider-free synthetic evidence; the text above
describes the approved behavior and does not assert live browser/provider acceptance.

Source: approved [chat requirements](../.ai/specs/what/codex-chat-agent.md),
[runtime design](../.ai/specs/how/codex-chat-agent.md), and
[independent-agent decision](../.ai/specs/decisions/002-independent-codex-agent.md),
[integration profile](../.ai/specs/how/conversation-integration-profile.md), and
[future guidance requirements](../.ai/specs/what/guided-workspace.md).
