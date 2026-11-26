---
id: 2026-11-24-fix-chough-438-chough-gate-record-closes-all-the-named-gates
title: "Fix chough#438: chough gate record closes all the named gates or none"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-24
time: 2026-11-24T14:20:12Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-23-fix-438.md
  - .specsync/archive/changes/2026-11-24-fix-438/
record: .chough/archive/sessions/2026-11-23-fix-438.md
actor:
  - terminal: t14z
---
# Fix chough#438: chough gate record closes all the named gates or none

Fix chough#438: chough gate record closes all the named gates or none, as REQ-cli-061 requires

Why: 0xLeif decided "Approve the scope of fix-438": "I approve" (pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change approve fix-438 as 0xLeif

Verified: Ran cargo test --locked --test gates -- --exact a_second_close_over_the_session_bound_refuses_the_whole_reply_with_nothing_written for TC-cli-gate-record-all-or-none: passed (exit 0). - 80ab38c66f6b; limits: outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected; inherited environment variables were not recorded

Read more: [the story](../../.chough/archive/sessions/2026-11-23-fix-438.md) · [the archived change](../../.specsync/archive/changes/2026-11-24-fix-438/)
