---
kind: evidence
id: 2026-11-12-red-rerun-of-the-stop-survivor-at-048a7c84-without-the-fix-7
aliases: []
claim: "Red rerun of the stop survivor at 048a7c84 (without the fix): 7 of 48 loaded runs failed (peak 1-minute loads 41.0 to 64.7), each with '<signal>: the command chough started survived' (HUP 3, INT 3, TERM 1) in stopping_a_gate_list_stops_git_and_records_nothing, stopping_a_native_read_stops_specsync_and_records_nothing and stopping_a_case_run_stops_its_command_and_records_nothing; t38b's flaky_stop.sh and lane_stress.py copied unchanged, under the heavy-tests lock: the stop test binary built with Rust 1.96.0 and run 8 times at once per round beside 30 CPU busy loops and 4 F_FULLFSYNC writers, 6 rounds (48 runs)"
result: failed
evidence_kind: product-test
method: automated
revision: f69889a9a33445be9e5a4e17566768172a12f543
content: 5d0e6fb343b4b356629b8380d7e902a652dad63249579a0d2283a7774d0d3053
environment: local-macos-arm64
criteria: [DRIVE-19]
requirements: [REQ-cli-011]
contracts: [specs/cli/cli.spec.md]
limitations:
  - Built and run at 048a7c84 in a throwaway worktree, not at this entry's recorded revision
observed: 2026-11-12T18:07:13Z
actor:
  - agent: claude-code
    terminal: t382
---
**Red rerun of the stop survivor at 048a7c84 (without the fix): 7 of 48 loaded runs failed (peak 1-minute loads 41.0 to 64.7), each with '<signal>: the command chough started survived' (HUP 3, INT 3, TERM 1) in stopping_a_gate_list_stops_git_and_records_nothing, stopping_a_native_read_stops_specsync_and_records_nothing and stopping_a_case_run_stops_its_command_and_records_nothing; t38b's flaky_stop.sh and lane_stress.py copied unchanged, under the heavy-tests lock: the stop test binary built with Rust 1.96.0 and run 8 times at once per round beside 30 CPU busy loops and 4 F_FULLFSYNC writers, 6 rounds (48 runs).**

Recorded as failed in local-macos-arm64 at revision f69889a9a334, observed 2026-11-12T18:07:13Z.

## Key failing lines

- HUP: the command chough started survived
