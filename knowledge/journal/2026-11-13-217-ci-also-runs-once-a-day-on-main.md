---
id: 2026-11-13-217-ci-also-runs-once-a-day-on-main
title: "#217: CI also runs once a day on main"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-13
time: 2026-11-13T14:51:30Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-13-daily-ci.md
  - .specsync/archive/changes/2026-11-13-217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs/
record: .chough/archive/sessions/2026-11-13-daily-ci.md
actor:
  - agent: claude-code
    terminal: t38r
---
# #217: CI also runs once a day on main

#217: CI also runs once a day on main, so the flake window sees a macOS and Linux run every day; the SpecSync Archive jobs take the run's own commit as their base when the event has none (REQ-cli-064)

Why: A daily scheduled CI run on main, macOS and Linux (Leif, Q3, 2026-11-13T03:30Z, relayed by t370; rejected: counting only real pushes)

Verified: The full test suite passed 738 of 738 across 48 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at 7b9c837f, the daily CI head with main (#242) merged and REQ-cli-064 materialized as cli version 90

Read more: [the story](../../.chough/archive/sessions/2026-11-13-daily-ci.md) · [the archived change](../../.specsync/archive/changes/2026-11-13-217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs/)
