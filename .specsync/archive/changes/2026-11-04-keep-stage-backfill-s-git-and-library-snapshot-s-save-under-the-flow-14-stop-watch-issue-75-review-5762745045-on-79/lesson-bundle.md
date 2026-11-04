# Lesson bundle: keep-stage-backfill-s-git-and-library-snapshot-s-save-under-the-flow-14-stop-watch-issue-75-review-5762745045-on-79

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep stage backfill's Git and library snapshot's save under the FLOW-14 stop watch (issue 75, review 5762745045 on #79)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/library.rs, src/stage.rs
- **Acceptance**: stopping_a_backfill_stops_its_git_and_records_nothing passes: a stop while stage backfill checks its commit stops that Git and leaves .chough byte-identical,stopping_a_library_snapshot_stops_git_and_records_nothing passes: a stop while library snapshot reads the HEAD stops that Git and leaves .chough and the library byte-identical,Every other stage and library test passes

## Evidence

- Verification commit: `f7d2da55390fc41f8095c343f9f66a2bc25512f5`
- Base commit: `a2e3bc073970d75e2399f217c9efb122f2c06126`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The Chough bot's review 5762745045 on #79 found two more FLOW-14 gaps. `stage backfill` ran Git with a bare `Command::output`, so a normal kill left that Git running. `library snapshot` copied its files before it read the Git HEAD for the manifest, so a stop during that read left copied files with no manifest. The requirement and the watch itself are in stop-what-chough-started-when-chough-is-stopped-issue-75 (src/records.rs). This change is the call-site wiring in src/stage.rs and src/library.rs.

## From the change's testing.md

# Testing

- `tests/stop.rs` `stopping_a_backfill_stops_its_git_and_records_nothing` fails before the fix (the Git stand-in survives) and passes now.
- `tests/stop.rs` `stopping_a_library_snapshot_stops_git_and_records_nothing` fails before the fix (a copied file is left with no manifest) and passes now.
- The library unit test `an_interrupted_snapshot_has_no_completion_manifest` now asserts that a failed snapshot leaves nothing under `snapshots/`, neither the copy nor a staging directory.
- `cargo test --locked` and `fledge lanes run verify` pass.

## Where these lessons go

- `specs/cli/context.md`
