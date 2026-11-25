---
id: 2026-11-24-fix-chough-437-a-direct-write-entry-writes-before-state-and
title: "Fix chough#437: a direct-write entry writes before_state and…"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-24
time: 2026-11-24T14:18:39Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-23-fix-437.md
  - .specsync/archive/changes/2026-11-24-fix-437/
record: .chough/archive/sessions/2026-11-23-fix-437.md
actor:
  - terminal: t14z
---
# Fix chough#437: a direct-write entry writes before_state and…

Fix chough#437: a direct-write entry writes before_state and after_state as none when the state is absent, in the Timeline and its text

Why: 0xLeif decided "Approve the scope of fix-437": "I approve" (pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change approve fix-437 as 0xLeif

Verified: Ran cargo test --locked --bin chough -- --exact direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none for TC-cli-direct-write-absent-state-is-none: passed (exit 0). - 87359ca16450; limits: outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected; inherited environment variables were not recorded

Read more: [the story](../../.chough/archive/sessions/2026-11-23-fix-437.md) · [the archived change](../../.specsync/archive/changes/2026-11-24-fix-437/)
