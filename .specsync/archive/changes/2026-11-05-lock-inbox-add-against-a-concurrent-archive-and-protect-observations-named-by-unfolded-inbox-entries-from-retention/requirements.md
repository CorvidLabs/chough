---
change: lock-inbox-add-against-a-concurrent-archive-and-protect-observations-named-by-unfolded-inbox-entries-from-retention
artifact: requirements
---

# Requirements

ADDED REQ-history-009 (see `deltas/history.md`): `inbox add` holds the checkpoint writer
lock while it checks and writes, and retention protects observations named by unfolded
inbox entries. Trace: DRIVE-3, LIFE-8.
