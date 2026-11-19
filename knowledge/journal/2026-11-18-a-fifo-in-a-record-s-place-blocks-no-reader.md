---
id: 2026-11-18-a-fifo-in-a-record-s-place-blocks-no-reader
title: A FIFO in a record's place blocks no reader
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T01:15:47Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-262.md
  - .specsync/archive/changes/2026-11-18-fix-262/
record: .chough/archive/sessions/2026-11-18-fix-262.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# A FIFO in a record's place blocks no reader

A FIFO in .chough/bugs/ blocked status and next for ever, and a session whose session.md was one vanished from session list (#262). Readers no longer open a non-regular file, and such a session is listed with the reason and the shelve recovery, as REQ-history-006 required. Two tests failed on main first; the verify lane passed on macOS at 448f47c, before main was merged in.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-262.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-262/)
