---
id: restore-the-test-closing-brace-lost-when-merging-main-into-know-22-121
state: implementing
type: bug_fix
base_commit: b3868b844923348903cf67e40dfe6347fb1fb6a1
---

# Restore the test closing brace lost when merging main into KNOW-22 (121)

## Intent

Restore the test closing brace lost when merging main into KNOW-22 (121)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- tests/reduce.rs compiles again with every test from main and KNOW-22, differing from main only by the KNOW-22 tests and the one doc comment KNOW-22 rewrote; the full suite and fledge lanes run verify pass on Linux.

## No-spec Rationale

Merge repair only: both branches appended tests to tests/reduce.rs after a shared closing brace and the merge kept one; no behavior or requirement changes.
