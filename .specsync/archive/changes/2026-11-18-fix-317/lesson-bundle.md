# Lesson bundle: fix-317

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 317
- **Kind**: BugFix
- **Specs**: cli, history
- **Paths**: src/evidence.rs, src/bugs.rs, tests/evidence.rs, .chough/cases/cli.evidence-writers-lock.cases.toml, changelog.d/2026-11-18-fix-317.md
- **Acceptance**: evidence add, cases run and cases record take the project's writer lock before they read session.md to add their observation reference, and save under it, so an update another writer saves while one of them waits is kept: with the lock held while evidence add waits and the checkpoint's next action changed meanwhile, the change survives the save. bug add and bug update with --observation do the same when they add the bug to the active session, so an update saved while bug add waits survives too. Ten checkpoint --decision and ten evidence add commands run in parallel keep all ten decisions and all ten observation references. cases run still runs its case without holding the lock.

## Evidence

- Verification commit: `4b8bfe0b3b3fe8e0f62a60a2ad2953e34f0f4d53`
- Base commit: `66543c2bf509627ec2a165414acae0bc8ab59c4a`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

GitHub #317, found by the macOS swarm test (s3) on 2026-11-18 and present since 0.6.0. `evidence add`, `cases run` and `cases record` write their evidence entry, then read the active session's `session.md` with no lock, add the observation reference, and save it under the writer lock. Another writer's update saved between that read and the save is overwritten by the stale copy. With 10 `checkpoint --decision` and 10 `evidence add` commands in parallel, only 6 to 9 of the decisions and 1 or 2 of the observation references survived, and `chough doctor` exited 0. In one run, a lost reference to a failing case let `test -> accept` pass.

REQ-cli-040 already requires every record writer, `evidence add` and `cases record` named among them, to hold the project's writer lock while it reads what it changes and saves it, and a command that first runs a case to take the lock for its save and re-read what it changes under it. #302 fixed the same defect for `journal add`.

`bug add` and `bug update` given `--observation` add the bug to the active session in `join_active_session` (`src/bugs.rs`), which reads the session with no lock of its own. When this draft was recreated, that looked like the same defect. It is not: both commands take the project's writer lock first (`sessions::lock_project`), before they read anything, so that read is already under the lock. A test now guards it, and `src/bugs.rs` is unchanged.

Leif approved the scope on 2026-11-18, relayed by his planning session (hc t4c): first "File it, fixer's A list" (take the writer lock before reading, with a failing parallel test first), then "Yes, include them" for `bug add` and `bug update --observation`, and "Recreate the draft". The first draft was approved before `src/bugs.rs` joined the scope and was never committed, so it was recreated with that path declared and the same rationale. It changes no requirement text, command or flag.

Merge order: since #306, `evidence add` writes its entry's temporary file into `.chough/local/` before it takes the lock, and `remove_orphaned_temps` in `src/sessions.rs`, run under the lock, can delete a live writer's temporary file. The parallel test hit that in 1 of 5 runs. This change merges after hc t4b's revert of #306.

## From the change's design.md

# Design

The three commands share one save, the function in `src/evidence.rs` that writes the entry and then adds its observation reference to the session. That function takes the project's writer lock (`checkpoint::lock_checkpoint`) before it reads `session.md`, and keeps it through `save_checkpoint`, as it already does for the rest of the process. `save_checkpoint` takes the same lock re-entrantly, so nothing else changes.

`bug add` and `bug update` given `--observation` with an entry of the active session add the bug to that session in `bugs::join_active_session`. Both commands already take the writer lock at their start, so that read is under the lock, and `src/bugs.rs` does not change; a test guards it.

`cases run` runs its case before the evidence function, so it still holds no lock while the case runs, and a long case never makes another writer wait past the 30-second bound of REQ-cli-040. The observation-cap check that runs before the case (REQ-cli-032) stays an early refusal without the lock; `save_checkpoint` still validates the bound under the lock.

## From the change's testing.md

# Testing

`cargo test --locked --test evidence evidence_add_rereads_the_session_under_the_writer_lock` holds the project's writer lock from the test, starts `evidence add`, waits until that command is blocked, changes the session's next action in `session.md`, and releases the lock. The change must survive the save and the observation must be referenced. On main the stale copy overwrites it. `cargo test --locked --test evidence parallel_decisions_and_evidence_adds_all_land` runs ten `checkpoint --decision` and ten `evidence add` commands at once and requires all ten decisions and all ten observation references in the session. On main it loses some. `cargo test --locked --test evidence bug_add_holds_the_writer_lock_and_keeps_another_writers_update` guards `bug add --observation`: with the lock held, `bug add` waits and writes no bug; the test changes the next action and releases the lock; the change must survive and the bug must join the session. It passes on main, since `bug add` already takes the lock first. Case `TC-cli-evidence-writers-reread-under-the-lock` binds the first, deterministic test, and `TC-cli-bug-add-holds-the-writer-lock` the third. Until #306's revert lands, the parallel test can also fail on its temporary-file sweep (see the context).

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
