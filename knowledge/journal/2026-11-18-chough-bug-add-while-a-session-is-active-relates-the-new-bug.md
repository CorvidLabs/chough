---
id: 2026-11-18-chough-bug-add-while-a-session-is-active-relates-the-new-bug
title: chough bug add while a session is active relates the new bug to that…
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T23:04:25Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-329.md
  - .specsync/archive/changes/2026-11-18-fix-329/
record: .chough/archive/sessions/2026-11-18-fix-329.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# chough bug add while a session is active relates the new bug to that…

chough bug add while a session is active relates the new bug to that session, so a blocking bug stops it; with no session active it stays unlinked (REQ-cli-012, REQ-cli-054, #329)

Why: 0xLeif decided "Scope approval of fix-329": "auto-link. chough bug add while a session is active relates the new bug to that session, so a blocking bug stops it. A bug filed with no session active stays unlinked and gets the existing warning.

Verified: Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-329.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-329/)
