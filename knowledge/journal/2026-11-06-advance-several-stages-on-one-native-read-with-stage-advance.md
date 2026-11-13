---
id: 2026-11-06-advance-several-stages-on-one-native-read-with-stage-advance
title: Advance several stages on one native read with stage advance --to
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T06:32:03Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-advance-through-several-stages-on-one-native-read-with-stage.md
  - .specsync/archive/changes/2026-11-06-advance-through-several-stages-on-one-native-read-with-stage-advance-to-and-read-native-status-once-on-the-move-into/
record: .chough/archive/sessions/2026-11-06-advance-through-several-stages-on-one-native-read-with-stage.md
actor:
  - agent: claude-code
    terminal: t382
---
# Advance several stages on one native read with stage advance --to

This is one half of splitting PR #69. stage advance gains --to, so several stages advance on a single native status read (DRIVE-9), and the move into done reuses the gate's read, so it takes one read instead of two (DRIVE-2). It is recorded as REQ-cli-052. The persistent archive-verdict cache from #69 was left out and moved to a separate change, so every native read here stays live.

Six cases were written first: the five new-behaviour cases failed as expected and the unchanged plain advance passed. After the implementation and a merge of main, all six passed on macOS and verify passed through the spec check.

The implementation review was recorded as a pass under the maintainer's overnight delegation, and the change was archived. The story closed as done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-advance-through-several-stages-on-one-native-read-with-stage.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-advance-through-several-stages-on-one-native-read-with-stage-advance-to-and-read-native-status-once-on-the-move-into/)
