---
id: human-gate-hold-stage-advance-and-skip-refuse-while-the-checkpoint-has-an-open-awaiting-human-gate-issue-38
state: implementing
type: bug_fix
base_commit: 825a3805530456005e361dfa6d96d93eee52cd09
---

# Human-gate hold: stage advance and skip refuse while the checkpoint has an open awaiting-human gate (issue 38)

## Intent

Human-gate hold: stage advance and skip refuse while the checkpoint has an open awaiting-human gate (issue 38)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- stage advance and stage skip refuse with refused:awaiting-human at every main stage and release while awaiting_human is not empty, naming up to 3 open gates and a count of the rest and saying to record the decision and clear the gate
- After chough checkpoint --clear-awaiting-human the same move is allowed and gated as before; back, reset, iterate, pause, resume and side-stage enter and return are not refused by it
- chough next and chough status keep the Waiting for a human decision action and say that advance and skip are refused until the gate is cleared
- A Linux reproduction of issue 38 and tests fail before the fix and pass after it; fledge lanes run verify passes; hosted CI is reported only as observed

## No-spec Rationale

Not applicable
