# Lesson bundle: chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Chough change ship: ship after the review, close the review gate with the relay, and move to done (179, DRIVE-20)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/change.rs, src/checkpoint.rs, src/main.rs, src/skill.rs, tests/change.rs, tests/package.rs, tests/docs.rs, docs/cli.md, docs/walkthrough.md, docs/lifecycle.md, README.md, CHANGELOG.md, skills/chough-workflow/references/accept.md, skills/chough-workflow/references/lifecycle.md, skills/chough-workflow/references/stages.md, .agents/skills/chough-workflow/references/accept.md, .agents/skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/stages.md
- **Acceptance**: REQ-cli-059 (new): chough change ship <id> with the review gate's relay checks the session, stage, open gates and relay before anything runs (each refusal names a recovery that a test runs), runs specsync change ship <id> unbounded with its output and refusals passed through, then closes the gate as a relayed claim and moves the session to done; a refused step after a successful ship exits 1, says shipped and names the remaining command, which a test runs. The walkthrough and the skill's closeout run chough change new, check and ship, with review still specsync change review. Six tests in tests/change.rs fail first, each mutant in testing.md fails one of them, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `cf6d1837618d4d017449053eb6de5a35b05736d0`
- Base commit: `852605831643d975411015c58cc0f9a2919d0568`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #179, slice 2 of "Chough drives SpecSync" (DRIVE-20: "Chough runs SpecSync's agent steps itself: change new, the interview answers, check --commit, ship or finalize, and status"). Slice 1 (#171, merged in #174) added `chough change new`, `answer`, `check` and `status`. Leif approved the slice order on 2026-11-06, relayed by the orc: slice 2 is "`chough change ship`: ship or finalize after the review, then the done move". During #174's re-approval he chose option A, which moves the walkthrough's direct calls in this slice.

## Decisions that bind this change

- **The relay goes with the ship.** The skill's closeout says nothing writes to `.chough/` between `change check --commit` and finalize, because SpecSync's verification digest covers `.chough/`. So the review gate can only be closed after ship, and a one-command closeout must take the relay with it. The relay is checked in full before ship, so a bad value never leaves a shipped change with its gate open.
- **`ship` covers finalize.** `specsync change ship` preflights and finalizes, so there is no separate `chough change finalize`.
- **Review stays direct.** `specsync change review` is the native record of Leif's review. Writing it from Chough is slice 5 (FLOW-19), after spec-sync #900.
- **IDs.** REQ-cli-059 is this change's; REQ-cli-058 is #177's, agreed with t27.
- **Ruled out:**
  - A second native read: closing the gate reads no native state, so the move into done keeps its one read.
  - Rewriting `stage advance` or `checkpoint`: the steps after ship run Chough's own commands.
  - `library index` inside ship: the agent still runs it before committing.

## From the change's design.md

# Design

- **`ChangeCommand::Ship`** takes `<id>` and the optional `--close-gate`, `--decided-by`, `--words`, `--where` and `--recorded-by`.
- **Pre-checks**, reads only, before any process starts:
  - The checkpoint's active session records native change `<id>`. If not, it refuses: "the active session's native change is <x>, not <id>; run `chough session use <id>` or `chough change ship <x>`".
  - The stage is accept. If not, it refuses naming `chough stage advance --to accept`.
  - Each open `awaiting_human` gate equals `--close-gate`. Otherwise it refuses, naming the full `chough change ship <id> --close-gate "<gate>" --decided-by … --recorded-by …` command.
  - With `--close-gate` given, the four values are checked by the function extracted from `checkpoint --close-gate`. That function takes the command a refusal tells you to run again. For `checkpoint`, its incomplete, gate-not-open and too-long refusals keep their exact text. For `ship`, they name `chough change ship <id> --close-gate …`, because closing the gate with `checkpoint` before ship would write `.chough/` inside the verification window.
- **Ship** runs `specsync change ship <id>` through `records::run_owned`: working directory `--root`, no deadline, under the stop watch. Its stdout and stderr bytes are copied through. A non-zero exit returns that code with nothing written.
- **After a successful ship**, Chough runs `std::env::current_exe()` with `--root <root> checkpoint --close-gate <gate> --decided-by … --recorded-by …` (only when a gate was named), then with `--root <root> stage advance --to done`, each through `run_owned`. Each child's stdout and stderr are copied through after SpecSync's. If a child exits non-zero, Chough exits 1. On stderr it says: "specsync change ship <id> succeeded, so the change is shipped; <step> was refused. Run `<that step>`, then `<any step after it>`."
- No other file changes behavior. `checkpoint --close-gate` keeps its output and refusals.

## From the change's testing.md

# Testing

New tests in `tests/change.rs`. They use an archived session at accept (as in `tests/advance_to.rs`) and a stand-in `specsync`. For `change ship` the stand-in records its argv, sleeps for `.stub-ship-sleep` seconds when that file exists, prints `.stub-ship-out` and `.stub-ship-err`, and exits with `.stub-ship-exit`. For `change status` it reports archived, and with `.stub-status-fail` it exits 2.

- `ship_with_the_review_relay_ships_closes_the_gate_and_moves_to_done`: one `change ship <id>` call; the gate is closed with the four values in `relayed_by`; the stage is done.
- `a_specsync_ship_refusal_passes_through_and_records_nothing`: SpecSync's bytes and exit code pass through, and `.chough/` is unchanged.
- `ship_refuses_before_running_without_the_relay_or_outside_accept`: each refusal leaves SpecSync uncalled and `.chough/` unchanged, and each named command then succeeds. The cases: an open gate with no relay; an incomplete relay; a gate that is not open; another change; stage test.
- `a_refused_done_move_after_ship_names_the_remaining_step`: with status failing after ship, the exit is 1, and the output says shipped and names `chough stage advance --to done`. Without the failure, that command then reaches done.
- `ship_outlives_the_native_read_bound`: with `CHOUGH_NATIVE_TIMEOUT_SECONDS=1` and a 2 s ship, the ship completes and the session reaches done.
- `the_walkthrough_and_closeout_run_chough_change`: the walkthrough and the accept reference name `chough change new`, `check` and `ship`, and neither has a direct `specsync change new`, `check … --commit` or `ship`.

They fail first because `chough change ship` does not exist and the docs still name the direct calls.

Mutation round, each mutant failing at least one new test:
- skip the stage check;
- skip the gate check;
- close the gate before ship;
- skip the close;
- skip the done move;
- return 0 after a refused done move;
- apply the native read bound to ship.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-059 | tests/change.rs, the six tests above; cases in `.chough/cases/cli.change-ship.cases.toml` |

## Lanes

`fledge lanes run verify` passes on Linux, and the change's tests pass on macOS.

## Where these lessons go

- `specs/cli/context.md`
