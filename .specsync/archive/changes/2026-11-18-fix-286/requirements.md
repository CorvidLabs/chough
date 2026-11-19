---
change: fix-286
artifact: requirements
---

# Requirements

REQ-history-006 is modified, in `deltas/history.md`. Its list of receipt fields gains
`criteria` and `requirements` (each one scalar, comma-and-space separated) and
`limitations` (one scalar, semicolon-and-space separated), each when the entry has
any, after `case_digest`. Nothing else in it changes. REQ-cli-017, which lists
an evidence entry's fields, gains a clause saying the receipt keeps the three. The
records spec's Public API row for `Receipt` says the same of a migrated receipt.
