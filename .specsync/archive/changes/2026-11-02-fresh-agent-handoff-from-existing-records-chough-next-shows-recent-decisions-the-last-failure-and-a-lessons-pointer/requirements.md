---
change: fresh-agent-handoff-from-existing-records-chough-next-shows-recent-decisions-the-last-failure-and-a-lessons-pointer
artifact: requirements
---

# Requirements

Two requirements are modified; none is added or removed. See `deltas/cli.md`.

- REQ-cli-021 (focused next action) gains the three handoff blocks (recent decisions,
  last failure, lessons pointer), byte-identical output for the same records with no
  ordering by file modification time, and the archived-not-done rule for main stages
  before done. Trace: LIFE-3 ("resume work with focused context, decisions, open
  questions, and the next action rather than loading the whole history"), FLOW-13
  (a fresh agent can resume under the same scope, preferences and gates), FLOW-2,
  LIFE-7, and now LIFE-1 and FLOW-12.
- REQ-cli-002 (focus context for resumption) names the lesson source (`<module>.3md`
  where present, otherwise the module `context.md` Lessons section), says the helper
  points at it and keeps no lesson store, index or ledger, and requires the shipped
  skill to state the certainty labels (observation, hypothesis, verified), source and
  limits, and that a project decision outranks a lesson. Trace: LIFE-3, LIFE-6, and
  now LIFE-1 ("... delivery, and lessons without losing why it matters") and FLOW-12.

Unchanged and relied on: REQ-cli-007 (checkpoint schema 2 stays; decisions stay free
text), REQ-cli-015 (closeout works with the librarian off; checkpoints and resume never
depend on it), REQ-cli-016 and REQ-cli-019 (host copies, known earlier digests, no
dashes), REQ-cli-012 (owned by PR #26, untouched). No HI criterion is added or
reworded; HI owns wording.

Canonical heading normalization (made after approval, before the scoped `change
check`, because `specsync change audit` refuses canonical edits under a draft): in
`specs/cli/requirements.md` the heading line of REQ-cli-002, which today carries an
em dash (U+2014) and the title `Focus context for resumption`, becomes exactly

```text
### REQ-cli-002
```

so the MODIFIED item applies. The requirement's title moves into the first sentence of
its body, as the delta shows. No other canonical text is edited by hand.
