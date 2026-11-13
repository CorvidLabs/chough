---
id: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
state: approved
type: bug_fix
base_commit: 25af1398b2627c339a5ce0a0b3f3cd78a79e261c
---

# Fix #246: chough migrate's preview and receipt name the sessions chough doctor will report after the apply

## Intent

Fix #246: chough migrate's preview and receipt name the sessions chough doctor will report after the apply

## Affected Canonical Specs

- `records`
- `cli`

## Acceptance Criteria

- chough migrate's preview and its receipt's Follow-up name each session chough doctor reports right after the apply: finished_session ones to archive with chough doctor --fix, and stale_gates ones with each open gate's text and chough doctor --fix --reason, computed by doctor's own session check on the planned records (red then green)

## No-spec Rationale

Not applicable
