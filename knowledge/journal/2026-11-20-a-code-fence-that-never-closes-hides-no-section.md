---
id: 2026-11-20-a-code-fence-that-never-closes-hides-no-section
title: A code fence that never closes hides no section
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T19:34:47Z
kind: shipped
links:
  - 423
  - .chough/archive/sessions/2026-11-20-394-unclosed-fence.md
  - .specsync/archive/changes/2026-11-20-fix-394-a-code-fence-that-never-closes-hides-no-section/
record: .chough/archive/sessions/2026-11-20-394-unclosed-fence.md
actor:
  - agent: t4c
    model: claude-opus-5-5
    terminal: t4c
---
# A code fence that never closes hides no section

A code fence hides the lines up to its close only when it closes, in one rule the session reader and the bug merge driver share: a session whose Goal opens a fence that never closes saves any number of times with each section once, and two branches' Log entries on such a bug merge. The closes are found in one backward pass with a stack per fence character, linear for fences of any length (#394, #356).

Verified: corvid-agent's blocker on long fences fixed at 71e05c1b (its scenario 36.5 s down to 0.56 s) and its approval at 1999ea5d; the tester's PASS at 1999ea5d; main 50345a3b merged in at a70869e1, own lines the same per file; full lane green at 00667016.

Read more: the pull request 423 · [the story](../../.chough/archive/sessions/2026-11-20-394-unclosed-fence.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-fix-394-a-code-fence-that-never-closes-hides-no-section/)
