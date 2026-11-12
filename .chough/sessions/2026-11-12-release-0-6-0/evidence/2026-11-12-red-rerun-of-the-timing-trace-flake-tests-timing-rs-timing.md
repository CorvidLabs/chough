---
kind: evidence
id: 2026-11-12-red-rerun-of-the-timing-trace-flake-tests-timing-rs-timing
aliases: []
claim: "Red rerun of the timing trace flake: tests/timing.rs::timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit built with Rust 1.96.0 at 046e737c (before #172) in a throwaway worktree passed 0 of 20 under load (20 busy loops and 8 loops that each write a new executable and run it once, the macOS first-exec assessment load), 2 lanes x 10 runs, inside the heavy-tests lock, with t37f's rerun.sh and stress.sh copied unchanged"
result: failed
evidence_kind: product-test
method: automated
revision: 2a45769806ab21e3c3c75a728d1702ab7ed30bbe
content: a8bbf1f6b64867bc1f17d59a3ce3286baedcbf2944611f9b461345b8af3b5e33
environment: local-macos-arm64
criteria: [DRIVE-19]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations:
  - Built and run at 046e737c in a throwaway worktree, not at this entry's recorded revision
observed: 2026-11-12T16:54:10Z
actor:
  - agent: claude-code
    terminal: t382
---
**Red rerun of the timing trace flake: tests/timing.rs::timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit built with Rust 1.96.0 at 046e737c (before #172) in a throwaway worktree passed 0 of 20 under load (20 busy loops and 8 loops that each write a new executable and run it once, the macOS first-exec assessment load), 2 lanes x 10 runs, inside the heavy-tests lock, with t37f's rerun.sh and stress.sh copied unchanged.**

Recorded as failed in local-macos-arm64 at revision 2a45769806ab, observed 2026-11-12T16:54:10Z.
