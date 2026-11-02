---
id: unlinked-failure-ignores-not-run-leaving-test-keeps-refusing-when-a-referenced-case-s-latest-executed-observation
state: archived
type: bug_fix
base_commit: 466594bf7ed98b332c701afe1cae4d0f70355aa1
---

# Unlinked failure ignores not-run: leaving test keeps refusing when a referenced case's latest executed observation failed and a later observation is only not-run (PR 43 review)

## Intent

Unlinked failure ignores not-run: leaving test keeps refusing when a referenced case's latest executed observation failed and a later observation is only not-run (PR 43 review)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With a referenced case failed at time 100 and only not-run at time 200, stage advance and stage skip at test are refused with refused:unlinked-failure naming the failed observation, and next and status name the same refusal
- A later passed observation still lifts the refusal even when a not-run follows it; a case with only not-run observations does not refuse
- The regression test fails before the fix and passes after it; fledge lanes run verify passes; hosted CI is reported only as observed

## No-spec Rationale

Not applicable
