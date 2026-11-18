---
id: 2026-11-18-a-migrated-bug-s-log-links-each-carried-observation
title: A migrated bug's Log links each carried observation
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T20:29:59Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-277.md
  - .specsync/archive/changes/2026-11-18-fix-277/
record: .chough/archive/sessions/2026-11-18-fix-277.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# A migrated bug's Log links each carried observation

A bug that chough migrate writes named its carried observations in its Log by bare 0.5.x ID (#277). The Log now links each to the open session's evidence entry or to the archived story that holds its receipt, relative to the bug's file; a receipt only the bug holds is named as its own. A new test failed on main first; the verify lane passed on macOS after main was merged in, and the tester passed it. Linux was not run locally.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-277.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-277/)
