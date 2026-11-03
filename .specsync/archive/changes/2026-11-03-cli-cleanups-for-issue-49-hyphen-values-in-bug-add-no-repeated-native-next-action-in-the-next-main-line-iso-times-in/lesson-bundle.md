# Lesson bundle: cli-cleanups-for-issue-49-hyphen-values-in-bug-add-no-repeated-native-next-action-in-the-next-main-line-iso-times-in

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: CLI cleanups for issue 49: hyphen values in bug add, no repeated native next action in the next main line, ISO times in next text, help text, cli.md drift and exercise temp dir cleanup
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/bugs.rs, src/cases.rs, src/stage.rs, src/main.rs, src/timing.rs, tests/bugs.rs, tests/stages.rs, tests/next.rs, docs/cli.md, docs/exercises/feature-bug-loop.md, examples/feature-bug-loop.sh, CHANGELOG.md, .chough/cases/
- **Acceptance**: bug add accepts free-text values that begin with -- (for example --title "--name prints nothing") and records them verbatim
- **Acceptance**: A gate refusal names the native status command once, even when the native next action already names it; the next view's main Next line no longer repeats the native next action, so a long native next action does not cut it
- **Acceptance**: The next text view shows the last failure's time as UTC ISO 8601; the JSON keeps observed_at_unix_seconds
- **Acceptance**: Every bug add and cases run option has a help description
- **Acceptance**: docs/cli.md lists history show|adopt|prune in the Commands table and describes SpecSync 6.1.1 versus 6.1.2 accurately in the approved-delta guard intro
- **Acceptance**: examples/feature-bug-loop.sh removes the temporary directory it created after a passing full run, and keeps a passed-in directory, a prepare-cold-start directory and a failed run's directory
- **Acceptance**: No approval or actor wording changes (#44); fledge lanes run verify passes

## Evidence

- Verification commit: `63db2b6b08990d0d18fe28a9ed92a3acd0c151f1`
- Base commit: `a788869b32b8442cbc8d7be2bf19c19a922954a8`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #49 collects six cleanups found while dogfooding `main` at a788869 (findings 2, 3, 6, 10, 11 and 12 of the 2026-11-02 status notes). Plan comment: https://github.com/CorvidLabs/chough/issues/49#issuecomment-6593417500.

- `bug add --title "--x"` fails in clap with `unexpected argument`.
- In `next`, the main line cuts mid-word because a `Gate not open` action embeds the gate detail, and that detail already carries the native next action, which the view also prints on its own line. Native refusals append "Run `specsync change status <id>`" even when the native next action names that command.
- The `next` text shows `Last failure: … at <unix seconds>`.
- `bug add` and `cases run` options have no help text.
- The `docs/cli.md` Commands table has no `history` row, and the approved-delta intro treats 6.1.1 behavior as current.
- `examples/feature-bug-loop.sh` leaks one `chough-exercise.*` temp dir per run.

The following are out of scope:

- The approval and actor wording in `stage.rs` refusals and `docs/cli.md` rows 391 to 393, which #44 changes.
- `SKILL.md` and `references/lifecycle.md`.
- `tests/next.rs` and the `next()` regions of `src/main.rs`, which #46 changes.

## From the change's design.md

# Design

1. `src/bugs.rs` `AddArgs`: `allow_hyphen_values = true` on the free-text options, plus a doc comment on every option.
2. `src/stage.rs`:
   - `native_hint(next, status)` returns `Native next action: {next}.`, adding ` Run \`{status}\`.` only when `next` does not contain `status`. It is used in the five native refusals that append both.
   - `assess` and `done_action` build `Gate not open` actions from the gate detail before the native hint (`action_detail`). The full detail stays in `gate.detail` and in refusals.
3. `src/main.rs` `next_text`: `Last failure: … at {timing::utc(seconds)}`. `timing::utc` becomes `pub(super)`.
4. `src/cases.rs` `CasesCommand::Run`: doc comments on `id`, `--environment` and `--observation-id`.
5. `docs/cli.md`:
   - Add a Commands row for `history show|adopt [--apply]|prune [--apply]`.
   - Rewrite the guard intro sentence: SpecSync 6.1.1 and earlier keep `approval_valid` true (BUG-0002), and 6.1.2 flags it. Chough keeps the guard. The later 6.1.1 mentions name that version's digest framing and module-name rule and stay.
6. `examples/feature-bug-loop.sh`: an EXIT trap removes the directory only when the script created it, the mode is `full` and the exit status is 0. Otherwise the directory is kept and its path printed.

## From the change's testing.md

# Testing

- `tests/bugs.rs`: a title and symptom starting with `--` are recorded verbatim, and every `bug add --help` option has a description.
- `tests/cases.rs`: every `cases run --help` option has a description.
- `tests/stages.rs`, with a long native next action that names the status command:
  - the advance refusal names `specsync change status <id>` once;
  - the `next` main line is not cut and does not repeat the native next action;
  - the native next action and the gate detail still appear.
- `tests/stages.rs`: the `next` text shows the last failure at `2026-11-02T…Z` for a pinned recorded time, and the JSON keeps the seconds.
- Exercise: run `fledge lanes run verify` (which runs the exercise) and count `chough-exercise.*` directories before and after. Record this as a human-observation with a small log.
- Each new test is checked against the old behavior: it fails on `main`.

## Where these lessons go

- `specs/cli/context.md`
