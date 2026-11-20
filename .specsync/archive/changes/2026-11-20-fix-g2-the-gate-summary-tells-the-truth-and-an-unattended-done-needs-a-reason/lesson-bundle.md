# Lesson bundle: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix G2: the gate summary tells the truth and an unattended done needs a reason
- **Kind**: BugFix
- **Specs**: cli, history
- **Paths**: src/gates.rs, src/stage.rs, src/doctor.rs, src/main.rs, tests/gates.rs, tests/stages.rs, tests/doctor.rs, docs/cli.md, specs/cli/requirements.md, specs/cli/cli.spec.md, specs/history/requirements.md, specs/history/history.spec.md, changelog.d/, .chough/cases/cli.g2-gate-summary.cases.toml, tests/next.rs
- **Acceptance**: The gate summary counts a passing observation with no case, shows uncommitted work as git diff <base> and counts a session with no base from the first commit; doctor's unfinished_archive fix names the journal post and finishes no archive while it is uncommitted; without SpecSync, stage advance says no human decision is recorded for plan or accept, and --to across them refuses as refused:no-human-decision unless --reason is given, which lands in the Timeline.

## Evidence

- Verification commit: `b2031107b4baf73e8bd325b32c7287c3fc36ba69`
- Base commit: `7bf110dcca70bdd3565007086aee40a286f40eff`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

G2: #389 (s9-F4, F5, F14) and #390 (s9-F6), from the 0.6.2 candidate swarm's fresh-eyes
walk. Leif's decision on #390, 2026-11-19: "B: Warn".

Measured on main 7bf110dc (t4b, 2026-11-20):
- F4: a passing `evidence add --criterion WC-1 --kind product-test --outcome passed`,
  then a review gate: `gate list` says `Tests: 0 passed, 0 failed, 0 not run`. Cause:
  `gates::tests` counts only observations bound to a case.
- F5: done with the librarian on, then `chough doctor`: its fix names session.md and the
  story, not the journal post. After that commit, `doctor --fix` exits 0, reports no
  finding, and `knowledge/journal/<post>.md` is still untracked. D1 (#414) fixed the
  rest of F5: the folder no longer goes while the story is uncommitted.
- F14: the diff line is `git diff <base>..<HEAD>`; with the work uncommitted HEAD is the
  base, so it is empty. A session started before the first commit has no base and the
  summary says only "unknown (no base commit recorded)" (code read, gates.rs 690-760).
- F6 (#390): without SpecSync, `session new; stage advance --to done` prints
  `Move: intent -> done` and archives, with no warning and no human decision.

REQ-cli-061 pins the summary's wording (cases; `git diff <base>..<HEAD>`), so F4 and
F14 change approved sentences, and #390 adds a refusal: this scope needs Leif's own
approval, not the standing OK.

## From the change's design.md

# Design

Nothing visual changes beyond the lines named.

- F4: `gates::tests` also counts each observation the session names that links no case,
  by its own entry name, in the same Tests line.
- F14: the diff line is `git diff <base>` while `git diff --quiet HEAD` reports changes,
  with "(work not committed yet)"; with no recorded base and exactly one root commit,
  what changed and the diff count from that root and say "no base commit was recorded:
  the session started before the first commit".
- F5: `unfinished_archive`'s fix names the session's journal post (the story links it;
  else `knowledge/journal/<date>-<slug>.md` by the session's title) with the folder's
  files and the story, and `doctor --fix` skips while it is untracked or uncommitted.
  Narrowed by Leif's "A: narrow it, keep B" (2026-11-20): the fix names the post, and
  an untracked or ignored post never holds the finish; the history delta holds the text.
- #390 (B: Warn): in `stage advance`, for each stage passed that is plan or accept,
  ungated (`ungated:not-configured` or `ungated:sdd-disabled`) and holding no closed
  human gate: print "no human decision is recorded for <stage>", and in the done
  summary. `--to` across such a stage refuses as `refused:no-human-decision` without
  `--reason <text>`, naming the flag; `--reason` is a new `stage advance` flag, leak-
  checked like every recorded text, and written into that move's Timeline entry.

Options for #390 were Leif's (A require a gate, B warn, C document); he chose B. Option A
can follow after 0.6.2 as its own change.

## From the change's testing.md

# Testing

Each failing on main first, as listed in tasks.md. Then every verify step, the cases on
macOS, and, since #390 adds a refusal (rule 3), the tester's run on the real
repositories: no project that uses SpecSync gets the new refusal or warning (they are
gated); in a copy of each project without SpecSync, `stage advance --to done` from
intent refuses naming `--reason`, and with it moves and records the reason; the gate
summary's Tests line on each real open session counts at least what main counts.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
