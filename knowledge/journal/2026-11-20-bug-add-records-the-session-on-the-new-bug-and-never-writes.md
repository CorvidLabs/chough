---
id: 2026-11-20-bug-add-records-the-session-on-the-new-bug-and-never-writes
title: bug add records the session on the new bug and never writes the session
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T17:04:27Z
kind: shipped
links:
  - 422
  - .chough/archive/sessions/2026-11-20-bug-add-leaves-the-session.md
  - .specsync/archive/changes/2026-11-20-bug-add-leaves-the-session/
record: .chough/archive/sessions/2026-11-20-bug-add-leaves-the-session.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# bug add records the session on the new bug and never writes the session

bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)

Why: 0xLeif decided "Approve the scope of bug-add-leaves-the-session": "A: on the bug (Recommended)" (Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 14:13Z, relayed by hc t4c through fable-lead (hc t53); the question as shown: 'Chough R2a (#382, #395, two regressions from #343).

Verified: Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1).

Read more: the pull request 422 · [the story](../../.chough/archive/sessions/2026-11-20-bug-add-leaves-the-session.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-bug-add-leaves-the-session/)
