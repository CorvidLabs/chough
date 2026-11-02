---
change: human-gate-hold-stage-advance-and-skip-refuse-while-the-checkpoint-has-an-open-awaiting-human-gate-issue-38
artifact: context
---

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
