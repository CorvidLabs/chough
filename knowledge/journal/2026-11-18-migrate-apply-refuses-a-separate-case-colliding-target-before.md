---
id: 2026-11-18-migrate-apply-refuses-a-separate-case-colliding-target-before
title: migrate --apply refuses a separate case-colliding target before writing
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T19:26:09Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-263.md
  - .specsync/archive/changes/2026-11-18-fix-263/
record: .chough/archive/sessions/2026-11-18-fix-263.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# migrate --apply refuses a separate case-colliding target before writing

On a case-sensitive file system a separate knowledge/index.md passed the pre-write check, and the apply removed the 0.5.x files before failing on it, leaving a receipt for bytes it never wrote (#263). The check now exempts a case-colliding target only when it is the removed file under its other spelling. The test fails on main and passes with the fix in the chough-linux:3 image; the verify lane passed on macOS after main (169062c) was merged in. The full lane was not run on Linux locally.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-263.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-263/)
