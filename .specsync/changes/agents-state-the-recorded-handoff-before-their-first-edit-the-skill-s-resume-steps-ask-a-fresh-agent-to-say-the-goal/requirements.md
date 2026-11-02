---
change: agents-state-the-recorded-handoff-before-their-first-edit-the-skill-s-resume-steps-ask-a-fresh-agent-to-say-the-goal
artifact: requirements
---

# Requirements

One requirement is modified; none is added or removed. See `deltas/cli.md`.

- REQ-cli-002 (focus context for resumption) gains one sentence and one acceptance
  criterion: the shipped skill asks an agent resuming work to state, in its own
  message before its first edit, the goal, the binding decisions with their reasons,
  the last failure with its bug status, the lessons that apply and the next task, and
  to say what the records do not tell it rather than guess. Trace: LIFE-3 ("resume
  work with focused context, decisions, open questions, and the next action"),
  FLOW-13 (a fresh agent can resume under the same scope, preferences and gates),
  LIFE-1, FLOW-12.

Unchanged and relied on: REQ-cli-021 (the next view and its blocks stay byte for
byte), REQ-cli-007 (checkpoint schema 2), REQ-cli-016 and REQ-cli-019 (host copies,
known earlier digests, no dashes). No HI criterion is added or reworded; HI owns
wording.
