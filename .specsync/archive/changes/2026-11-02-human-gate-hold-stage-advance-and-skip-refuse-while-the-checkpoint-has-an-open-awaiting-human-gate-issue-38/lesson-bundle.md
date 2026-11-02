# Lesson bundle: human-gate-hold-stage-advance-and-skip-refuse-while-the-checkpoint-has-an-open-awaiting-human-gate-issue-38

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Human-gate hold: stage advance and skip refuse while the checkpoint has an open awaiting-human gate (issue 38)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/stage.rs, tests/stages.rs, specs/cli/, docs/cli.md, CHANGELOG.md
- **Acceptance**: stage advance and stage skip refuse with refused:awaiting-human at every main stage and release while awaiting_human is not empty, naming up to 3 open gates and a count of the rest and saying to record the decision and clear the gate
- **Acceptance**: After chough checkpoint --clear-awaiting-human the same move is allowed and gated as before; back, reset, iterate, pause, resume and side-stage enter and return are not refused by it
- **Acceptance**: chough next and chough status keep the Waiting for a human decision action and say that advance and skip are refused until the gate is cleared
- **Acceptance**: A Linux reproduction of issue 38 and tests fail before the fix and pass after it; fledge lanes run verify passes; hosted CI is reported only as observed

## Evidence

- Verification commit: `d8c6e1e48b31ee4a1047bc2db495af7a762ed535`
- Base commit: `825a3805530456005e361dfa6d96d93eee52cd09`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #38 (CorvidLabs/chough), filed by corvid-agent from a Linux dogfood run.
`chough checkpoint --awaiting-human "<gate>"` records a gate and `chough next` says
"Waiting for a human decision ... Do not record it on their behalf", but `stage
advance` and `stage skip` never read `awaiting_human`. Intent and design have no
native record, so nothing stops an agent from leaving them before the human
answers, and the open gate is carried into later stages.

Reproduced on Linux (x86_64) at main 825a380 with SpecSync 6.1.2, Fledge 1.8.0 and
hi 0.8.0 in a disposable copy of `examples/shipping` (SDD off): with an open
gate, `stage advance` applies intent -> design (exit 0), and `stage skip` applies
plan -> build (`ungated:sdd-disabled`). The log is `.chough/artifacts/bug38-repro.log`.

Proposed answers (posted on #38 as proposals; binding through the scope approval
of this definition): enforce rather than leave it advisory; refuse advance and skip
with `refused:awaiting-human`; no reason flag bypasses it, because an
agent-supplied reason is the reported bypass; the gate is closed by clearing it,
with the decision recorded in the checkpoint. Other moves stay available so
independent work can continue while waiting.

Ruled out: a `--reason` override; warning only; refusing every stage move (back,
reset and side stages are how work continues safely while waiting).

Approvals: per 0xLeif's chat instruction of 2026-11-02 9:14 AM MT ("Use corvid-agent
for all approvals for all things!! You are corvid-agent"), corvid-agent records the
native approval, review and finalize for this change.

Overlap: PR #43 (corvid-agent, #39) and PR #37 (0xLeif) edit `move_forward` in
`src/stage.rs`; this change adds one check at its start and one sentence to the
waiting action in `assess`. REQ-cli-008 is modified by #43, so the rule is a new
REQ-cli-030 (025 to 029 are reserved by #37, #32 and #42). #42 (SKILL.md,
references/lifecycle.md, README) is not touched; its standing-authorization guidance
still applies: an agent whose recorded authorization covers a gate clears it and
records that source.

## From the change's design.md

# Design

- `move_forward` in `src/stage.rs` (used by `advance`, `skip` and release -> done):
  after the active and main-stage checks and before computing the destination or
  any native read, refuse when `session.awaiting_human` is not empty:
  `{kind} refused (refused:awaiting-human): waiting for a human decision: <g1>; <g2>;
  <g3> (and N more); record the decision, then clear it with chough checkpoint
  --clear-awaiting-human (re-pass gates still waiting with --awaiting-human)`.
- `assess`: the existing waiting action gains one sentence: "`chough stage advance`
  and `chough stage skip` are refused until it is cleared."
- Not changed: `back`, `reset`, `iterate`, `pause`, `resume`, `enter`, `return`, the
  checkpoint schema, gate records, the next view's fields and bounds.

Estimate: small, about 20 lines plus tests. Risk: a textual merge with #43 and #37
in `move_forward`.

## From the change's testing.md

# Testing

| Case | Requirement | Criteria | Method | What it shows |
|---|---|---|---|---|
| TC-cli-open-human-gates-refuse-advance-and-skip | REQ-cli-030 | FLOW-3, FLOW-5, FLOW-13 | automated | Advance and skip are refused at intent, design, plan and release with nothing written; 3 gates named and the rest counted; clearing allows the move |
| TC-cli-open-human-gates-leave-other-moves-and-agree-in-next | REQ-cli-030 | FLOW-2, FLOW-13 | automated | Back, iterate, pause, resume, enter and return still work with a gate open; next and status give the same waiting action naming the refusal |

The cases are added after the scoped check materializes REQ-cli-030.

Reproduction: a disposable copy of `examples/shipping` with SDD off, an open gate at
intent, then `stage advance`, and at plan `stage skip`. Before the fix both apply;
after, both are refused.

Regression: the full suite unchanged, `chough cases check` and `fledge lanes run
verify` on Linux (Fledge 1.8.0, SpecSync 6.1.2, hi 0.8.0). The verify exercise
clears its scope gate before advancing. Hosted CI is reported only as observed.

## Where these lessons go

- `specs/cli/context.md`
