---
change: unlinked-failure-ignores-not-run-leaving-test-keeps-refusing-when-a-referenced-case-s-latest-executed-observation
artifact: testing
---

# Testing

- tests/stages.rs::a_later_not_run_keeps_an_unlinked_failure_refused: fails before the fix, passes after.
- The existing #39 tests (leaving_test_refuses_an_unlinked_failed_case_until_it_passes, a_linking_bug_lifts_the_unlinked_failure_refusal_and_other_moves_are_unchanged) still pass unchanged.
- A new case TC-cli-a-later-not-run-keeps-an-unlinked-failure-refused binds the test (cargo-test, target stages) and links REQ-cli-008; it runs on local-linux-x86_64 after the scoped check.
- fledge lanes run verify on Linux; hosted CI is reported only as observed.
