---
change: a-blocking-bug-stops-only-a-session-it-is-related-to
artifact: testing
---

# Testing

cargo test --locked --test bugs, --test advance_to, --test skill_releases, the stages test a_linking_bug_lifts_the_unlinked_failure_refusal_and_other_moves_are_unchanged, and the next test both_forms_stay_within_the_cap_with_totals_for_large_records. All of those passed.

The new test a_blocking_bug_stops_only_a_session_it_is_related_to failed on the previous behavior, then passed. It covers a bug that names nothing, a bug listed on one session, and a bug that shares a requirement with one session.
