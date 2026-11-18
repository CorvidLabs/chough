---
id: fix-325
state: archived
type: bug_fix
base_commit: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
---

# Fix 325

## Intent

fix 325

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- While another process holds the project's writer lock, chough evidence add writes no entry while it waits, and when it refuses as refused:checkpoint-busy after the wait every file is unchanged, so a rerun records one entry, not two. cases run and cases record take the lock before writing their entry too, after any case has run.

## No-spec Rationale

REQ-cli-040 already says a writer that cannot take the lock within 30 seconds refuses as refused:checkpoint-busy with nothing saved; only the evidence save's lock order and its test change.
