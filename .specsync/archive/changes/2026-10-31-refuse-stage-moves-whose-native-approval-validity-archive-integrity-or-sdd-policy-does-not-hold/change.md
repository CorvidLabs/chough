---
id: refuse-stage-moves-whose-native-approval-validity-archive-integrity-or-sdd-policy-does-not-hold
state: archived
type: bug_fix
base_commit: ef6695fe9a753112dc77b156d11d9441ea81e529
---

# Refuse stage moves whose native approval validity, archive integrity or SDD policy does not hold

## Intent

Refuse stage moves whose native approval validity, archive integrity or SDD policy does not hold

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Entering build, test or accept by advance, skip, back or reset is refused unless native status reports state approved, implementing or verifying with summary.approval_valid true and summary.correction_valid not false; the refusal names the failed condition and the native next action. With approval_valid false and an eligible state, plan -> build is refused (BUG-0006, REQ-cli-008).
- Reaching done by advance, skip, reset or release -> done is refused unless native state is archived and summary.terminal_evidence.validity is authenticated-history, exact or successor-covered; corrupt-history, a missing validity or any other state refuses. Reset and back to intent, design and plan stay allowed, and test and accept are also reachable for an archived change with valid terminal evidence (BUG-0007, REQ-cli-008).
- The SDD policy is read with SpecSync semantics: no .specsync/sdd.json, or a valid policy with enabled false, stays ungated with the existing labels; a missing enabled key means enabled; malformed JSON, a non-object, or an ill-typed known field refuses gated moves as refused:sdd-policy-invalid and names the file (BUG-0008, REQ-cli-008, REQ-cli-003).
- Every existing transition, side-stage and release behavior that these rules do not name is unchanged; the full suite, the feature-and-bug exercise and native structural checks pass; regression tests reproduce BUG-0006, BUG-0007 and BUG-0008 with real SpecSync in disposable repositories using clearly synthetic approvals, and the three bugs are verified by passing observations.

## No-spec Rationale

Not applicable
