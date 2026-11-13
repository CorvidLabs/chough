---
change: 0-6-1-the-intent-gate-summary-gives-each-criterion-s-sentence-no-diff-line-without-a-base-and-adds-or-modifies-said-as
artifact: context
---

# Context

In Chough 0.6.0's fourth fresh-eyes round (t38n, F7), the intent gate's summary gave
criterion IDs ("Intent: TALLY-2", "hi/tally.md: TALLY-2") without the sentences being
approved, printed "Diff: unknown, with no merge base and no recorded base commit", and a
delta that only adds said "adds or modifies". The bug is recorded as
`.chough/bugs/2026-11-12-the-intent-gate-summary-names-criterion-ids-without-their.md`,
deferred to 0.6.1 by Leif. On 2026-11-13T02:03Z he chose "Yes, start (Recommended)" for
0.6.1 as six small PRs; this is one.
