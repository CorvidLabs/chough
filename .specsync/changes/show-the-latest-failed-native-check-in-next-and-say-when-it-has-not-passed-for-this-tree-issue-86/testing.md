---
change: show-the-latest-failed-native-check-in-next-and-say-when-it-has-not-passed-for-this-tree-issue-86
artifact: testing
---

# Testing

- tests/native_check.rs, with stub `hi` and `specsync`:
  - `a_failed_native_attempt_is_shown_in_next`: a failed attempt (two failing
    commands and one passing) is shown in text and JSON with its time, commit, failing
    commands and exit codes; a later passing attempt replaces it.
  - `next_says_when_the_native_check_has_not_passed_for_this_tree`: at state
    `implementing` with no attempts file, `next` says so, that SpecSync keeps no reason
    for refusals, and names `specsync change check <id>`; at `verifying` it does not.
  - `an_unreadable_attempts_record_is_unknown_and_adds_no_native_read`: a malformed
    file is reported as unknown, and `next` runs the stub `specsync` once either way.
- Each new test fails on main 91a1fdd; `fledge lanes run verify` passes on Linux.
