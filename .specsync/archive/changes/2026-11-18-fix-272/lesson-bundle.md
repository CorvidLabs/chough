# Lesson bundle: fix-272

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 272
- **Kind**: BugFix
- **Specs**: records
- **Paths**: src/journal.rs, tests/journal.rs, .chough/cases/cli.journal-lock.cases.toml, changelog.d/2026-11-18-fix-272.md
- **Acceptance**: While another process holds .chough/local/checkpoint.lock, chough journal add waits and writes no post; once the lock is released it writes its post and a feed that lists it. chough library index --check passes after concurrent journal adds.

## Evidence

- Verification commit: `3f3f95ce600e482aeead54a0a14a75f3232ddcfc`
- Base commit: `8abb5bb89b5244e72327e1ea6abfc7e66634ebe1`
- Verified by: `specsync check --spec records`

## From the change's context.md

# Context

GitHub #272: twelve `chough journal add` commands run at once all write their post, but the regenerated `knowledge/journal/feed.xml` can miss some, and `chough library index --check` then fails. REQ-cli-040 already names `journal add` among the commands that hold the project's writer lock while they read what they change and save it. On main at 8abb5bb `journal add` takes no lock, so each command rebuilds the feed from a directory listing another command is still adding to. This change makes the code do what the requirement says. It changes no requirement text and adds no command or flag. `chough library index` is not in REQ-cli-040's list and is not changed here.

## From the change's testing.md

# Testing

`cargo test --locked --test journal journal_add_waits_for_the_writer_lock` holds `.chough/local/checkpoint.lock` in the test process, starts `chough journal add`, and checks that it is still waiting with no post written; after the lock is released the command exits 0 and the feed lists its post. It fails on main, where the command finishes while the lock is held. `cargo test --locked --test journal parallel_journal_adds_all_reach_the_feed` runs twelve adds at once and checks that the feed lists all twelve and that `chough library index --check` passes. Case `TC-cli-journal-add-holds-the-writer-lock` binds the first test.

## Where these lessons go

- `specs/records/context.md`
