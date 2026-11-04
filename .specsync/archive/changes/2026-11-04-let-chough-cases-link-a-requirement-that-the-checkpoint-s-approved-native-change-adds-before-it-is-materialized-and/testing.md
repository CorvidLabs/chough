---
change: let-chough-cases-link-a-requirement-that-the-checkpoint-s-approved-native-change-adds-before-it-is-materialized-and
artifact: testing
---

# Testing

`tests/cases.rs` with the SpecSync stub and an approved `cli` delta adding REQ-cli-090: `cases check`, `run` and `record` accept a case linking it on one native read and `cases check` lists it under `approved_delta_requirements`; an edited delta, approval_valid false, correction_valid false, draft and archived states, a delta of another module, no checkpoint change, no native configuration and a failed read refuse with the reason; a typo still refuses. The tests fail before the fix. `fledge lanes run verify` runs the full suite.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-009 | `tests/cases.rs`: `a_requirement_the_approved_delta_adds_is_linkable_before_materialization` and `an_unapproved_or_changed_delta_does_not_resolve_a_requirement`; the existing `an_unresolved_requirement_is_refused_with_the_check_first_hint` still passes unchanged. |
