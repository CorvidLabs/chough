---
id: keep-finished-sessions-with-open-human-gates-open-on-session-use-new-naming-doctor-s-decision-fix-181
state: draft
type: bug_fix
base_commit: 2981a59cd4a545c7ce38421c72e53469993194cd
---

# Keep finished sessions with open human gates open on session use/new, naming doctor's decision fix (#181)

## Intent

Keep finished sessions with open human gates open on session use/new, naming doctor's decision fix (#181)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- A finished session (stage done or native change finalized on HEAD) with a non-empty awaiting_human list and a finalized native change stays open on session new and session use and is reported in kept_open with its open-gate count and the recovery 'chough doctor --fix --reason <text>'; a finished session without open gates is archived unchanged as before; the existing inbox-unfolded kept-open behavior is unchanged. Regression: a switch with one gated finalized session and one ungated finalized session archives only the ungated one.

## No-spec Rationale

Not applicable
