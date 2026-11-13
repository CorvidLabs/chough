---
id: 2026-11-05-split-the-archive-history-walk-across-the-ci-shards-105
title: Split the archive history walk across the CI shards (#105)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-05
time: 2026-11-05T22:29:25Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-05-spread-the-specsync-archive-history-walk-across-the-ci-shards.md
  - .specsync/archive/changes/2026-11-05-spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105/
record: .chough/archive/sessions/2026-11-05-spread-the-specsync-archive-history-walk-across-the-ci-shards.md
actor:
  - agent: claude-code
    terminal: t382
---
# Split the archive history walk across the CI shards (#105)

The SpecSync Archive job's history walk was spread across the four CI shards so a long pull request fits within the unchanged ten-minute limit (#105).

The archive fixtures passed on Linux: 25 tests ran OK, including the three new #105 tests that failed first. On the long #99 candidate the slowest shard measured 88.8 s, against 268.6 s.

The maintainer accepted PR #118, recorded a passing review and chose to record and merge, and also asked for a draft PR to prove it on a hosted run of the long #99 candidate. The change was archived and the story closed done on 2026-11-05.

Read more: [the story](../../.chough/archive/sessions/2026-11-05-spread-the-specsync-archive-history-walk-across-the-ci-shards.md) · [the archived change](../../.specsync/archive/changes/2026-11-05-spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105/)
