---
change: warn-when-the-native-check-runs-none-of-the-project-commands-judged-from-the-effective-commands-specsync-reports-and
artifact: testing
---

# Testing

- tests/verification.rs, with stub `hi` and `specsync` at the test stage:
  - `a_check_that_runs_only_specsync_commands_is_reported`: effective
    `["specsync check --spec cli"]` warns in `next` text and JSON and in `status`,
    naming that command.
  - `configured_commands_the_check_does_not_run_are_counted`: `.specsync/sdd.json`
    lists `cargo test --locked` but the effective list does not; the warning still shows
    and says 1 configured command is not run (the #89 review regression).
  - `no_warning_when_a_project_command_runs_or_the_list_is_unknown`: an effective list
    with `cargo test --locked`, no list in the status reply, SDD disabled, and no native
    configuration each give `null`.
  - `the_test_stage_points_to_recorded_test_evidence_before_accept`.
  - `the_warning_changes_no_gate_and_adds_no_native_read`.
- #92 review: `the_warning_stays_within_the_next_bound` (a 20,000-character effective
  command; 10 commands) fails at 2d73262 with `next --json` at 21,769 bytes.
- Each new test fails on main 91a1fdd; `fledge lanes run verify` passes on Linux.
