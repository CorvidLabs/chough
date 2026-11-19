---
change: fix-329
artifact: testing
---

# Testing

- `tests/bugs.rs::a_blocking_bug_added_in_a_session_stops_it`: fails on main. With a session active, `bug add --scope blocking` with nothing linked puts the bug on the session's bugs list, and the session refuses to leave test, naming it.
- `tests/bugs.rs::a_blocking_bug_added_with_no_session_active_stays_unlinked`: with no session open, `bug add --scope blocking` writes the bug and no checkpoint. A session opened afterwards is not stopped by it, and `chough next` names it as not blocking this session.

Case `TC-cli-bug-add-joins-the-active-session` binds the first test.

Three existing tests assert the old rule and are updated:
- `unverified_out_of_scope_bugs_stay_visible_without_changing_scope`: the bug joins the list, and the session's scope is unchanged.
- `a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story`: a bug added without an observation joins too, and an update naming another session's entry still leaves the active checkpoint unchanged.
- `tests/records.rs::names_are_date_and_slug_with_aliases`: it reads the bugs list from the checkpoint.
