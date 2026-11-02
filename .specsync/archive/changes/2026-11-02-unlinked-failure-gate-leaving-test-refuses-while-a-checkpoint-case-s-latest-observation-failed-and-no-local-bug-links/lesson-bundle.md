# Lesson bundle: unlinked-failure-gate-leaving-test-refuses-while-a-checkpoint-case-s-latest-observation-failed-and-no-local-bug-links

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Unlinked failure gate: leaving test refuses while a checkpoint case's latest observation failed and no local bug links it (issue 39)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/stage.rs, tests/stages.rs, specs/cli/, docs/cli.md, CHANGELOG.md
- **Acceptance**: Leaving test for accept, by advance or skip, is refused with refused:unlinked-failure while the latest observation of a case referenced in the checkpoint failed and no local bug links that observation; the refusal names the case, the observation and both remedies
- **Acceptance**: The same move is allowed once a later observation of that case passed or any local bug links the failed observation; open blocking bugs keep refused:blocking-bugs
- **Acceptance**: Observations that are stale but not failed, failed observations of cases not referenced in the checkpoint, and other stage moves are not affected
- **Acceptance**: At test, chough next and chough status name the same refusal as the next action from the same rule, within the existing next bounds
- **Acceptance**: A Linux reproduction of issue 39 fails before the fix and passes after, with new tests in tests/stages.rs failing first; the verify lane passes locally and CI is reported only as observed

## Evidence

- Verification commit: `4c3db9a25c1003b491812eb5b1b1f6f51a07e625`
- Base commit: `825a3805530456005e361dfa6d96d93eee52cd09`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #39 (CorvidLabs/chough), filed by corvid-agent from a Linux dogfood run of a
small sample project. In stage test, `chough cases run` recorded a failed
observation; no bug was written (the `bug add --title "--chars ..."` call failed in
argument parsing because the value began with `--`); `chough stage advance` then
applied test -> accept. The only gate on leaving test is `refused:blocking-bugs`, so
a failure that never became a bug is left behind, although FLOW-7 says a failing
case stays failed and LIFE-5 connects bugs to verification evidence.

Reproduced on Linux (x86_64) at main 825a380 with SpecSync 6.1.2, Fledge 1.8.0 and
hi 0.8.0 in a disposable copy of `examples/shipping` (SDD off): a failed,
unlinked TC-shipping-0002 observation, then `stage advance` from test exits 0 to
accept, and `chough next` says "Continue test" with no warning. Recorded as
BUG-leaving-test-ignores-a-referenced-case-s-failed-unlinked-observation-ea76be10351c67bc019ac939 (in-scope product defect) linking
OBS-product-test-9e2d612534e636aa16dbc628 and the log
`.chough/artifacts/bug39-repro.log`.

Proposed answers to the issue's open questions (posted on #39 as proposals; they
become binding only through native scope approval of this definition):

- Yes, leaving test reads the latest observation of each case referenced in the
  checkpoint and refuses when it failed and no local bug links it. Refuse rather
  than warn, because the missed warning is the reported problem.
- A linked bug of any status satisfies the gate; open blocking bugs keep their own
  refusal. Stale but not failed observations do not trigger it; they stay under
  "Observations needing attention".
- `chough next` and `chough status` name the same refusal at test from the same
  rule; no new list is added to the bounded next view.

Ruled out: warning only (the failure is the one the next agent misses); gating on
every failed observation in the repository (old, unreferenced failures would block
unrelated work); counting stale observations (a separate decision).

Not in this change: the `bug add` value-starting-with-`--` parse (clap behavior;
`--title=--chars ...` works). Approvals: corvid-agent does not record approval for
a human; this definition waits for 0xLeif's native scope approval.

Overlap: PR #37 (0xLeif) edits `src/stage.rs` in `assess`, `log_transition`,
`log_gate`, `recent_transitions` and `move_forward`'s logging call; this change adds
a prerequisite check beside the existing blocking-bug check and one branch in
`assess`. PR #41 and issue #42 own no file this change edits. REQ-cli-025 to 029
are reserved elsewhere, so this change modifies REQ-cli-008 only.

## From the change's design.md

# Design

## Rule

`unlinked_failures(root, session)`, in `src/stage.rs`: for each case ID in
`session.references.cases`, take the observations from `evidence::load_all` whose
`cases` name it, pick the greatest `(observed_at_unix_seconds, id)`, and keep it when
its outcome is `failed` and no entry of `bugs::observation_references` contains its
ID. The result is sorted by case ID. Pruned observations are not loaded, as
elsewhere. One evidence read and one bug read per call; no native read.

## Where it applies

- `move_forward`, beside the blocking-bug check, when `from == "test"` and `to ==
  "accept"` (covers `advance` and `skip`): refuse with
  `{kind} refused (refused:unlinked-failure): case <case> latest observation <obs>
  failed and no bug links it; rerun the case until it passes, or link it with chough
  bug add ... --observation <obs> or chough bug update <bug> --observation <obs>`
  (the first failure is named, with the count of others).
- `assess`, at stage test after the blocking-bug branch: the next valid action names
  the same refusal and the remedy command, so `next` and `status` agree.

Blocking bugs are checked first, so their refusal is unchanged.

## Not changed

`src/main.rs`, the next view fields and bounds, evidence and bug record formats,
the skill bundle and `examples/`. The repository's own checkpoint has no
referenced case whose latest observation failed unlinked (checked at 825a380), so
its flow is unaffected.

## Estimate

Small: one helper, one prerequisite and one `assess` branch, about 60 lines, plus
tests. Main uncertainty: a textual merge with PR #37 in `src/stage.rs`.

## From the change's testing.md

# Testing

## Cases

| Case | Requirement | Criteria | Method | What it shows |
|---|---|---|---|---|
| TC-cli-leaving-test-refuses-an-unlinked-failed-case | REQ-cli-008 | FLOW-7, LIFE-5, FLOW-2 | automated | The tests/stages.rs gate tests pass: refusal, both remedies, the unaffected cases, blocking-bug precedence and the next action |
| TC-cli-a-linking-bug-lifts-the-unlinked-failure-refusal | REQ-cli-008 | FLOW-7, LIFE-5, FLOW-8 | automated | A linking bug of any status lifts it; blocking bugs refuse first; unreferenced and passed-latest cases do not refuse; equal times order by ID; back is unaffected |

The cases were added after the scoped check materializes the delta (case recording is
not a tasks.md checkbox); TC-cli-0032 (blocking bugs) was rerun as well.

## Reproduction

A disposable copy of `examples/shipping` with SDD off: advance to test, reference
TC-shipping-0002, record a failed observation without a bug, then `stage advance`.
Expected before the fix: exit 0 and stage accept. After: refusal naming
`refused:unlinked-failure`, the case and the observation. The log is kept as an
artifact linked to the local bug.

## Regression

The full suite, including the existing blocking-bug and stage tests unchanged,
`chough cases check` and `fledge lanes run verify` on Linux with Fledge 1.8.0,
SpecSync 6.1.2 and hi 0.8.0 (its exercise step links its failure to a bug before
leaving test). Hosted CI is reported only as observed.

## Where these lessons go

- `specs/cli/context.md`
