---
id: 2026-11-18-a-stale-gate-that-doctor-withdraws-keeps-the-reason
title: A stale gate that doctor withdraws keeps the reason
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T21:21:39Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-250.md
  - .specsync/archive/changes/2026-11-18-fix-250/
record: .chough/archive/sessions/2026-11-18-fix-250.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# A stale gate that doctor withdraws keeps the reason

chough doctor --fix --reason withdrew stale gates and wrote the reason nowhere (#250). Each withdrawn gate's gate-closed Timeline entry now holds it as its withdrawn field, so the archived story carries it, and doctor's JSON echoes it as decision_reason. The reason passes the leak guard before anything is written. Two tests failed first; the verify lane passed on macOS after main was merged in, with the tests run after the deltas were materialized, and tester2 passed the recheck. The change was sent back once to rewrite its deltas onto main. Linux was not run locally.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-250.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-250/)
