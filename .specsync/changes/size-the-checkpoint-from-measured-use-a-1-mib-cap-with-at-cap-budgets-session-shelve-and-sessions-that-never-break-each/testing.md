---
change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
artifact: testing
---

# Testing

## tests/cli.rs and tests/stages.rs

The checkpoint bound tests use `CHECKPOINT_CAP` (1,048,576): exactly at the cap is read, one byte more is refused naming 1048576, and the merge input and result bounds are checked. They fail first under the 131,072 cap.

## tests/next.rs

- `a_checkpoint_at_the_cap_stays_fast_and_one_byte_more_is_refused`: at exactly 1,048,576 bytes, `next`, `next --json`, `status --json` and `context` succeed in under 2 s, with `next` within 16,384 bytes. One byte more is refused naming 1048576.
- `handoff_blocks_stay_within_the_cap_and_are_byte_identical`: REQ-cli-021 AC1's full shape, with 40 gates, 40 blockers and 50 decisions of 4,096 bytes each, within the cap.

## tests/sessions.rs

- `one_oversized_or_unreadable_session_does_not_break_the_others`
- `shelving_an_oversized_session_lets_work_continue`
- `shelving_an_oversized_legacy_checkpoint_lets_a_session_start`

All fail first.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-032 | tests/next.rs `a_checkpoint_at_the_cap_stays_fast_and_one_byte_more_is_refused`; tests/cli.rs `checkpoint_byte_bound_is_exactly_1048576` |
| REQ-cli-021 | tests/next.rs `handoff_blocks_stay_within_the_cap_and_are_byte_identical` |
| REQ-cli-007 | tests/stages.rs `checkpoint_merge_enforces_input_collection_and_output_byte_bounds` |
| REQ-cli-034 | tests/cli.rs `oversized_records_fail_explicitly` |
| REQ-cli-042 | tests/cli.rs `oversized_records_fail_explicitly` |
| REQ-history-006 | tests/sessions.rs `one_oversized_or_unreadable_session_does_not_break_the_others`, `shelving_an_oversized_session_lets_work_continue`, `shelving_an_oversized_legacy_checkpoint_lets_a_session_start` |

## Lanes

`fledge lanes run verify` passes on Linux.
