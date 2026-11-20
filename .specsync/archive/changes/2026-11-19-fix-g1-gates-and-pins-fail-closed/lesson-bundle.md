# Lesson bundle: fix-g1-gates-and-pins-fail-closed

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix g1 gates and pins fail closed
- **Kind**: BugFix
- **Specs**: cli, history
- **Paths**: src/checkpoint.rs, src/change.rs, src/sessions.rs, src/main.rs, tests/checkpoint.rs, tests/stages.rs, tests/change.rs, tests/sessions.rs, tests/session_pin.rs, docs/cli.md, changelog.d/, specs/cli/requirements.md, specs/cli/cli.spec.md, specs/history/requirements.md, specs/history/history.spec.md, .chough/cases/cli.gates-and-pins.cases.toml, .chough/cases/history.gates-and-pins.cases.toml
- **Acceptance**: A hand-edited blank, placeholder or doubled no_native_change is refused by every reader with the edit named, and session list and doctor still run; inbox list honors --session and CHOUGH_SESSION; session use and session new say when a pin still names another session; session shelve resolves a folder name, alias or bare slug; change ship closes its gate and moves its stage in the session it resolved; the ship history check covers the whole range and refuses when Git cannot read it; chough change refuses a leaking or control-character argument before SpecSync runs

## Evidence

- Verification commit: `41d486ebb63ffd6909f439780d13fb5940d4af28`
- Base commit: `9d260310bca8553a0203e5ef90bc8372b4e15aae`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Five reports from the 0.6.2 candidate swarm and the reviews of merged PRs, all in the
code that decides which session a command acts on and whether a gate holds
(#401, #400, #369, #393, #372), with one item added by the lead from the leaks sweep
(#367): arguments of `chough change` reach SpecSync's error text unchecked.

Each was reproduced on main (9d260310) with the built binary before this was written:

- #401: a session file edited to hold `no_native_change: " "` reads as "none needed"
  with an empty reason, so plan to build moves ungated. The blank and both-fields
  checks exist only in `chough checkpoint --no-native-change`.
- #393: with the pointer on beta, `chough inbox list --session alpha` lists beta.
  `InboxCommand::List` calls `active(root)`; `Fold` resolves the pinned path.
- #372: `CHOUGH_SESSION=beta chough session use alpha` prints `active: alpha` and
  says nothing about the variable; `chough session shelve alpha-work` says no such
  folder though `session use alpha-work` resolves it.
- #369: `change ship` runs its gate close and the move into done as child `chough`
  commands with no `--session`, so a `--session` pin is lost for those two steps.
- #400: `refuse_corrupt_history` stops after 1,000 commits and then passes.

Already true and kept: `session new` and `session use` ignore the pin (REQ-cli-037);
`session list` reports a session that cannot be read with its reason and
`chough session shelve <id>`. `chough doctor` does not name a session it cannot read;
that is doctor's code (the fixer's batch) and is not changed here.

The REQ-cli-057 part needs `leaks::unshowable` from the leaks PR (#407) and is built
after that merges.

## From the change's design.md

# Design

Nothing visual changes. New and changed text a person sees:

- A session whose `no_native_change` is blank, a placeholder, or held with a native
  change is refused by every reader, through the existing "cannot be read" wrapper,
  with a line naming the field and the edit.
- `chough session use` and `chough session new`, given a `--session` or
  `CHOUGH_SESSION` that names another open session, add one line on standard error;
  standard output stays the same JSON.
- `chough change ship` can refuse as `refused:history-unchecked`.
- `chough change new|answer|check|status|ready|ship` refuse an argument holding a leak
  or a control character before SpecSync runs.

## From the change's testing.md

# Testing

One test per issue, each failing on main before the fix:

- #401 (`tests/stages.rs`, `tests/checkpoint.rs`): a session file edited by hand to
  hold `no_native_change: " "`, a `<placeholder>`, or both fields is refused by
  `next`, `checkpoint` and `stage advance` with every file byte-identical;
  `session list` reports it not readable; `doctor` runs; another session works.
- #393 (`tests/session_pin.rs`): `inbox list` under `--session` and under
  `CHOUGH_SESSION` lists the named session's entry, not the pointer's.
- #372 (`tests/session_pin.rs`, `tests/sessions.rs`): the stderr line with the
  variable set and its absence without; shelve by alias, by bare slug, by a folder
  name that is also another session's alias, and of an unreadable session.
- #369 (`tests/change.rs`): with the pointer elsewhere, `change ship --session`
  closes the gate and moves the stage in the named session; the pointer's file is
  byte-identical.
- #400 (`tests/change.rs`): 1,001 commits after the duplicate is still
  `refused:corrupt-history`; a base commit the repository lacks is
  `refused:history-unchecked`.
- REQ-cli-057 (`tests/change.rs`, after #407): a credential sample and a control
  character in each checked argument, with and without `--redact`.

Then every verify step, and the cases run on macOS.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
