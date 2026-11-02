---
change: human-gate-hold-stage-advance-and-skip-refuse-while-the-checkpoint-has-an-open-awaiting-human-gate-issue-38
artifact: testing
---

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
