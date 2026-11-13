---
kind: bug
id: 2026-11-13-flake-window-counts-a-day-covered-when-its-product-jobs-passed
aliases: []
title: flake window counts a day covered when its Product jobs passed only in different runs
status: open
classification: product-defect
scope: blocking
impact: The 1.0 flake window could read clean a day early; found before merge
criteria: [LIFE-21]
requirements: [REQ-cli-062]
cases: []
evidence: [2026-11-13-flake-window/2026-11-13-tc-cli-flake-window-restarts-ec7983]
receipts: []
environment: local-macos-arm64
revision: 3287c91f35c7d024b21e7ba423ab15e68d85b0c8
found_by: claude-code (hc t38r) (manual report)
actor:
  - agent: claude-code
    terminal: t38r
related: []
created: 2026-11-13T05:00:40Z
updated: 2026-11-13T05:01:40Z
---
# flake window counts a day covered when its Product jobs passed only in different runs

**What happened:** A UTC day whose only main runs are one with ubuntu green and macOS red and one with macOS green and ubuntu red records no no-green-run restart, so the window reads clean up to a day early
Actual: With 20 green days except 2026-11-17's two split runs and --now 2026-12-01T12:00:00Z the verdict was clean, latest restart the 08:00Z red run (t384's probe)

**Steps:** Trigger: compute() in .github/scripts/flake-window.py marks a day covered one OS at a time across all of that day's main runs (lines 306-313 at 3287c91f)
1. Feed --input 20 days of green main runs, except one day with only two runs that each fail a different Product job
2. Run flake-window.py with --now after that day plus 14 days

**Expected:** REQ-cli-062: a day counts only when one main run had both Product / ubuntu-latest and Product / macos-latest succeed; otherwise a no-green-run restart at the end of that day

**Why it matters:** The 1.0 flake window could read clean a day early; found before merge

**Fix:** Not fixed yet.

## Log

- 2026-11-13T05:00:40Z · Reported (manual report) · by claude-code (hc t38r)
- 2026-11-13T05:01:40Z · evidence: 2026-11-13-flake-window/2026-11-13-tc-cli-flake-window-restarts-ec7983 · by claude-code (hc t38r)
