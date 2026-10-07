# Visible server defaults before the first Codex turn

Status: approved by the maintainer in this conversation on 2026-10-07, in
response to the explicit request to approve this amendment as the next
implementation scope. This authorizes coordinated local implementation and
synthetic verification. Contract publication, provider use, service activation,
merge and push retain their separate authorization gates.

## Problem and governing requirement

The [approved runtime design](codex-chat-agent.md#model-and-reasoning-effort-controls),
inherited from the root How, requires the UI to display resolved default selections
before the first prompt. Published `thought-khoral-agent-conversation-v1.0.0`
returns no selected settings when a room has no conversation; its catalog has
per-model default effort but no deployment-default model. Selecting the first
catalog entry would invent a server default. The compatible interim safety
correction requires an explicit visible model/effort choice before invocation.

Final review also found F1: when admission supports reasoning effort but omits
model selection, the fresh-selection guard requires a model that the UI cannot
select. Initial and reset submissions are blocked. Optional capabilities are
independent; the proposed discovery mechanism must support each combination.

## Approaches considered

| Approach | Benefit | Cost |
|---|---|---|
| Authenticated broker defaults query (recommended) | Uses authoritative policy and catalog; supports reload and reset without duplicated configuration | Adds one read endpoint and coordinated contract pins |
| Public host configuration for the UI | Avoids an extra query | Duplicates policy, introduces synchronization/versioning work, and still needs a configuration contract |
| Keep explicit manual selection permanently | Smallest interface change | Requires amending the approved automatic-default-display requirement; every fresh session needs a human choice |

## Approved design

Add an authenticated, read-only human-facing defaults query to immutable
`thought-khoral-agent-conversation-v1.1.0`, without changing the published
v1.0.0 bytes. The profile string and namespace remain v1 because this is an
additive query; existing clients and schemas remain compatible. The proposed
route is `GET /api/agent-conversations/v1/rooms/{roomId}/agents/{agentId}/defaults`.
Its new `ResolvedSettingsView` JSON schema disallows unknown fields and requires
only `profileVersion: "thought-khoral.agent-conversation.v1"`, UUID `roomId`, UUID
`agentId`, and `selectedSettings`. That object uses the existing exact model,
reasoning-effort and catalog-revision constraints; it contains no effective
settings or runtime confirmation because no turn has run.
The owning broker resolves the deployment pair through its current authenticated
catalog bridge and allowlist. It checks the same room authority, capability
admission and token bounds as the catalog/conversation query. Empty catalog,
removed model, unsupported effort and unavailable runtime fail with existing safe
codes; there is no model fallback.

The defaults query uses the existing closed `ProfileError` shape and HTTP
mapping: 400 for malformed request identifiers, 401 for missing/invalid human
authentication, 403 for denied authority, 404 for unavailable room/agent scope,
and 503 `runtime_unavailable` when admission, catalog access or the configured
default pair cannot be established. Since this read creates no task, it never
returns a terminal `TaskView` execution failure. Model entitlement remains
unproven until an authorized invocation; known inference failures retain their
existing terminal codes. Server logs and public errors contain no raw provider
response or credential.
Use `Cache-Control: no-store` and the current five-second settings-validation
bound; no unbounded catalog traversal or retry loop is added. Before displaying
the resolved pair, the UI verifies that its loaded catalog revision and allowed
model/effort match the response. A mismatch requires bounded refresh or a visible
unavailable state, never a guessed selection.

This query creates no conversation, task, native thread, human event or lease.
It exposes no credential, native identifier or private room content. Fresh New
without settings still uses the broker's deployment defaults. Continuation
without settings uses persisted shared accepted defaults; accepted replay retains
its original pair.

The UI fetches this pair when no conversation exists or the human explicitly
requests New/reset, and settings controls are supported. It displays the
concrete model and effort as the next-turn
selection and submits that explicit pair, including its catalog revision. Human
changes stay local until accepted. If the catalog revision changes or the pair
becomes disallowed between display and submission, the server rejects it and
the UI refreshes for human review; it never silently picks another model. A
deployment-default-only change does not make an explicitly submitted still-valid
pair stale: the displayed explicit pair continues to govern. A restored conversation uses its
persisted server pair. Capabilities without model/effort support do not call or
show these controls.

For reasoning-only admission, show the broker-resolved model as read-only and
enable the effort selector for that model. For model-only admission, enable the
model selector and show the resolved effort as read-only; a model change uses
that model's catalog-defined default effort, displayed before submission. Both
controls remain editable when both capabilities are present. A hidden control
must never leave a required selection impossible to establish. In each supported
combination, the displayed concrete pair is submitted explicitly and validated
against the catalog. No capability combination authorizes guessing the model
from catalog order or adding an unsupported editable control. If the authoritative
pair cannot be resolved, show an explicit unavailable reason and block invocation.
Agents with neither settings capability preserve the existing settings-free path.

## Contract and approval boundaries

The contract owner defines the endpoint, exact schema, safe failures, semantic
validator and fixtures in a new versioned immutable artifact. Version selection
and consumer pin updates belong to that coordinated minor-version change. Existing v1.0.0
clients remain compatible with the additive route; existing schemas and fixture
bytes remain unchanged. Publication still needs its separate authorization.

The approved amendment governs the root How/plan and owning contracts/broker/UI
How specifications. Update those sources before implementing the mechanism.
Build a reproducible, immutable local v1.1.0 candidate artifact from an exact
committed contracts revision, with archive and per-file SHA-256 verification.
Consumers may pin this explicitly unreleased candidate on isolated branches
for synthetic verification; it must not be represented as a published release
or shipped before the separate publication and release review gates. Preserve
the published v1.0.0 artifact and its provenance unchanged. This local candidate
exception to the earlier published-artifact execution order is approved only
for this amendment's pre-publication development and tests.

## Required acceptance evidence

- In a never-used room, place the policy-default model after another catalog
  entry. Show the actual policy model/effort before any prompt, and prove the
  first accepted/native-request pair exactly matches the display.
- Prove the query is read-only and authenticated, rejects unsupported or stale
  selections without fallback, and does not expose native/private data.
- Change catalog revision or invalidate the pair between display and submit;
  prove safe rejection and explicit refresh rather than substituted inference.
  Change only deployment defaults while the explicit pair remains valid; prove
  the shown explicit pair is submitted without substitution.
- Preserve restored shared defaults, explicit reset semantics, immutable replay,
  existing contract pins, and capabilities that omit settings controls.
- Cover both capabilities, reasoning-only, model-only and neither capability for
  initial invocation, restored continuation and explicit reset. Prove supported
  combinations can establish and submit the displayed pair without inaccessible
  required controls, and unresolved defaults show an unavailable reason. Include
  a regression that reproduces and resolves the F1 reasoning-only deadlock.

Risk: this adds one public read endpoint and a coordinated contract release. The
interim selection guard remains necessary when defaults cannot be discovered.

## Later local integration authorization — 2026-10-07

The user later authorized a local merge of the reviewed provider-free agent and
worker history into this repository's `main`. This updates the merge gate only
for this repository. It does not authorize merging the separate contracts,
broker, mediator, UI or platform repositories, publishing the v1.1.0 candidate,
or activating a service or provider.
