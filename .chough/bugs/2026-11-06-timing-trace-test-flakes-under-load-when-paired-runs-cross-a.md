---
kind: bug
id: 2026-11-06-timing-trace-test-flakes-under-load-when-paired-runs-cross-a
aliases:
  - BUG-timing-trace-test-flakes-under-load-when-paired-runs-cross-a-second-281cc64e4a699ad097f7b97b
title: timing trace test flakes under load when paired runs cross a second
status: verified
classification: evidence-gap
scope: in-scope
impact: "A spurious full-suite failure on a loaded machine; unrelated to the change under test (ch-d7, #123)"
criteria: []
requirements: []
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-red-rerun-of-the-timing-trace-flake-tests-timing-rs-timing
  - 2026-11-12-release-0-6-0/2026-11-12-green-rerun-of-the-timing-trace-flake-tests-timing-rs-timing
receipts: []
environment: local-macos-arm64
revision: 62854369d4408b83750c1f6b4785fa71c55c168e
found_by: ch-d7 full suite, 2026-11-05
related: []
created: 2026-11-06T03:55:29Z
updated: 2026-11-13T04:33:19Z
---
# timing trace test flakes under load when paired runs cross a second

**What happened:** tests/timing.rs timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit panics with 'traced and untraced reports differ'
Actual: All 5 attempts produced reports that differ, because an ongoing segment ends at now and each pair crossed a second boundary; rerun alone it failed 1 of 3 and passed 2 of 3

**Steps:** cargo test --no-fail-fast while other heavy work (a Docker cargo build and the history tests) loads the machine
1. Load the machine (for example a Docker cargo build)
2. cargo test --test timing timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit

**Expected:** The traced and untraced timing reports match, as they do when each pair of runs finishes within one second

**Why it matters:** A spurious full-suite failure on a loaded machine; unrelated to the change under test (ch-d7, #123)

**Fix:** Not fixed yet.

## Log

- 2026-11-06T03:55:29Z · Reported (ch-d7 full suite, 2026-11-05)
- 2026-11-12T16:54:35Z · status: open -> verified; scope: out-of-scope -> in-scope; evidence: 2026-11-12-release-0-6-0/2026-11-12-red-rerun-of-the-timing-trace-flake-tests-timing-rs-timing, 2026-11-12-release-0-6-0/2026-11-12-green-rerun-of-the-timing-trace-flake-tests-timing-rs-timing. Fixed by #172 (b8088923) before this release; t37f found the as-filed test (69caa0e7) does not flake even at 60 busy loops, since it needs 5 straddles in a row, and checked the 2x edge is absent from the compared JSON. Rerun by t382 with t37f's scripts: #169's red at 046e737c 0/20 passed, green at b6fcd0e9 20/20 · by claude-code (hc t382)
- 2026-11-13T04:33:19Z · Tracked by the flake window (DRIVE-19); GitHub #169 · by claude-code (hc t38r)
