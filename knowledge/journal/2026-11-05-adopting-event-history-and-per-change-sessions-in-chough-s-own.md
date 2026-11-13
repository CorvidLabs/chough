---
id: 2026-11-05-adopting-event-history-and-per-change-sessions-in-chough-s-own
title: Adopting event history and per-change sessions in Chough's own repo
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-05
time: 2026-11-05T15:17:03Z
kind: shipped
links:
  - .chough/archive/sessions/2026-10-31-adopt-event-history-and-per-change-sessions-in-this-repository.md
  - .specsync/archive/changes/2026-11-05-adopt-event-history-and-per-change-sessions-in-this-repository-and-union-merge-the-retention-ledger-and-changelog/
record: .chough/archive/sessions/2026-10-31-adopt-event-history-and-per-change-sessions-in-this-repository.md
actor:
  - agent: claude-code
    terminal: t382
---
# Adopting event history and per-change sessions in Chough's own repo

A Chough session from 31 October to 5 November 2026 ran many cycles: the resumable stage workflow, transition history, faster native reads, human-gate guards and a knowledge vault. Its last change adopted event history and per-change sessions in this repository and union-merged the retention ledger and changelog, so parallel PRs stop conflicting.

Its simulation found that a new session after adopting a legacy transition log had no legacy anchor, so its first transition was refused. A bug-fix change fixed that first; case TC-history-session-after-legacy-adoption passed, then a parallel PR simulation passed on Linux.

Earlier, verify passed all 9 steps with 253 tests; Linux verify for the #109 fix passed 438, none failed.

The maintainer accepted both changes on PR #111, to merge on green CI. Both reviews were recorded, the adoption was archived and it closed as done on 5 November.

Read more: [the story](../../.chough/archive/sessions/2026-10-31-adopt-event-history-and-per-change-sessions-in-this-repository.md) · [the archived change](../../.specsync/archive/changes/2026-11-05-adopt-event-history-and-per-change-sessions-in-this-repository-and-union-merge-the-retention-ledger-and-changelog/)
