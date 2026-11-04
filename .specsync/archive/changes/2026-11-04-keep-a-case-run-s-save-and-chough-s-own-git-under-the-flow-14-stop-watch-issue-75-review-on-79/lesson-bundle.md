# Lesson bundle: keep-a-case-run-s-save-and-chough-s-own-git-under-the-flow-14-stop-watch-issue-75-review-on-79

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep a case run's save and Chough's own Git under the FLOW-14 stop watch (issue 75, review on #79)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/cases.rs, src/evidence.rs
- **Acceptance**: stopping_a_case_run_while_it_saves_stops_git_and_records_nothing passes: a stop while cases run takes its Git snapshot after the command exits stops that Git and leaves .chough byte-identical,Every other cases and evidence test passes unchanged

## Evidence

- Verification commit: `8f186147eb858aaa0e4c16d3be80be142f1de2eb`
- Base commit: `c22bf0c2e6a6cb60e1453b7adb83414c3f930d6c`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The Chough bot's review 5762369485 on #79 found that `cases run` wrote its artifact and took its Git snapshot after the stop watch ended, so a SIGTERM in that window left an OBS log with no observation (23 of 121 trials), and Chough's own Git commands survived a normal kill. The requirement and the watch itself are in stop-what-chough-started-when-chough-is-stopped-issue-75 (src/records.rs). This change is the call-site wiring in src/cases.rs and src/evidence.rs.

## From the change's testing.md

# Testing

- `tests/stop.rs` `stopping_a_case_run_while_it_saves_stops_git_and_records_nothing` fails at e97633e (the Git stand-in survives and an orphan OBS log is left) and passes now.
- `cargo test --locked` and `fledge lanes run verify` pass.

## Where these lessons go

- `specs/cli/context.md`
