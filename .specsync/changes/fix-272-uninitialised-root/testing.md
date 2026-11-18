---
change: fix-272-uninitialised-root
artifact: testing
---

# Testing

`cargo test --locked --test journal journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory` runs one `journal add` in a project with no `.chough/` and checks that `.chough/local/checkpoint.lock` exists afterwards and that, with that lock held by the test, a second add waits and writes no post. The first check fails on main every time. `parallel_journal_adds_all_reach_the_feed_in_a_project_with_no_chough_directory` runs twelve adds at once in each of five fresh projects and checks every feed lists all twelve; on main it fails only when the race is lost, so it is a timing-sensitive check, not a deterministic one. Case `TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory` binds the first test.
