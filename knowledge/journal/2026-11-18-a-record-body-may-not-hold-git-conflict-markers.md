---
id: 2026-11-18-a-record-body-may-not-hold-git-conflict-markers
title: A record body may not hold Git conflict markers
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T20:33:03Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-265.md
  - .specsync/archive/changes/2026-11-18-fix-265/
record: .chough/archive/sessions/2026-11-18-fix-265.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# A record body may not hold Git conflict markers

A record body may not hold Git conflict markers: writes refuse naming the file, line and marker, and chough doctor reports committed records that hold them (REQ-records-001, #265)

Why: 0xLeif decided "Scope approval of fix-265": "Approve the scope.

Verified: Ran cargo test --locked --test migrate -- --exact migrate_refuses_a_plan_whose_files_would_hold_conflict_markers for TC-records-migrate-refuses-conflict-markers: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-265.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-265/)
