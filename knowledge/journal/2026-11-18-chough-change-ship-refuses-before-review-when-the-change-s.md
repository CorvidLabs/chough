---
id: 2026-11-18-chough-change-ship-refuses-before-review-when-the-change-s
title: chough change ship refuses before review when the change's history…
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T22:50:07Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-298.md
  - .specsync/archive/changes/2026-11-18-fix-298/
record: .chough/archive/sessions/2026-11-18-fix-298.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# chough change ship refuses before review when the change's history…

chough change ship refuses before review when the change's history holds a change both open and archived in one commit (REQ-cli-059, #298)

Why: SpecSync reports terminal-evidence validity only after it archives, and its ship --dry-run stops before the review, so Chough checks the change's own history (base..HEAD) for a change both open and archived in one commit, as an interim until SpecSync's preflight (spec-sync#1082).

Verified: Ran cargo test --locked --test change -- --exact ship_refuses_a_history_with_a_change_both_open_and_archived for TC-cli-ship-refuses-a-corrupt-history: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-298.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-298/)
