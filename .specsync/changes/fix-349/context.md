---
change: fix-349
artifact: context
---

# Context

GitHub #349, part 1, a follow-up to #342 (#286). Since #342 an archived story's
receipts keep each evidence entry's limitations, in the `## Receipts` block.
The human-readable What was verified line still showed only the claim, the
command and the fingerprint digits, so a reader had to read the receipts to see
what a claim did not cover.

Leif's decision in #349: "yes. The archived story's human-readable What was
verified line shows an entry's limitations next to its claim". On the scope of
this change, relayed by his planning session (hc t4c) on 2026-11-18: "Part 1
now; 2 and 3 later". Part 2 (limitations as a list in receipts) has a design
note on the issue; part 3 (a bound) waits for his number.

The line prints the receipt's `limitations` value as it is stored.
