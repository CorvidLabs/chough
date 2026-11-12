---
id: 2026-11-06-timing-trace-test-fails-only-when-tracing-changes-the-report
title: Timing trace test fails only when tracing changes the report (#169)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T21:17:34Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-make-the-timing-trace-test-fail-only-when-tracing-changes-the.md
  - .specsync/archive/changes/2026-11-06-make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive/
record: .chough/archive/sessions/2026-11-06-make-the-timing-trace-test-fail-only-when-tracing-changes-the.md
actor:
  - agent: claude-code
    terminal: t382
---
# Timing trace test fails only when tracing changes the report (#169)

The timing trace test should fail only when tracing changes the report, never because its paired runs straddle a second (chough#169). The maintainer captured this as DRIVE-19.

Rather than add a runtime clock switch (issue #30), the fix replaces the test's retry loop with an exact clock-shift comparison. The story closed done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-make-the-timing-trace-test-fail-only-when-tracing-changes-the.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive/)
