---
id: 2026-11-20-chough-moves-to-specsync-6-4-0-and-change-ready-asks-its-dry
title: Chough moves to SpecSync 6.4.0, and change ready asks its dry run
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T23:45:13Z
kind: shipped
links:
  - 429
  - .chough/archive/sessions/2026-11-20-specsync-6-4.md
  - .specsync/archive/changes/2026-11-20-specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments/
record: .chough/archive/sessions/2026-11-20-specsync-6-4.md
actor:
  - agent: t4c
    model: claude-opus-5-5
    terminal: t4c
---
# Chough moves to SpecSync 6.4.0, and change ready asks its dry run

chough change ready names what SpecSync 6.4.0's approval dry run refuses, in its words, in place of its own delta-set rule (#406, #307); it leaves SpecSync's own untracked change.lock and workflow-v2-baseline.json and another active change's work out of the paths check, a whole-project scope leaving nothing out; it no longer touches .specsync/hashes.json, which 6.4.0 keeps outside the work tree. Every pin names 6.4.0, and the spec merge line goes since 6.4.0 records history in fragments (#404).

Verified: the tester's PASS on 66 real changes at d4f520f6; tester2's recheck at d1f953a4; corvid-agent's approval at d1f953a4; main 51352d33 merged in, own lines the same; full lane green with 6.4.0 at 7eb2db10.

Read more: the pull request 429 · [the story](../../.chough/archive/sessions/2026-11-20-specsync-6-4.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments/)
