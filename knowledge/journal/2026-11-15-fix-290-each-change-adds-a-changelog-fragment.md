---
id: 2026-11-15-fix-290-each-change-adds-a-changelog-fragment
title: "Fix #290: each change adds a changelog fragment"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-15
time: 2026-11-15T16:41:23Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-15-fix-290-changelog-fragments.md
  - .specsync/archive/changes/2026-11-15-each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290/
record: .chough/archive/sessions/2026-11-15-fix-290-changelog-fragments.md
actor:
  - agent: claude-code
    terminal: t28
---
# Fix #290: each change adds a changelog fragment

Describe the current objective before beginning work.

Why: 0xLeif decided "Approve the scope of each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290": "yes" (Claude Code chat (hc t28), 2026-11-15, reply to the #290 scope summary); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290 as 0xLeif

Verified: On d081c91, cargo test --test changelog 18 passed after Grok's 8 findings were fixed; fmt and lint clean; repro of same-day #99/#100 now assembles newest first

Read more: [the story](../../.chough/archive/sessions/2026-11-15-fix-290-changelog-fragments.md) · [the archived change](../../.specsync/archive/changes/2026-11-15-each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290/)
