---
id: 2026-11-06-fast-warm-next-and-status-in-archive-heavy-repos-32
title: Fast warm next and status in archive-heavy repos (#32)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T23:55:14Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-issue-32-next-status-latency.md
  - .specsync/archive/changes/2026-11-06-make-archived-next-and-status-fast-within-explicit-warm-budgets/
record: .chough/archive/sessions/2026-11-06-issue-32-next-status-latency.md
actor:
  - agent: claude-code
    terminal: t382
---
# Fast warm next and status in archive-heavy repos (#32)

Archived native status alone took 4.36 to 4.84 seconds, so next and status got warm budgets of 2 s p95 and 3 s maximum over ten release runs (REQ-cli-053, DRIVE-15). The fix is a read-only, authenticated cache of archived reports with exact snapshot workers. Lifecycle reads and mutations stay live against SpecSync, and a timed-out prime writes nothing and leaves no child processes.

Failing tests came first and two local bugs were verified. All five focused cases passed, and full Fledge verify passed 10 of 10 before the last merge of main. The final release benchmark on 147 archives measured next at 0.66 s and status at 0.67 s, p95 and maximum, with zero warm SpecSync calls.

The maintainer approved the scope and a correction adding two spec files to it, then passed the review of PR #180. The story closed as done on 6 November 2026.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-issue-32-next-status-latency.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-make-archived-next-and-status-fast-within-explicit-warm-budgets/)
