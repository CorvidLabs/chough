---
id: unlinked-failure-gate-leaving-test-refuses-while-a-checkpoint-case-s-latest-observation-failed-and-no-local-bug-links
state: archived
type: bug_fix
base_commit: 825a3805530456005e361dfa6d96d93eee52cd09
---

# Unlinked failure gate: leaving test refuses while a checkpoint case's latest observation failed and no local bug links it (issue 39)

## Intent

Unlinked failure gate: leaving test refuses while a checkpoint case's latest observation failed and no local bug links it (issue 39)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Leaving test for accept, by advance or skip, is refused with refused:unlinked-failure while the latest observation of a case referenced in the checkpoint failed and no local bug links that observation; the refusal names the case, the observation and both remedies
- The same move is allowed once a later observation of that case passed or any local bug links the failed observation; open blocking bugs keep refused:blocking-bugs
- Observations that are stale but not failed, failed observations of cases not referenced in the checkpoint, and other stage moves are not affected
- At test, chough next and chough status name the same refusal as the next action from the same rule, within the existing next bounds
- A Linux reproduction of issue 39 fails before the fix and passes after, with new tests in tests/stages.rs failing first; the verify lane passes locally and CI is reported only as observed

## No-spec Rationale

Not applicable
