---
id: 2026-11-06-keep-five-archived-sessions-roll-older-ones-into-the-vault
title: Keep five archived sessions, roll older ones into the vault
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T04:02:23Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-keep-the-five-most-recent-archived-sessions-and-roll-older.md
  - .specsync/archive/changes/2026-11-06-keep-the-five-most-recent-archived-sessions-and-roll-older-ones-into-the-knowledge-vault-know-29/
record: .chough/archive/sessions/2026-11-06-keep-the-five-most-recent-archived-sessions-and-roll-older.md
actor:
  - agent: claude-code
    terminal: t382
---
# Keep five archived sessions, roll older ones into the vault

Chough now keeps the five most recent archived sessions and rolls older ones into the knowledge vault (KNOW-29).

The new tests failed first, then both cases passed on Linux: the move into done keeps the five newest archived sessions and rolls the rest into the vault, archived sessions stay when the librarian is off, and only generated knowledge notes merge by union.

The maintainer accepted PR #135 on 2026-11-05, choosing to record and merge. The change was archived and the story closed as done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-keep-the-five-most-recent-archived-sessions-and-roll-older.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-keep-the-five-most-recent-archived-sessions-and-roll-older-ones-into-the-knowledge-vault-know-29/)
