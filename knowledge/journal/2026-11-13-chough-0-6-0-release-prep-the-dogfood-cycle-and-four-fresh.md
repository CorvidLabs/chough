---
id: 2026-11-13-chough-0-6-0-release-prep-the-dogfood-cycle-and-four-fresh
title: "Chough 0.6.0 release prep: the dogfood cycle and four fresh-eyes rounds"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-13
time: 2026-11-13T01:28:58Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-12-release-0-6-0.md
  - .specsync/archive/changes/2026-11-13-prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood/
record: .chough/archive/sessions/2026-11-12-release-0-6-0.md
actor:
  - agent: claude-code
    terminal: t382
---
# Chough 0.6.0 release prep: the dogfood cycle and four fresh-eyes rounds

Chough 0.6.0 was prepared as its own dogfood cycle, driven only with the 0.6.0 build: Cargo 0.6.0 and the dated CHANGELOG, the dogfood's fixes, journal posts with short titles and real summaries for every archived story, the first cross-project lessons, and the flaky-test bugs settled.

Four fresh-eyes rounds by new testers found 12, 16, 12 and then 4 real problems, each fixed with a failing test first, helpers t37f, t38b and t38c sharing the work; the fourth round's polish is deferred to 0.6.1 by Leif. Every gate decision names a configured human, no agent is routed to approve or review its own work, and a gate's next step never outlives the move it named.

Verified: the full suite passed 729 of 729 locally on Rust 1.96.0, with clippy and rustfmt clean and no open bug. Next: the 0.6.0 tag and release, which Leif decides.

Read more: [the story](../../.chough/archive/sessions/2026-11-12-release-0-6-0.md) · [the archived change](../../.specsync/archive/changes/2026-11-13-prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood/)
