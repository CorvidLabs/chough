---
id: 2026-11-13-fix-247-the-example-waits-past-the-failing-run-s-second
title: "Fix #247: the example waits past the failing run's second"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-13
time: 2026-11-13T14:15:55Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-13-fix-247-example-race.md
  - .specsync/archive/changes/2026-11-13-fix-247-the-verify-lane-s-example-waits-past-the-failing-run-s-second-before-its-rerun/
record: .chough/archive/sessions/2026-11-13-fix-247-example-race.md
actor:
  - agent: claude-code
    terminal: t382
---
# Fix #247: the example waits past the failing run's second

examples/feature-bug-loop.sh waits until the clock is past the latest failure's recorded second before TC-shipping-0001's rerun, so the verify lane's exercise no longer flakes on REQ-cli-012's same-second rule, which is unchanged.

Verified: the case red then green, the bug verified, and the full suite 736 of 736. Next: Leif's review and merge.

Read more: [the story](../../.chough/archive/sessions/2026-11-13-fix-247-example-race.md) · [the archived change](../../.specsync/archive/changes/2026-11-13-fix-247-the-verify-lane-s-example-waits-past-the-failing-run-s-second-before-its-rerun/)
