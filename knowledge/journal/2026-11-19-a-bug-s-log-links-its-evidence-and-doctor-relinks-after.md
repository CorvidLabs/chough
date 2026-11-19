---
id: 2026-11-19-a-bug-s-log-links-its-evidence-and-doctor-relinks-after
title: A bug's Log links its evidence, and doctor relinks after archiving
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T02:53:52Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-347.md
  - .specsync/archive/changes/2026-11-19-fix-347/
record: .chough/archive/sessions/2026-11-18-fix-347.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# A bug's Log links its evidence, and doctor relinks after archiving

A bug's Log entry names each piece of linked evidence as a Markdown link relative to the bug's file, chosen after the evidence is resolved: the open session's entry, or the archived story that holds it as a receipt. Doctor reports a bug whose body still links an entry that left with its archived session as stale_bug_links and appends one Log line linking the story; a bug too near its size bound gets advice instead.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-347.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-fix-347/)
