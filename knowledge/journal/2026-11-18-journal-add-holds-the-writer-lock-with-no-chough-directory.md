---
id: 2026-11-18-journal-add-holds-the-writer-lock-with-no-chough-directory
title: journal add holds the writer lock with no .chough directory
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T05:59:23Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-272-uninitialised-root.md
  - .specsync/archive/changes/2026-11-18-fix-272-uninitialised-root/
record: .chough/archive/sessions/2026-11-18-fix-272-uninitialised-root.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# journal add holds the writer lock with no .chough directory

In a project with no .chough directory, journal add took no lock, so parallel posts could still miss the feed after #302. It now takes the checkpoint lock directly, as REQ-cli-040 requires. Two tests failed first; the verify lane passed on macOS after main (5408c44) was merged in. Linux was not run locally.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-272-uninitialised-root.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-272-uninitialised-root/)
