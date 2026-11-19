---
change: fix-264
artifact: requirements
---

# Requirements

REQ-records-009 is modified, in `deltas/records.md`. Two sentences change and
nothing else:

- The apply checks every file it would write, the receipt included, before it
  writes any, and refuses with nothing written when one already exists with
  other bytes. A file that appears during the apply still refuses with nothing
  more written.
- The receipt's path is named by the plan, so the digest covers it:
  `<YYYY-MM-DD>-from-0-5.md`, or, while a receipt of that name exists for
  another plan, the first free `<YYYY-MM-DD>-from-0-5-<n>.md` with `<n>` from 2.
  A receipt this plan's own interrupted apply wrote is used again.
