# Lesson bundle: run-specsync-s-change-new-answer-check-and-status-through-chough-171-drive-20

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Run SpecSync's change new, answer, check and status through Chough (171, DRIVE-20)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/change.rs, src/main.rs, src/skill.rs, tests/change.rs, tests/package.rs, docs/cli.md, docs/lifecycle.md, CHANGELOG.md, README.md, hi/drive.md, hi/flow.md, skills/chough-workflow/SKILL.md, skills/chough-workflow/references/plan.md, skills/chough-workflow/references/accept.md, skills/chough-workflow/references/lifecycle.md, skills/chough-workflow/references/checkpoint.md, skills/chough-workflow/references/stages.md, skills/chough-workflow/templates/brief.md, .agents/skills/chough-workflow/SKILL.md, .agents/skills/chough-workflow/references/plan.md, .agents/skills/chough-workflow/references/accept.md, .agents/skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/checkpoint.md, .agents/skills/chough-workflow/references/stages.md, .agents/skills/chough-workflow/templates/brief.md
- **Acceptance**: REQ-cli-057 (new): chough change new, answer, check and status run the matching SpecSync step (check with --commit, the others with --json) in the --root directory, pass SpecSync's stdout, stderr and exit code through byte for byte, and write nothing under .chough/. Status keeps the native read bound and, past it, refuses naming a retry that a test runs; check is never stopped by that bound; SIGTERM stops the SpecSync child; a missing specsync refuses naming cargo install --locked specsync@6.2.0 and the step to rerun, which a test runs. Six tests in tests/change.rs fail first because chough change does not exist, and each mutant in testing.md fails one of them. The skill (both copies) and docs name chough change for these four steps. fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `e325d4fe57f52ec371b2888c435f4b2ded144886`
- Base commit: `167790489735b3e6a88ec21a20bb2530d4c96138`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #171, slice 1 of "Chough drives SpecSync". Leif's interview answer 1 ("Also records my gates", 2026-11-06, orc tab): agents only call chough, and Chough runs the SpecSync agent steps itself. Captured as DRIVE-20. The interim re-audit (2026-11-06) counted 18 direct `specsync` calls in each walkthrough: init, scaffold, check, adopt, change new, two approve attempts, four answers, status, four change checks, review and finalize. This slice moves 10 of them behind `chough`: new, the four answers, the four checks and status.

## Decisions that bind this change

- Leif, relayed by the orc (t18), 2026-11-06: "A: amend, behind a summary" and "Capture + approve slice 1". DRIVE-20, DRIVE-21, FLOW-19 and FLOW-20 are captured as drafted, with "amended by FLOW-19" notes on FLOW-16 and the Drive intent. The slice order is approved, and this slice's definition is approved as drafted in the plan.
- Slice order: (1) this one; (2) `chough change ship`; (3) direct lifecycle writes flagged (DRIVE-21); (4) gate summaries and one-reply lists; (5) native gate records with `relayed_by`, which needs spec-sync #900 released; (6) project setup, if Leif wants it.
- IDs agreed with t27, who drives #169/#170: they take DRIVE-18, DRIVE-19 and REQ-cli-056; this change takes DRIVE-20, DRIVE-21, FLOW-19, FLOW-20 and REQ-cli-057. They touch `src/evidence.rs`; this change does not.
- Redefined on 2026-11-06 after #164 (FLOW-18) merged into main: its `tests/docs.rs` requires a README helper-table row for every command, so `README.md` is added to the affected paths. Leif chose option A (README.md only; the `docs/walkthrough.md` lines that still show `specsync change new` and `check --commit` move in slice 2), relayed by the orc, who recorded the send-back as 0xLeif. Scope, requirement and tests are otherwise unchanged.
- Ruled out for this slice:
  - Approve, review, ship and finalize.
  - History records of Chough-run steps (slice 3).
  - Rewriting SpecSync's recovery text to name `chough` commands.
  - A `--no-commit` form of check: the plain `specsync change check <id>` used to materialize a delta stays a direct call for now.

## From the change's design.md

# Design

- **`src/change.rs`** builds the SpecSync argv for each subcommand. It runs SpecSync with its working directory at `--root`, then copies SpecSync's stdout and stderr bytes to Chough's own and exits with SpecSync's exit code. Chough adds nothing to either stream on success or on a SpecSync refusal.
- **Writes** (`new`, `answer`, `check`) run through `records::run_owned` with no deadline, so a signal to Chough stops the child (FLOW-14) and no time bound does.
- **`status`** runs with the native read deadline (`native_deadline`, 120 s unless `CHOUGH_NATIVE_TIMEOUT_SECONDS`). Past it, Chough stops the child and refuses with exit 1: "Error: specsync change status gave no response within <bound>; nothing was recorded. Run `CHOUGH_NATIVE_TIMEOUT_SECONDS=<longer> chough change status [<id>]` to wait longer."
- **A missing `specsync`** (it cannot be started) refuses with exit 1 before anything runs: "Error: specsync could not be started (<io error>). Install it with `cargo install --locked specsync@6.2.0` (README, Install the toolchain), then run `chough change <step> …` again."
- `--root` defaults to `.`, as for every other command. No file under `.chough/` is read or written, so no session, lock or history is involved.
- Argument order passed to SpecSync: for `new`, the description, then `--kind`, each `--spec`, each `--path`, and `--json`.

## From the change's testing.md

# Testing

New tests in `tests/change.rs`, one or more per REQ-cli-057 criterion. Each uses a stub `specsync` on PATH that records its argv and working directory, then prints known bytes to stdout and stderr and exits with a chosen code:

- `change_new_answer_check_and_status_run_specsync_with_exact_arguments`: exact argv and working directory for all four.
- `specsync_output_and_exit_code_pass_through_byte_for_byte`: stdout, stderr and exit code, including a SpecSync refusal (exit 1) and a nonstandard exit (3).
- `no_change_command_writes_under_dot_chough`: the bytes of every file under `.chough/` are unchanged.
- `check_outlives_the_native_bound_while_status_refuses_past_it`: with `CHOUGH_NATIVE_TIMEOUT_SECONDS=1` and a stub that sleeps 2 s, check exits 0 and status refuses; the retry it names, with a longer bound, exits 0.
- `a_missing_specsync_refuses_naming_install_and_the_step_to_rerun`: the refusal names `cargo install --locked specsync@6.2.0` and the step; putting the stub on PATH and running that step exits 0.
- `sigterm_to_change_check_stops_specsync`: the stub's process is gone after Chough is stopped.

They fail first because `chough change` does not exist.

Mutation round, each mutant failing at least one new test: drop `--commit`; drop `--json` from status; swap the stdout and stderr copies; return 0 instead of SpecSync's exit code; apply the native bound to check; skip the bound for status; omit the working directory.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-057 | tests/change.rs, the six tests above; cases in `.chough/cases/cli.change-steps.cases.toml` |

## Lanes

`fledge lanes run verify` passes on Linux; the change's tests pass on macOS.

## Where these lessons go

- `specs/cli/context.md`
