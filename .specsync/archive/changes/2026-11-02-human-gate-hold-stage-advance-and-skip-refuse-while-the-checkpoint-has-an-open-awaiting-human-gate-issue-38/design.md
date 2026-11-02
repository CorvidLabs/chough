---
change: human-gate-hold-stage-advance-and-skip-refuse-while-the-checkpoint-has-an-open-awaiting-human-gate-issue-38
artifact: design
---

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
