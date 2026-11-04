---
id: trim-old-proof-automatically-when-a-change-reaches-done-with-the-librarian-on-and-let-retention-manual-opt-out-issue-88
state: approved
type: bug_fix
base_commit: 2896f5ada1947a307b9958007a109c06dc3a2831
---

# Trim old proof automatically when a change reaches done with the librarian on, and let retention manual opt out (issue 88)

## Intent

Trim old proof automatically when a change reaches done with the librarian on, and let retention manual opt out (issue 88)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With defaults stage advance into done reports trimmed and prunes the cold archived proof that library archive would with a ledger entry whose authority names KNOW-13. With retention manual or the librarian off it reports skipped and removes nothing. With a native change open it reports refused naming it and removes nothing and done is still recorded. Each new test fails on main 91a1fdd and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
