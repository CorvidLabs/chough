---
kind: evidence
id: 2026-11-12-red-rerun-of-the-timing-native-budget-flake-tests-timing-rs-a
aliases: []
claim: "Red rerun of the timing native-budget flake: tests/timing.rs::a_timed_out_or_unconfigured_native_read_leaves_native_times_unknown built with Rust 1.88.0 at 9c41e959 (the bug's revision) in a throwaway worktree passed 3 of 20 and failed 17 under load (20 busy loops and 8 loops that each write a new executable and run it once, the macOS first-exec assessment load), 2 lanes x 10 runs, inside the heavy-tests lock, with t37f's rerun.sh and stress.sh copied unchanged"
result: failed
evidence_kind: product-test
method: automated
revision: 2a45769806ab21e3c3c75a728d1702ab7ed30bbe
content: a8bbf1f6b64867bc1f17d59a3ce3286baedcbf2944611f9b461345b8af3b5e33
environment: local-macos-arm64
criteria: [LIFE-10]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations:
  - Built and run at 9c41e959 in a throwaway worktree, not at this entry's recorded revision
observed: 2026-11-12T16:54:22Z
actor:
  - agent: claude-code
    terminal: t382
---
**Red rerun of the timing native-budget flake: tests/timing.rs::a_timed_out_or_unconfigured_native_read_leaves_native_times_unknown built with Rust 1.88.0 at 9c41e959 (the bug's revision) in a throwaway worktree passed 3 of 20 and failed 17 under load (20 busy loops and 8 loops that each write a new executable and run it once, the macOS first-exec assessment load), 2 lanes x 10 runs, inside the heavy-tests lock, with t37f's rerun.sh and stress.sh copied unchanged.**

Recorded as failed in local-macos-arm64 at revision 2a45769806ab, observed 2026-11-12T16:54:22Z.
