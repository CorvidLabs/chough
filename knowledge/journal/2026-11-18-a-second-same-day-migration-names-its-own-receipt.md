---
id: 2026-11-18-a-second-same-day-migration-names-its-own-receipt
title: A second same-day migration names its own receipt
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T19:57:22Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-264.md
  - .specsync/archive/changes/2026-11-18-fix-264/
record: .chough/archive/sessions/2026-11-18-fix-264.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# A second same-day migration names its own receipt

A second chough migrate on one day wrote its session folder and then refused on the first receipt's name (#264). The plan now names its receipt, with a -<n> suffix while the name is taken by another plan, as part of its digest, and the apply checks the receipt with every other target before writing. Two tests failed on main first; the verify lane passed on macOS after main (275fe4d) was merged in, and tester2 passed it on macOS and in the Linux container. Every migrate plan digest changes with this change.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-264.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-264/)
