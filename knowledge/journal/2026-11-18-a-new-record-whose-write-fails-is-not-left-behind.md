---
id: 2026-11-18-a-new-record-whose-write-fails-is-not-left-behind
title: A new record whose write fails is not left behind
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T14:10:31Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-260.md
  - .specsync/archive/changes/2026-11-18-fix-260/
record: .chough/archive/sessions/2026-11-18-fix-260.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# A new record whose write fails is not left behind

A bug add whose write failed part way left a partial record at the name (#260). A new record is now written to a private temporary file and linked into place, and a failure removes only the temporary this call created. Tests failed first, including both review findings on #306; the verify lane passed on macOS after main (c78f878) was merged in. Linux and a real full disk were not run locally.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-260.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-260/)
