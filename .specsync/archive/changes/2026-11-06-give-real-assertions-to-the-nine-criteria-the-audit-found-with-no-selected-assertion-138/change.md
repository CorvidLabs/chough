---
id: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
state: archived
type: bug_fix
base_commit: 260ecc949ea01214f4bec43b30ea4226ce8fae59
---

# Give real assertions to the nine criteria the audit found with no selected assertion (138)

## Intent

Give real assertions to the nine criteria the audit found with no selected assertion (138)

## Affected Canonical Specs

- `cli`
- `reporting`

## Acceptance Criteria

- Each of the nine criteria the audit found with no selected assertion (#138) has a test that asserts it: REQ-cli-002 lesson labels and no lesson store; REQ-cli-007 no second checkpoint store and no APS; REQ-cli-012 the upgrade note for new-form bug IDs; REQ-cli-029 recorded authority before approval and no human decision from labels, and no skipped cold-start proof promoted to a pass; REQ-cli-031 the walkthrough opening; REQ-cli-045 librarian off and no reset time; REQ-reporting-003 before/after measurement through examples/measure-report.sh. Each new assertion fails under a recorded targeted mutation and passes restored; no criterion, test, bound or timeout is weakened. The measure-report test fails first, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Issue #138 (audit assertion review): the nine criteria stay as written; this change adds tests that assert them, and one measurement script, examples/measure-report.sh, that REQ-reporting-003's before/after criterion is asserted through (Leif's choice A, 2026-11-05 21:52 MST)
