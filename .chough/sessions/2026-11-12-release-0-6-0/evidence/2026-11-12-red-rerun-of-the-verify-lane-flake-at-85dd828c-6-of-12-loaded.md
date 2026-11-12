---
kind: evidence
id: 2026-11-12-red-rerun-of-the-verify-lane-flake-at-85dd828c-6-of-12-loaded
aliases: []
claim: "Red rerun of the verify-lane flake at 85dd828c: 6 of 12 loaded runs failed (peak 1-minute load 80.0): cases 3 of 3 (a_descendant_holding_the_pipes_cannot_outlast_the_case_deadline and a_descendant_holding_the_pipes_after_exit_is_stopped_by_the_deadline) and stop 3 of 3 (stopping_a_case_run_while_it_saves_stops_git_and_records_nothing); t38b's flaky_lane.sh and lane_stress.py copied unchanged, under the heavy-tests lock: the cli, change, stop and cases test binaries built with Rust 1.96.0, each run 3 times at once (12 runs) beside 30 CPU busy loops and 4 writers of 32 MiB with F_FULLFSYNC"
result: failed
evidence_kind: product-test
method: automated
revision: b0d0b5d75a8f4b666dde9f437f593143c3614b90
content: 5eeff7f6b75cd076203c7323f888502e085c40b771c64919a8978ab15a8db736
environment: local-macos-arm64
criteria: [DRIVE-19]
requirements: [REQ-cli-011]
contracts: [specs/cli/cli.spec.md]
limitations:
  - Built and run at 85dd828c in a throwaway worktree, not at this entry's recorded revision
observed: 2026-11-12T17:09:38Z
actor:
  - agent: claude-code
    terminal: t382
---
**Red rerun of the verify-lane flake at 85dd828c: 6 of 12 loaded runs failed (peak 1-minute load 80.0): cases 3 of 3 (a_descendant_holding_the_pipes_cannot_outlast_the_case_deadline and a_descendant_holding_the_pipes_after_exit_is_stopped_by_the_deadline) and stop 3 of 3 (stopping_a_case_run_while_it_saves_stops_git_and_records_nothing); t38b's flaky_lane.sh and lane_stress.py copied unchanged, under the heavy-tests lock: the cli, change, stop and cases test binaries built with Rust 1.96.0, each run 3 times at once (12 runs) beside 30 CPU busy loops and 4 writers of 32 MiB with F_FULLFSYNC.**

Recorded as failed in local-macos-arm64 at revision b0d0b5d75a8f, observed 2026-11-12T17:09:38Z.
