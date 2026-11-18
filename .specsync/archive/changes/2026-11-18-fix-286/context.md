---
change: fix-286
artifact: context
---

# Context

GitHub #286, found in corvid-agent's review of #285. When a session is archived,
each evidence entry becomes a receipt in the story. The receipt kept the claim,
result, command, revision, environment, digests, case and times, and dropped the
entry's `criteria`, `requirements` and `limitations`. The full entry survived
only in Git history. So an archived record read stronger than the evidence was,
and a reader could not go from a story to the requirement or criterion it
proved.

Leif's decision, in his comment on #286 on 2026-11-18 from his planning
interview: "keep all three. Archived receipts keep the entry's `criteria`,
`requirements` and `limitations`, so the story says what was verified against
what and its honest limits stay visible. This adds the three to REQ-history-006's
receipt field list."

The story's What was verified line is not changed. The issue also suggested
naming a limitation on that line; the decision is about the receipt fields.

A receipt's values are scalars, so the two ID lists are written as `aliases`
already is, comma-and-space separated, and limitations, which are free text and
may hold commas, are separated by a semicolon and a space.
