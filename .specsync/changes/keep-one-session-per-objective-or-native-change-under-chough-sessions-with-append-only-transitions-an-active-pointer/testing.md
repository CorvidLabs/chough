---
change: keep-one-session-per-objective-or-native-change-under-chough-sessions-with-append-only-transitions-an-active-pointer
artifact: testing
---

# Testing

`tests/sessions.rs`: `session new` creates the file with its own cycle and points at
it, and stage moves write that session and its cycle only; `adopt` keeps the bytes and
removes `.chough/session.json`; resolution refuses with no session, several open
sessions and no pointer, or a pointer to an archived session, and uses the only open
session without a pointer; `session use` archives a done session unchanged and `list`
reports it; `inbox add` never changes the session file, `fold` adds each entry once,
moves it to `folded/` and a rerun adds nothing; an inbox entry with an approval field
is refused; a real-Git merge of two branches that each advance a different session
has no conflict and both sessions keep their history; a legacy project is unchanged.
`tests/workflow_authority.rs`: an unedited v0.2.0 checkpoint reference is
known-outdated and `skill update` delivers the sessions guidance. The tests fail
before the change. `fledge lanes run verify` runs the full suite.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-037 | `tests/sessions.rs`: `commands_use_the_active_session_and_refuse_when_unclear`, `adopt_moves_the_checkpoint_byte_for_byte`, `a_switch_partway_through_a_command_does_not_redirect_its_save`. |
| REQ-history-006 | `tests/sessions.rs`: `each_session_has_its_own_cycle`, `switching_archives_done_sessions`, `inbox_entries_fold_once_without_touching_the_session`, `branches_with_different_sessions_merge_without_conflict`, `a_done_session_with_unfolded_entries_stays_open_on_switch`, `fold_checks_its_targets_before_saving_the_session`, `fold_validates_entries_like_add`; `tests/evidence.rs`: `retention_protects_what_inactive_and_archived_sessions_reference`. |
| REQ-cli-007 | `tests/sessions.rs`: `the_repository_declares_the_merge_driver_for_every_session_file`, `branches_with_different_sessions_merge_without_conflict`, with the existing checkpoint merge tests. |
| REQ-history-002 | `tests/sessions.rs`: `each_session_has_its_own_cycle`, with the existing history tests; `tests/spec_split.rs` pins the modified wording. |
