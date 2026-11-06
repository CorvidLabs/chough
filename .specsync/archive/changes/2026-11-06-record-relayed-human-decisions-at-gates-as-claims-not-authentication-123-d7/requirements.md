---
change: record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7
artifact: requirements
---

# Requirements

- ADDED REQ-cli-050 (`deltas/cli.md`): `chough checkpoint --close-gate` with the four
  relay values closes one gate, records `relayed_by` on its gate-closed event and one
  decision, and never touches SpecSync or a stage gate (FLOW-16).
- MODIFIED REQ-cli-030: the awaiting-human refusal and waiting action name
  `--close-gate` for recording a decision; `--clear-awaiting-human` withdraws gates.
- ADDED REQ-history-010 (`deltas/history.md`): the optional `relayed_by` field on
  gate-closed transitions and its payload validation.
- Narrowed wording: cli.spec.md invariant 2 becomes "never creates native approval,
  review, release or deployment records and never authenticates anyone; a relayed
  decision it keeps is a recorded claim and never satisfies a native gate."
