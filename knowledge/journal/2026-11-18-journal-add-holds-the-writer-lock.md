---
id: 2026-11-18-journal-add-holds-the-writer-lock
title: journal add holds the writer lock
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T00:58:31Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-272.md
  - .specsync/archive/changes/2026-11-18-fix-272/
record: .chough/archive/sessions/2026-11-18-fix-272.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# journal add holds the writer lock

chough journal add took no writer lock, so several adds at once left the feed missing posts (#272). It now holds the lock REQ-cli-040 already named for it. Two tests failed on main first; the verify lane passed on macOS at 3f3f95c. Linux and CI were not run locally.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-272.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-272/)
