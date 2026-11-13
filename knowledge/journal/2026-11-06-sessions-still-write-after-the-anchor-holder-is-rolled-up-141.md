---
id: 2026-11-06-sessions-still-write-after-the-anchor-holder-is-rolled-up-141
title: Sessions still write after the anchor holder is rolled up (#141)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T05:10:43Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-find-the-legacy-history-anchor-in-sessions-rolled-into-the.md
  - .specsync/archive/changes/2026-11-06-find-the-legacy-history-anchor-in-sessions-rolled-into-the-vault-or-shelved-141-p1/
record: .chough/archive/sessions/2026-11-06-find-the-legacy-history-anchor-in-sessions-rolled-into-the.md
actor:
  - agent: claude-code
    terminal: t382
---
# Sessions still write after the anchor holder is rolled up (#141)

Issue #141, a P1: once the session holding the legacy history anchor was rolled into the vault or shelved, Chough refused to write sessions. The anchor lookup now also searches rolled and shelved sessions.

The new test, that sessions still write after the anchor holder rolls into the vault or is shelved, failed first with the P1 refusal and then passed on Linux. The SpecSync change check passed.

The maintainer accepted PR #142 on 2026-11-05 with a passing review, choosing to record and merge. The change was archived and the story closed as done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-find-the-legacy-history-anchor-in-sessions-rolled-into-the.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-find-the-legacy-history-anchor-in-sessions-rolled-into-the-vault-or-shelved-141-p1/)
