---
id: 2026-11-24-fix-chough-440-the-stage-advance-to-go-back-hint-must-name
title: "Fix chough#440: the stage advance --to go-back hint must name…"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-24
time: 2026-11-24T14:24:29Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-23-fix-440.md
  - .specsync/archive/changes/2026-11-24-fix-440/
record: .chough/archive/sessions/2026-11-23-fix-440.md
actor:
  - terminal: t14z
---
# Fix chough#440: the stage advance --to go-back hint must name…

Fix chough#440: the stage advance --to go-back hint must name --reason like every other back/reset hint

Why: 0xLeif decided "scope approval for fix-440: the stage advance --to go-back hint must name --reason like every other back/reset hint": "I approve" (pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch); recorded by pi-t14z; a relayed claim, not authenticated

Verified: Ran cargo test --locked --test advance_to -- --exact a_refused_step_refuses_the_whole_batch_with_nothing_recorded for TC-cli-advance-to-not-ahead-hint-names-reason: passed (exit 0). - 297213e0122c; limits: outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected; inherited environment variables were not recorded

Read more: [the story](../../.chough/archive/sessions/2026-11-23-fix-440.md) · [the archived change](../../.specsync/archive/changes/2026-11-24-fix-440/)
