---
change: a-session-archives-itself-at-its-own-move-into-done-155
artifact: testing
---

# Testing

New tests in tests/reduce.rs:
- `the_move_into_done_archives_the_session_and_clears_the_pointer`:
  - the move into done reports `session_archive` `archived` with key `legacy`;
  - the file is now in `archive/` at stage done, and the pointer is gone;
  - `next` refuses, naming `chough session new <key>`;
  - `session new beta` archives nothing.
- `the_move_into_done_keeps_a_session_with_unfolded_inbox_entries_open`: the result is `kept_open` with 1 entry, the file and pointer stay, and `next` works.
- `the_move_into_done_skips_the_archive_without_sessions`: with the single checkpoint, the result is `skipped`.

All three fail first: there is no `session_archive` step. The KNOW-29 roll-up tests and every other existing test keep their assertions.

A case binds the three tests. `fledge lanes run verify` passes on Linux.
