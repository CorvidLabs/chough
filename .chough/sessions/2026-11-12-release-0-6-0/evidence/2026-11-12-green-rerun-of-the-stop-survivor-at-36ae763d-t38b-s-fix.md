---
kind: evidence
id: 2026-11-12-green-rerun-of-the-stop-survivor-at-36ae763d-t38b-s-fix
aliases: []
claim: "Green rerun of the stop survivor at 36ae763d (t38b's fix 16afb63a, signalling the process group until no live member is left): 0 of 48 loaded runs failed (peak 1-minute loads 54.8 to 64.0); t38b's flaky_stop.sh and lane_stress.py copied unchanged, under the heavy-tests lock: the stop test binary built with Rust 1.96.0 and run 8 times at once per round beside 30 CPU busy loops and 4 F_FULLFSYNC writers, 6 rounds (48 runs)"
result: passed
evidence_kind: product-test
method: automated
revision: f69889a9a33445be9e5a4e17566768172a12f543
content: 5d0e6fb343b4b356629b8380d7e902a652dad63249579a0d2283a7774d0d3053
environment: local-macos-arm64
criteria: [DRIVE-19]
requirements: [REQ-cli-011]
contracts: [specs/cli/cli.spec.md]
limitations: [Built and run at 36ae763d in a throwaway worktree]
observed: 2026-11-12T18:07:16Z
actor:
  - agent: claude-code
    terminal: t382
---
**Green rerun of the stop survivor at 36ae763d (t38b's fix 16afb63a, signalling the process group until no live member is left): 0 of 48 loaded runs failed (peak 1-minute loads 54.8 to 64.0); t38b's flaky_stop.sh and lane_stress.py copied unchanged, under the heavy-tests lock: the stop test binary built with Rust 1.96.0 and run 8 times at once per round beside 30 CPU busy loops and 4 F_FULLFSYNC writers, 6 rounds (48 runs).**

Recorded as passed in local-macos-arm64 at revision f69889a9a334, observed 2026-11-12T18:07:16Z.
