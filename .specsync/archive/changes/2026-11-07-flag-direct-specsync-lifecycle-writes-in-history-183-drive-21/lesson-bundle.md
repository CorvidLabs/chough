# Lesson bundle: flag-direct-specsync-lifecycle-writes-in-history-183-drive-21

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Flag direct SpecSync lifecycle writes in history (183, DRIVE-21)
- **Kind**: Feature
- **Specs**: cli, history
- **Paths**: src/direct_writes.rs, src/stage.rs, src/history.rs, src/change.rs, src/main.rs, src/checkpoint.rs, src/skill.rs, tests/direct_writes.rs, tests/change.rs, tests/package.rs, tests/docs.rs, docs/cli.md, CHANGELOG.md, skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/lifecycle.md
- **Acceptance**: REQ-cli-060 and REQ-history-013 (new) and REQ-cli-057 (modified): a lifecycle record of the active session's change written outside Chough is recorded once as a direct-native-write history event, with its files and states, at the next successful stage move; Chough's own change steps are never flagged because they update an untracked baseline; first sight flags nothing; next and status show pending writes and change nothing; nothing is recorded in the check-to-finalize window; a finding never refuses. The change log names the oldest Chough that reads the event. Seven tests in tests/direct_writes.rs and the modified change-steps test fail first, each mutant in testing.md fails one, and fledge run linux-verify passes.

## Evidence

- Verification commit: `cb19ebe72eb18515b508f40a95a89fde3a81faa7`
- Base commit: `0a20d934a08a5c92a14ca6b0016dfb520daa0815`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue #183, slice 3 of "Chough drives SpecSync". DRIVE-21, captured from Leif's interview answer 7 ("Allowed but flagged", 2026-11-06), reads: "Agents may still call specsync directly, and it still works. Chough notes each direct lifecycle write in history, so the need for it can be seen and removed." Slices 1 and 2 (#174, #182) moved new, answer, check, status and ship behind `chough change`. Approve and review stay direct until slice 5 (spec-sync #900 in 6.3.0), so those are the writes this slice will show.

## Decisions that bind this change

- **The slice order.** Approved by Leif, relayed by the orc. When the orc assigned this slice it asked: "name the oldest Chough that reads the new event kind in the release note".
- **When findings are recorded.** Nothing may write `.chough/` between `change check --commit` and finalize. A finding is therefore kept in an untracked local file and recorded with the next successful stage move, which writes history anyway. `next` and `status` only show findings.
- **How Chough knows its own writes.** It must update a baseline after its own steps, and the baseline lives in `.chough/local/`, which Git ignores. REQ-cli-057 promised that the change steps write nothing under `.chough/`, so this change modifies it to allow that one untracked file. Its test then checks that no tracked file changes and that the baseline is the only local write.
- **Compatibility.** `Transition` uses `deny_unknown_fields`, and `validate_payload` refuses unknown kinds. Any Chough before this change, including 0.4.0, therefore refuses a history holding the new event. The change log names the first release that contains it (planned 0.5.0) as the oldest reader.
- **IDs.** REQ-cli-060 and REQ-history-013, announced to t27.
- **Ruled out:**
  - Recording from `next` or `status`: they stay read-only.
  - Recording during the verification window.
  - Watching authored artifacts (`context.md`, design and plan files, deltas): agents write those by hand on purpose.
  - Watching other changes than the active session's.
  - Refusing on a finding: DRIVE-21 says "it still works".
- **Granularity.** Several direct writes between two Chough commands show as one finding listing each changed file. Chough sees the records' state at each of its commands, not each SpecSync call.

## From the change's design.md

# Design

- **Watched set.** In `.specsync/changes/<id>/`: `state.json`, `approvals.json`, `review.json`, `review-attempts.json`, `verification.json` and `verification-attempts.json`. The fingerprint is a map from file name to SHA-256 of its bytes for each present file, plus the `state` value of `state.json`.
- **Local file.** `.chough/local/native-writes/<id>.json` holds `{schema_version: 1, baseline: {files, state}, pending: [finding]}`. It is written whole to a temporary file and renamed into place.
- **Finding.** `{files: [{path, change}], before_state, after_state}`, where each `change` is `added`, `changed` or `removed`.
- **Change steps** (`src/change.rs`): before the SpecSync step, a difference from the baseline is appended to `pending`. After the step, the baseline is set to the current records, whatever the step's exit code: the step was Chough's. For `new`, the change ID comes from SpecSync's JSON output (`change.id`). Ship's refusals before running write nothing, as REQ-cli-059 requires.
- **Stage moves** (`src/stage.rs`): with a native change recorded, the save that publishes a successful move first publishes one `direct-native-write` event per pending finding and per current difference. The event has `from` and `to` set to the stage before the move. Then the local file is cleared and re-baselined. With no local file, the move sets the baseline and records nothing.
- **Events.** `Transition` gains `direct_write: Option<DirectWrite>`, skipped when it is `None`, so other records keep their bytes. `validate_payload` accepts the kind `direct-native-write` only with `direct_write` and `from == to`, and refuses `direct_write` on any other kind.
- **Views.** `next` prints `Direct lifecycle writes: N not yet recorded (approvals.json changed, …)` when any exist. `status` adds `direct_writes: {pending: [...], baseline: true|false}`. Neither writes.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
