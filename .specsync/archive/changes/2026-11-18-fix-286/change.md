---
id: fix-286
state: archived
type: bug_fix
base_commit: 9dd28de28015ce6f873e237044cc9650e74a3712
---

# Fix 286

## Intent

fix 286

## Affected Canonical Specs

- `history`
- `cli`
- `records`

## Acceptance Criteria

- A receipt in an archived story keeps its evidence entry's criteria and requirements, each one scalar, comma-and-space separated, and its limitations, one scalar, semicolon-and-space separated, each only when the entry has any, placed after case_digest; a migrated receipt keeps them the same way. The What was verified lines are unchanged.

## No-spec Rationale

Not applicable
