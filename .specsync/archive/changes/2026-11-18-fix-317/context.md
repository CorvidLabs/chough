---
change: fix-317
artifact: context
---

# Context

GitHub #317, found by the macOS swarm test (s3) on 2026-11-18 and present since 0.6.0. `evidence add`, `cases run` and `cases record` write their evidence entry, then read the active session's `session.md` with no lock, add the observation reference, and save it under the writer lock. Another writer's update saved between that read and the save is overwritten by the stale copy. With 10 `checkpoint --decision` and 10 `evidence add` commands in parallel, only 6 to 9 of the decisions and 1 or 2 of the observation references survived, and `chough doctor` exited 0. In one run, a lost reference to a failing case let `test -> accept` pass.

REQ-cli-040 already requires every record writer, `evidence add` and `cases record` named among them, to hold the project's writer lock while it reads what it changes and saves it, and a command that first runs a case to take the lock for its save and re-read what it changes under it. #302 fixed the same defect for `journal add`.

`bug add` and `bug update` given `--observation` add the bug to the active session in `join_active_session` (`src/bugs.rs`), which reads the session with no lock of its own. When this draft was recreated, that looked like the same defect. It is not: both commands take the project's writer lock first (`sessions::lock_project`), before they read anything, so that read is already under the lock. A test now guards it, and `src/bugs.rs` is unchanged.

Leif approved the scope on 2026-11-18, relayed by his planning session (hc t4c): first "File it, fixer's A list" (take the writer lock before reading, with a failing parallel test first), then "Yes, include them" for `bug add` and `bug update --observation`, and "Recreate the draft". The first draft was approved before `src/bugs.rs` joined the scope and was never committed, so it was recreated with that path declared and the same rationale. It changes no requirement text, command or flag.

Merge order: since #306, `evidence add` writes its entry's temporary file into `.chough/local/` before it takes the lock, and `remove_orphaned_temps` in `src/sessions.rs`, run under the lock, can delete a live writer's temporary file. The parallel test hit that in 1 of 5 runs. This change merges after hc t4b's revert of #306.
