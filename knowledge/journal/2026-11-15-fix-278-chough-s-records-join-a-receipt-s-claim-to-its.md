---
id: 2026-11-15-fix-278-chough-s-records-join-a-receipt-s-claim-to-its
title: "Fix #278: Chough's records join a receipt's claim to its evidence…"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-15
time: 2026-11-15T03:45:43Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-15-fix-278-receipt-dash.md
  - .specsync/archive/changes/2026-11-15-chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with/
record: .chough/archive/sessions/2026-11-15-fix-278-receipt-dash.md
actor:
  - agent: claude-code
    terminal: t28
---
# Fix #278: Chough's records join a receipt's claim to its evidence…

Describe the current objective before beginning work.

Why: 0xLeif decided "Approve the scope of chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with": "yes" (Claude Code chat (hc t28), 2026-11-14, reply to the #278 scope summary); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with as 0xLeif

Verified: Ran cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt for TC-history-archived-stories-join-evidence-without-an-em-dash: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-15-fix-278-receipt-dash.md) · [the archived change](../../.specsync/archive/changes/2026-11-15-chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with/)
