---
id: 2026-11-20-a-new-record-is-written-whole-or-not-at-all
title: A new record is written whole or not at all
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T18:58:07Z
kind: shipped
links:
  - 426
  - .chough/archive/sessions/2026-11-20-260-whole-or-nothing.md
  - .specsync/archive/changes/2026-11-20-fix-260-a-new-record-is-written-whole-or-not-at-all/
record: .chough/archive/sessions/2026-11-20-260-whole-or-nothing.md
actor:
  - agent: t4c
    model: claude-opus-5-5
    terminal: t4c
---
# A new record is written whole or not at all

chough bug add and journal add write a new record to .chough/local/ first, sync it, and give it its name without replacing anything, so a full disk or a file-size limit leaves no file at the name; where the file system cannot rename that way it is created at its name and removed on any error. No hard link is used, so exFAT works (#260, #424). An empty record is named as empty, with the way out.

Verified: the tester's PASS at 829d479b; corvid-agent's approval and Linux run; its four notes taken at 73145abd (the temporary file goes through the symlink guard); main de49485f merged in at 50f0fd0c, own lines byte-identical; full lane green at 34fd2a92.

Read more: the pull request 426 · [the story](../../.chough/archive/sessions/2026-11-20-260-whole-or-nothing.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-fix-260-a-new-record-is-written-whole-or-not-at-all/)
