---
change: fix-262
artifact: testing
---

# Testing

`cargo test --locked --test special_files` (Unix only) runs each command with a 20 second deadline, so a blocked reader fails the test instead of hanging the suite. `a_fifo_in_the_bugs_folder_blocks_no_reader` puts a FIFO in `.chough/bugs/` and checks that `status`, `next`, `next --json` and `bug list` exit 0. `a_session_whose_record_is_a_fifo_is_listed_with_the_reason_and_recovery` replaces an open session's `session.md` with a FIFO and checks that `session list` reports it as not readable with a reason and `chough session shelve <id>`, that the other session's `next` and `status` work, and that the shelve moves the folder and its note names the entry as not a regular file. Both fail on main: the first by the deadline, the second because the session is not listed.
