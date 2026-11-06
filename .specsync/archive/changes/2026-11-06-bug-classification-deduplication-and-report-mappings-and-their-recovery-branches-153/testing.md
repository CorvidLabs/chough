---
change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
artifact: testing
---

# Testing

Six tests in tests/bugs.rs, one per REQ-cli-054 criterion:
- `every_classification_scope_and_status_is_accepted_and_others_are_refused`
- `only_an_open_blocking_bug_blocks_leaving_test`
- `search_scores_words_and_shared_ids_and_refuses_only_likely_duplicates`
- `links_to_the_bug_itself_or_to_an_unknown_bug_are_refused_with_their_recovery`: fails first, because the self-duplicate refusal names no recovery, a self-related link is accepted, and the unknown-bug refusal names no recovery.
- `a_mistaken_duplicate_is_reopened_and_an_unverifiable_fix_names_its_recovery`: fails first, because "duplicate_of is only set for duplicate bugs".
- `the_report_shows_every_stored_field_and_none_recorded_for_empty_ones`

Each test runs the recovery its refusal names: `chough bug list --all`, `chough bug update <id> --status open`, and `chough bug update <id> --observation <id>` followed by verified.

Mutation check of the four mapping tests. Each mutant fails the new test, while the existing test named for it passes:

| Mutation | Existing test that passes it |
|---|---|
| `evidence-gap` recorded as `requirement-gap` | `a_scope_given_as_a_classification_and_the_reverse_name_the_right_flag` |
| `wont-fix` counted as open | `an_open_blocking_bug_sends_work_back_to_build` |
| shared-ID bonus dropped | `a_reproduced_bug_is_deduplicated_and_keeps_added_evidence` |
| duplicate threshold lowered to 0.25 | `a_reproduced_bug_is_deduplicated_and_keeps_added_evidence` |
| report hides the fixture | `unverified_out_of_scope_bugs_stay_visible_without_changing_scope` |

Retained failure and fix proof: the two failing-first tests are recorded failing before the fix and linked to a local bug, then recorded passing after it, and the bug is verified.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-054 | tests/bugs.rs, the six tests above |

## Lanes

`fledge lanes run verify` passes on Linux.
