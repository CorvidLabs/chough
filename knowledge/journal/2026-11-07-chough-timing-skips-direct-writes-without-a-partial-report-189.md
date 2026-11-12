---
id: 2026-11-07-chough-timing-skips-direct-writes-without-a-partial-report-189
title: chough timing skips direct writes without a partial report (#189)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-07
time: 2026-11-07T04:10:52Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-07-chough-timing-skips-direct-native-write-events-without-marking.md
  - .specsync/archive/changes/2026-11-07-chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21/
record: .chough/archive/sessions/2026-11-07-chough-timing-skips-direct-native-write-events-without-marking.md
actor:
  - agent: claude-code
    terminal: t382
---
# chough timing skips direct writes without a partial report (#189)

Issue #189 was a regression from #183: chough timing counted the valid direct-native-write event kind as unrecognised and marked its report partial. It now treats that kind as a known non-segment kind, and REQ-history-013 says so (DRIVE-21). At the coordinating agent's request, chough change new also gained --no-spec-change and --rationale (REQ-cli-057).

Both cases passed and were refreshed on the merged PR head, where both passed again: timing skips direct-write events without marking the report partial, and change new passes --no-spec-change with its rationale.

The maintainer approved the definition and the implementation review. The change was archived and the story closed as done on 2026-11-07.

Read more: [the story](../../.chough/archive/sessions/2026-11-07-chough-timing-skips-direct-native-write-events-without-marking.md) · [the archived change](../../.specsync/archive/changes/2026-11-07-chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21/)
