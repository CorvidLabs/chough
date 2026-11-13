---
id: 2026-11-06-rotate-the-retention-ledger-to-its-newest-100-entries-know-23
title: Rotate the retention ledger to its newest 100 entries (KNOW-23)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T01:46:41Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-rotate-the-retention-ledger-keep-the-newest-100-entries-and.md
  - .specsync/archive/changes/2026-11-06-rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their/
record: .chough/archive/sessions/2026-11-06-rotate-the-retention-ledger-keep-the-newest-100-entries-and.md
actor:
  - agent: claude-code
    terminal: t382
---
# Rotate the retention ledger to its newest 100 entries (KNOW-23)

The retention ledger now rotates: it keeps the newest 100 entries and moves older summarized ones into the vault with their archive (KNOW-23). The maintainer chose 100 as the limit and re-approved the plan.

All three cases failed first on main and then passed on Linux: the newest entries stay while the rest move into the vault, moved entries are still read, and a leftover from an interrupted rotation is read once.

The maintainer accepted PR #127 on 2026-11-05, choosing to record and merge. The change was archived and the story closed as done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-rotate-the-retention-ledger-keep-the-newest-100-entries-and.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their/)
