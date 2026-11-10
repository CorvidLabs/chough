---
change: know-29-roll-up-keeps-or-flags-tracked-session-records-when-the-librarian-vault-is-untracked-165
artifact: research
---

# Research

Leif's decision on the issue (2026-11-10, recorded by the coordinator): "copy, never move" — tracked session records stay put; the vault gets a copy or a link. The implementation hard-links the vault entry and keeps the source, so bytes are shared and the tree is unchanged. Consequence flagged for acceptance: a rolled session stays listed by `session list` and stays a retention reference source, since its record no longer leaves the archive directory.
