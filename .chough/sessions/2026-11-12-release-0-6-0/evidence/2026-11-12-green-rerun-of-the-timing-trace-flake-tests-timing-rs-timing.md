---
kind: evidence
id: 2026-11-12-green-rerun-of-the-timing-trace-flake-tests-timing-rs-timing
aliases: []
claim: "Green rerun of the timing trace flake: tests/timing.rs::timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit built with Rust 1.96.0 at b6fcd0e9 (with #172 and both warm-ups) passed 20 of 20 under load (20 busy loops and 8 loops that each write a new executable and run it once, the macOS first-exec assessment load), 2 lanes x 10 runs, inside the heavy-tests lock, with t37f's rerun.sh and stress.sh copied unchanged"
result: passed
evidence_kind: product-test
method: automated
revision: 2a45769806ab21e3c3c75a728d1702ab7ed30bbe
content: a8bbf1f6b64867bc1f17d59a3ce3286baedcbf2944611f9b461345b8af3b5e33
environment: local-macos-arm64
criteria: [DRIVE-19]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations: [Built and run at b6fcd0e9 in a throwaway worktree]
observed: 2026-11-12T16:54:12Z
actor:
  - agent: claude-code
    terminal: t382
---
**Green rerun of the timing trace flake: tests/timing.rs::timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit built with Rust 1.96.0 at b6fcd0e9 (with #172 and both warm-ups) passed 20 of 20 under load (20 busy loops and 8 loops that each write a new executable and run it once, the macOS first-exec assessment load), 2 lanes x 10 runs, inside the heavy-tests lock, with t37f's rerun.sh and stress.sh copied unchanged.**

Recorded as passed in local-macos-arm64 at revision 2a45769806ab, observed 2026-11-12T16:54:12Z.
