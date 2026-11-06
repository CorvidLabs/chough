---
change: a-session-archives-itself-at-its-own-move-into-done-155
artifact: tasks
---

# Tasks

- [x] Add the failing tests `the_move_into_done_archives_the_session_and_clears_the_pointer`, `the_move_into_done_keeps_a_session_with_unfolded_inbox_entries_open` and `the_move_into_done_skips_the_archive_without_sessions`.
- [ ] Add the release test, and model a left-open done session in the tests/sessions.rs switch tests (`left_open`; release consent before done).
- [x] Sweep: this change's `session new` archives the done sessions open on main.
- [ ] Add `sessions::archive_at_done` and call it last in the done steps, re-pinning the checkpoint to the archived file.
- [ ] Update docs/cli.md, `skills/chough-workflow/references/checkpoint.md` and its `.agents` copy, and CHANGELOG.
- [ ] Add the cases and run verify on Linux.
