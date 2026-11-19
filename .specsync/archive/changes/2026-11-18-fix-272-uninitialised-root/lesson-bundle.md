# Lesson bundle: fix-272-uninitialised-root

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 272 uninitialised root
- **Kind**: BugFix
- **Specs**: records
- **Paths**: src/journal.rs, tests/journal.rs, .chough/cases/cli.journal-lock-uninitialised.cases.toml, changelog.d/2026-11-18-fix-272-uninitialised-root.md
- **Acceptance**: In a project with no .chough directory, chough journal add takes the writer lock .chough/local/checkpoint.lock before it writes, and twelve journal add commands run at once leave a feed that lists all twelve posts, with chough library index --check passing.

## Evidence

- Verification commit: `2c6264697004a0811e4a7722f12887c8f1716f48`
- Base commit: `7aed03f6993eef37826de783c84ccb4d9ab52648`
- Verified by: `specsync check --spec records`

## From the change's context.md

# Context

The agent review of #302 (fix for #272) found a path the merged fix does not cover. `chough journal add` also works in a project that has no `.chough/` directory, with default preferences. The fix called the helper `lock_project`, which takes no lock when `.chough/` is absent, so on such a project `journal add` still runs unlocked. The reviewer ran three storms of twelve adds on fresh roots; one feed listed 11 posts. That is on main at 7aed03f.

REQ-cli-040 names `journal add` among the writers that hold the lock and makes no exception for an uninitialised project. Leif chose, on 2026-11-17, to fix forward by taking the lock there too, not by refusing such a project. Taking the lock creates `.chough/local/` with its `.gitignore`, as any first writer does.

No requirement text changes.

## From the change's testing.md

# Testing

`cargo test --locked --test journal journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory` runs one `journal add` in a project with no `.chough/` and checks that `.chough/local/checkpoint.lock` exists afterwards and that, with that lock held by the test, a second add waits and writes no post. The first check fails on main every time. `parallel_journal_adds_all_reach_the_feed_in_a_project_with_no_chough_directory` runs twelve adds at once in each of five fresh projects and checks every feed lists all twelve; on main it fails only when the race is lost, so it is a timing-sensitive check, not a deterministic one. Case `TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory` binds the first test.

## Where these lessons go

- `specs/records/context.md`
