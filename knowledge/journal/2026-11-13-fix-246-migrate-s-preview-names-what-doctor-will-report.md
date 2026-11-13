---
id: 2026-11-13-fix-246-migrate-s-preview-names-what-doctor-will-report
title: "Fix #246: migrate's preview names what doctor will report"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-13
time: 2026-11-13T14:17:18Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-13-fix-246-migrate-preview.md
  - .specsync/archive/changes/2026-11-13-fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply/
record: .chough/archive/sessions/2026-11-13-fix-246-migrate-preview.md
actor:
  - agent: claude-code
    terminal: t382
---
# Fix #246: migrate's preview names what doctor will report

chough migrate's preview and receipt name each session chough doctor will report right after the apply: finished sessions to archive with chough doctor --fix, and stale-gated ones with each gate's text and chough doctor --fix --reason, from doctor's own session check (REQ-records-009, REQ-cli-055).

Verified: the case red then green, the bug verified, and the full suite 736 of 736. Next: Leif's review and merge.

Read more: [the story](../../.chough/archive/sessions/2026-11-13-fix-246-migrate-preview.md) · [the archived change](../../.specsync/archive/changes/2026-11-13-fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply/)
