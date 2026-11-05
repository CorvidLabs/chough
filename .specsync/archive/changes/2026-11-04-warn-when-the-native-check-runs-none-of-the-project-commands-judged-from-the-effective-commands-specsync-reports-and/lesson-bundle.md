# Lesson bundle: warn-when-the-native-check-runs-none-of-the-project-commands-judged-from-the-effective-commands-specsync-reports-and

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Warn when the native check runs none of the project commands, judged from the effective commands SpecSync reports, and point the test stage to recorded test evidence (issue 85)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/policy.rs, src/stage.rs, src/main.rs, tests/verification.rs, tests/common/mod.rs, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: With an effective native command list of only specsync check the next and status reports carry the warning naming that command. With a configured project command that the effective list omits the warning still shows and counts it. With a project command in the effective list or no list or SDD disabled or no native configuration there is no warning. At the test stage next names chough cases run and says recorded test evidence is needed before accept. Stage moves and native reads are unchanged. Each new test fails on main 91a1fdd and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `4c3b7ab9a9bfdfe5d07744b47a6f35fe0f7978b7`
- Base commit: `1ea894c91144947915dcaccf11aecb0f6ab86751`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue https://github.com/CorvidLabs/chough/issues/85 (field test 2026-11-03): with no
project commands in native verification, `specsync change check` reports "verified"
while the project's `cargo test` fails, and `chough stage advance` reaches accept with
no evidence. Leif chose "Warn and point" on 2026-11-04 (FLOW-15); a gate was offered
and not chosen.

This replaces PR #89, closed unmerged by Leif's decision. #89 warned when
`.specsync/sdd.json` listed no `verification_commands`. The 0xLeif agent review showed,
and a probe on 2026-11-04 confirmed (CorvidLabs/spec-sync#848), that SpecSync 6.1.2's
`change check` does not run configured `verification_commands` at all: a configured
`sh -c "printf invoked > .native-command-probe; exit 9"` never ran and the check passed.
The change's effective commands, reported as `summary.verification_commands` by
`specsync change status --json` (the read `next` and `status` already make) and recorded
in its attempts, were only `specsync check --spec cli`. So the configuration is not
what runs; the effective list is. #89's change never reached main; correcting its
approved criteria in place was not possible (spec-sync#844).

Criteria: FLOW-15, FLOW-6, LIFE-7. Authority: Leif's 2026-11-04 standing approval for
bounded Chough bug fixes, his brief to fix issues 81 to 88 one PR each, his "Warn and
point" choice, and his choice to close #89 and redo it from main.

## From the change's design.md

# Design

- New `src/policy.rs` owns reading `.specsync/sdd.json` (moved unchanged from
  src/stage.rs) and adds `verification_warning(root, effective)`, where `effective` is
  the `summary.verification_commands` list of the native status read that `next` and
  `status` already made (no new read). It returns a warning when that list is present
  and every entry is one of SpecSync's own commands (`specsync` followed by a space),
  naming them; when `.specsync/sdd.json` lists commands the effective list does not
  contain, the warning also says how many configured commands are not run. It returns
  nothing when the list is absent (native read unknown or older output), when a project
  command is present, when SDD is disabled or with no native configuration.
- `chough next` (text and JSON) and `chough status` carry it as
  `native_verification_warning` (null otherwise); text prints a `Warning:` line under
  the native change. The `stage` report is not changed: `stage show` makes no native
  read.
- `stage::assess`: the plain "Continue" action at test keeps its text, says recorded
  test evidence is needed before accept and names `chough cases run`.
- tests/common/mod.rs: the stub `specsync` reports `summary.verification_commands` from
  `.stub-verification` when that file exists; existing fixtures are unchanged.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
