---
id: 2026-11-19-with-chough-session-or-session-set-and-no-session-open-yet
title: With CHOUGH_SESSION or --session set and no session open yet
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T00:20:08Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-334.md
  - .specsync/archive/changes/2026-11-19-fix-334/
record: .chough/archive/sessions/2026-11-18-fix-334.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# With CHOUGH_SESSION or --session set and no session open yet

With CHOUGH_SESSION or --session set and no session open yet, the refusal names the variable and gives chough session new <key> (REQ-cli-037, #334)

Why: 0xLeif decided "Scope approval of fix-334": "Approve" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his scope approval of CorvidLabs/chough#334: the refusal names the variable and gives the session-new guidance); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-334 as 0xLeif

Verified: Ran cargo test --locked --test session_pin -- --exact a_pinned_session_before_any_session_names_session_new for TC-cli-a-pinned-session-before-any-session-names-session-new: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-334.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-fix-334/)
