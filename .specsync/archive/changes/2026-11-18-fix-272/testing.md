---
change: fix-272
artifact: testing
---

# Testing

`cargo test --locked --test journal journal_add_waits_for_the_writer_lock` holds `.chough/local/checkpoint.lock` in the test process, starts `chough journal add`, and checks that it is still waiting with no post written; after the lock is released the command exits 0 and the feed lists its post. It fails on main, where the command finishes while the lock is held. `cargo test --locked --test journal parallel_journal_adds_all_reach_the_feed` runs twelve adds at once and checks that the feed lists all twelve and that `chough library index --check` passes. Case `TC-cli-journal-add-holds-the-writer-lock` binds the first test.
