---
change: flag-direct-specsync-lifecycle-writes-in-history-183-drive-21
artifact: testing
---

# Testing

New tests in `tests/direct_writes.rs`. They use a native-configured fixture with a session for change `d1` and a stand-in `specsync` whose `change answer` and `change check` rewrite `state.json` and `verification.json`. A direct write is simulated by editing a lifecycle record.

- `a_direct_lifecycle_write_is_recorded_once_with_the_next_stage_move`: one event naming `approvals.json` as changed, with the states before and after; a later move records nothing more.
- `chough_change_steps_are_not_flagged_but_an_earlier_direct_write_is`: after `chough change answer` and `check`, the next move records nothing; a direct write made before an answer is still recorded.
- `first_sight_sets_a_baseline_and_flags_nothing`: no event at the first move; a write after it is recorded.
- `next_and_status_show_a_pending_write_and_change_nothing`: the `next` text and the `status` JSON name the file, and `.chough/` is unchanged.
- `a_direct_write_before_ship_is_recorded_at_the_done_move`: a direct `review.json` write, then `chough change ship` with the relay, leaves one event at the move into done.
- `a_direct_write_field_on_another_kind_is_an_invalid_payload`: a gate-closed event with `direct_write` is refused.
- `the_change_log_names_the_oldest_chough_that_reads_direct_write_events`: the CHANGELOG's entry names `direct-native-write` and the release.

In `tests/change.rs`, `no_change_command_writes_under_dot_chough` becomes `no_change_command_writes_a_tracked_file_under_dot_chough`. It checks that no tracked file changes, and that under `.chough/local/` the only file written is `native-writes/<id>.json`.

They fail first because no event kind, baseline or view exists, and the change steps write no baseline.

Mutation round, each mutant failing at least one new test:
- skip the pre-step diff;
- skip the post-step baseline;
- record from `next`;
- drop pending findings at a move;
- flag at first sight;
- accept `direct_write` on any kind.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-060 | tests/direct_writes.rs |
| REQ-history-013 | tests/direct_writes.rs |
| REQ-cli-057 (modified) | tests/change.rs |

## Lanes

`fledge run linux-verify` passes, and the tests pass on macOS.
