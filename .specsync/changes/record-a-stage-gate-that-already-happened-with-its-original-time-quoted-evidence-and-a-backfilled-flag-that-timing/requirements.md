---
change: record-a-stage-gate-that-already-happened-with-its-original-time-quoted-evidence-and-a-backfilled-flag-that-timing
artifact: requirements
---

# Requirements

ADDED REQ-cli-036 (`deltas/cli.md`): `chough stage backfill` records a move to the next
main stage that already happened, with its original time and the commit and quoted
choice it rests on. It applies every refusal `advance` applies, including the
destination's native prerequisite read now, refuses a future or out-of-order time, an
unreachable commit and a blank quote, and records no human approval, review, consent
or gate. `timing` lists backfilled moves separately, marks the segments they bound and
keeps those stages out of estimates.

ADDED REQ-history-005 (`deltas/history.md`): the transition record's optional
`backfill` object, omitted when absent so other records keep their bytes, and carried
by the legacy log and by history events.
