---
id: 2026-11-18-a-fifo-swapped-in-at-the-open-blocks-no-reader
title: A FIFO swapped in at the open blocks no reader
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T06:11:18Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-262-open-race.md
  - .specsync/archive/changes/2026-11-18-fix-262-open-race/
record: .chough/archive/sessions/2026-11-18-fix-262-open-race.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# A FIFO swapped in at the open blocks no reader

A record replaced by a FIFO between the file-type check and the open could still block session list and shelve (review finding on #305). Readers now open without blocking and check the opened handle. A unit test covers the open; the verify lane passed on macOS after main (347a005) was merged in. Linux was not run locally.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-262-open-race.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-262-open-race/)
