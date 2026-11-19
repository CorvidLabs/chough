---
id: 2026-11-18-a-credential-glued-to-a-hyphen-is-a-leak
title: A credential glued to a hyphen is a leak
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T01:07:18Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-267.md
  - .specsync/archive/changes/2026-11-18-fix-267/
record: .chough/archive/sessions/2026-11-18-fix-267.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# A credential glued to a hyphen is a leak

A token right after a hyphen, as in x-ghp_..., passed the leak guard and the doctor (#267). A hyphen is now a boundary at both matcher sites, as REQ-records-006 already required. Two tests failed on main first; the verify lane passed on macOS at dd4ed17, before main was merged in.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-267.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-267/)
