---
id: 2026-11-18-bug-add-checks-that-the-active-session-can-take-another-bug
title: bug add checks that the active session can take another bug link…
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T23:33:51Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-329-check-first.md
  - .specsync/archive/changes/2026-11-18-fix-329-check-first/
record: .chough/archive/sessions/2026-11-18-fix-329-check-first.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# bug add checks that the active session can take another bug link…

bug add checks that the active session can take another bug link before writing the bug, and refuses with nothing written (#343 review P2)

Why: 0xLeif decided "Scope approval of fix-329-check-first": "Yes, for today" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: his standing rule for today, that a real gap his review thread finds in an already-approved PR is fixed inside its approved scope as a small follow-up change, applied by hc t4c to his P2 thread on #343 ('Preflight the checkpoint bound before creating the bug'), with t4c's design point: check that the session can take another bug link before writing anything, and refuse with nothing written); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran…

Verified: Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-329-check-first.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-329-check-first/)
