---
id: 2026-11-19-a-story-s-verified-line-shows-limitations-bounded
title: A story's verified line shows limitations, bounded
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T01:17:05Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-349.md
  - .specsync/archive/changes/2026-11-19-fix-349/
record: .chough/archive/sessions/2026-11-18-fix-349.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# A story's verified line shows limitations, bounded

An archived story's What was verified line now shows an entry's limitations beside its claim, always on one line (#349). A person may give an entry at most 8 limitations of 512 bytes each; evidence add and cases record refuse more with nothing written, since a story writes each limitation twice. Tests failed first for the line, for multi-line text and at the bound; the verify lane passed on macOS after main was merged in, and tester2 passed the rechecks. Linux was not run locally. Limitations as a list in receipts, and the per-session overflow, are left for later.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-349.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-fix-349/)
