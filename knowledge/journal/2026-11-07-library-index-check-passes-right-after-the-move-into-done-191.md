---
id: 2026-11-07-library-index-check-passes-right-after-the-move-into-done-191
title: Library index check passes right after the move into done (#191)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-07
time: 2026-11-07T04:14:27Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-07-191-done-vault-current.md
  - .specsync/archive/changes/2026-11-07-after-the-move-into-done-chough-library-index-check-passes-with-no-command-in-between-session-notes-link-archive-notes/
record: .chough/archive/sessions/2026-11-07-191-done-vault-current.md
actor:
  - agent: claude-code
    terminal: t382
---
# Library index check passes right after the move into done (#191)

Issue #191, a review finding, asked that chough library index --check pass right after the move into done, with no command in between. Session notes now link archive notes from their sources, and the directory is regenerated after the session archives itself. The fix traces to the approved KNOW-4, that regenerating from the same sources gives identical bytes, so no new criterion was needed.

The maintainer approved the scope and the implementation review on PR #194. Another agent finished the move into done after an interrupted ship, and the story closed as done on 2026-11-07.

Read more: [the story](../../.chough/archive/sessions/2026-11-07-191-done-vault-current.md) · [the archived change](../../.specsync/archive/changes/2026-11-07-after-the-move-into-done-chough-library-index-check-passes-with-no-command-in-between-session-notes-link-archive-notes/)
