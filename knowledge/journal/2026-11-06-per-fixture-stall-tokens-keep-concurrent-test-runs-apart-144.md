---
id: 2026-11-06-per-fixture-stall-tokens-keep-concurrent-test-runs-apart-144
title: Per-fixture stall tokens keep concurrent test runs apart (#144)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T14:17:41Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-stop-tests-stall-on-a-per-fixture-token-so-concurrent-runs.md
  - .specsync/archive/changes/2026-11-06-stop-tests-stall-on-a-per-fixture-token-so-concurrent-runs-never-take-or-kill-each-other-processes-144/
record: .chough/archive/sessions/2026-11-06-stop-tests-stall-on-a-per-fixture-token-so-concurrent-runs.md
actor:
  - agent: claude-code
    terminal: t382
---
# Per-fixture stall tokens keep concurrent test runs apart (#144)

The stop tests now stall on a per-fixture token, so concurrent runs never take or kill each other's processes (#144, DRIVE-10).

The test that a backfill stop leaves another run's processes alone failed first, then passed on Linux, and the stop suite passed 14 of 14.

The maintainer's review was relayed and recorded on 2026-11-06 under a standing instruction to approve and continue. The change was archived and the story closed done that day.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-stop-tests-stall-on-a-per-fixture-token-so-concurrent-runs.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-stop-tests-stall-on-a-per-fixture-token-so-concurrent-runs-never-take-or-kill-each-other-processes-144/)
