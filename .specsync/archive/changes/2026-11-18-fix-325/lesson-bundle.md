# Lesson bundle: fix-325

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 325
- **Kind**: BugFix
- **Specs**: cli, history
- **Paths**: src/evidence.rs, tests/evidence.rs, .chough/cases/cli.busy-evidence-add.cases.toml, changelog.d/2026-11-18-fix-325.md
- **Acceptance**: While another process holds the project's writer lock, chough evidence add writes no entry while it waits, and when it refuses as refused:checkpoint-busy after the wait every file is unchanged, so a rerun records one entry, not two. cases run and cases record take the lock before writing their entry too, after any case has run.

## Evidence

- Verification commit: `f0f5c77b276731828e9d70991011a9e2df3f6620`
- Base commit: `cef86e2622a2a908856b0c2cf11b29d3c1d3e472`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

GitHub #325, found by the macOS swarm test (s3-F4) and seen again in the tester's check of #324. When `chough evidence add` waits 30 seconds for the writer lock and then refuses as `refused:checkpoint-busy` ("nothing was saved"), its evidence entry file has already been written, so a rerun creates a duplicate entry. REQ-cli-040 already says a busy writer refuses with nothing saved. The evidence save (`evidence::record`, shared by `evidence add`, `cases run` and `cases record`) writes its entry by create-new before it takes the lock to add the observation reference (#317).

Leif's decision (2026-11-18, planning interview, recorded on #325 and relayed by hc t4c as the scope approval): approved as a fix. A `refused:checkpoint-busy` evidence add leaves nothing behind, so a rerun doesn't duplicate the entry.

## From the change's design.md

# Design

`evidence::record` takes the project's writer lock (`checkpoint::lock_for_update`) before anything else: the entry's name, its create-new write, its side files and the session save. A busy lock therefore refuses before any file is written. `cases run` calls `record` only after its case has run, so the case still never runs under the lock. The lock is held through the session save, as since #317.

## From the change's testing.md

# Testing

`cargo test --locked --test evidence a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved` holds the project's writer lock from the test and starts `evidence add`.
- After a second, the command must still be waiting with no entry written. On main the entry is already there.
- After the 30-second wait, the command must refuse as `refused:checkpoint-busy` with every file unchanged.
- Once the lock is free, a rerun records exactly one entry.

Case `TC-cli-a-busy-evidence-add-leaves-nothing` binds it.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
