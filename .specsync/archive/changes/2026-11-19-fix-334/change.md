---
id: fix-334
state: archived
type: bug_fix
base_commit: af8ca8901b2a23affcd40ac74865d99637448346
---

# Fix 334

## Intent

fix 334

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With CHOUGH_SESSION set in a project with no open session, a command refuses as refused:session-not-open naming CHOUGH_SESSION and chough session new <key>, with nothing written; with --session the refusal names --session; a name that is not open while other sessions are still names chough session list, as before.

## No-spec Rationale

Not applicable
