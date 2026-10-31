# CLI requirements

These IDs describe durable behavior. Passing observations live separately in
`.chough/evidence/`, tied to repository content and environment. FRD provenance
and scope live in [docs/FRD.md](../../docs/FRD.md).

### REQ-cli-001 — Preserve user files

The helper SHALL initialize a resumable session and discoverable workflow skill
without replacing existing files. Trace: LIFE-4, LIFE-6.

Acceptance: repeated init preserves edited session and skill bytes; missing bundle
files can be added without overwriting neighbors.

### REQ-cli-002 — Focus context for resumption

The helper SHALL assemble applicable instructions, scoped HI export, selected
module contracts/companions, objective, decisions, open questions and next action.
Trace: LIFE-3, LIFE-6.

Acceptance: the same inputs yield the same ordered context; missing sources and
truncation remain explicit. Native module .3md is the lesson source where present.

### REQ-cli-003 — Delegate lifecycle state

The helper SHALL report actual native SpecSync status and guidance, with unavailable
or undecodable state explicitly unknown. Trace: LIFE-1, LIFE-6, LIFE-7.

Acceptance: missing SpecSync never becomes an approved or delivered state. An
agent-written session next action never grants approval or overrides a native gate.

### REQ-cli-004 — Bind observations to content and environment

The helper SHALL bind each observation to Git revision, working content identity,
and an explicit environment, and report freshness limitations. Trace: LIFE-2, LIFE-7.

Acceptance: changing observed content makes prior observations stale; another
environment is mismatched; no chosen environment is unchecked. Recording another
observation or editing the session does not self-invalidate the content identity.

### REQ-cli-005 — Validate traceability

The helper SHALL validate observation criterion IDs, requirement IDs and contract
paths against their native records before recording. Trace: LIFE-1, LIFE-2, LIFE-5.

Acceptance: unknown active HI IDs or absent native requirement IDs are refused;
observation IDs cannot overwrite prior observations. Observations are unauthenticated
claims, including those labelled human or delivery observations.

### REQ-cli-006 — Bound local input

The helper SHALL bound assembled context and reject unsupported records or paths
escaping the project. Trace: LIFE-3, LIFE-4.

Acceptance: traversal, invalid module/record IDs and symlink escape attempts fail;
large context is visibly bounded. Local artifacts remain reviewable plain files.
