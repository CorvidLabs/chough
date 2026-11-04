---
change: trim-old-proof-automatically-when-a-change-reaches-done-with-the-librarian-on-and-let-retention-manual-opt-out-issue-88
artifact: requirements
---

# Requirements

ADDED REQ-cli-041 (see `deltas/cli.md`): `stage advance` into done trims cold archived
proof with the librarian on unless `retention` is `manual`, under the same summary rules
(ledger first, KNOW-10 restore, refused while a native change is open), and never undoes
the move. Trace: KNOW-13, KNOW-11, KNOW-7, KNOW-5.
