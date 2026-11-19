---
id: 2026-11-18-each-shell-pins-its-session-with-session-or-chough-session
title: Each shell pins its session with --session or CHOUGH_SESSION
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T19:02:36Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-271.md
  - .specsync/archive/changes/2026-11-18-fix-271/
record: .chough/archive/sessions/2026-11-18-fix-271.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# Each shell pins its session with --session or CHOUGH_SESSION

Each shell pins its session with --session or CHOUGH_SESSION, whatever the shared pointer says (REQ-cli-037, #271)

Why: The per-shell option is a global --session, which clap leaves out of init and inbox add because they define their own --session, so both keep their meanings; CHOUGH_SESSION is read once in main beside it, and session new and session use ignore it. The pin resolves in sessions::active and never moves the pointer.

Verified: Ran cargo test --locked --test session_pin -- --exact a_pinned_shell_writes_its_own_session_whatever_the_pointer_says for TC-cli-a-pinned-shell-writes-its-own-session: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-271.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-271/)
