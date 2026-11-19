---
change: fix-317
artifact: testing
---

# Testing

`cargo test --locked --test evidence evidence_add_rereads_the_session_under_the_writer_lock` holds the project's writer lock from the test, starts `evidence add`, waits until that command is blocked, changes the session's next action in `session.md`, and releases the lock. The change must survive the save and the observation must be referenced. On main the stale copy overwrites it. `cargo test --locked --test evidence parallel_decisions_and_evidence_adds_all_land` runs ten `checkpoint --decision` and ten `evidence add` commands at once and requires all ten decisions and all ten observation references in the session. On main it loses some. `cargo test --locked --test evidence bug_add_holds_the_writer_lock_and_keeps_another_writers_update` guards `bug add --observation`: with the lock held, `bug add` waits and writes no bug; the test changes the next action and releases the lock; the change must survive and the bug must join the session. It passes on main, since `bug add` already takes the lock first. Case `TC-cli-evidence-writers-reread-under-the-lock` binds the first, deterministic test, and `TC-cli-bug-add-holds-the-writer-lock` the third. Until #306's revert lands, the parallel test can also fail on its temporary-file sweep (see the context).
