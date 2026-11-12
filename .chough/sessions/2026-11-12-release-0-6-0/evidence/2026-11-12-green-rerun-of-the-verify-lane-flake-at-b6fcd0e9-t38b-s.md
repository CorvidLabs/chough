---
kind: evidence
id: 2026-11-12-green-rerun-of-the-verify-lane-flake-at-b6fcd0e9-t38b-s
aliases: []
claim: "Green rerun of the verify-lane flake at b6fcd0e9 (t38b's install-time stand-in warm-up 1888f4f5 and stub PIDs published by rename 2ee84412): 0 of 12 loaded runs failed (peak 1-minute load 93.9); t38b's flaky_lane.sh and lane_stress.py copied unchanged, under the heavy-tests lock: the cli, change, stop and cases test binaries built with Rust 1.96.0, each run 3 times at once (12 runs) beside 30 CPU busy loops and 4 writers of 32 MiB with F_FULLFSYNC"
result: passed
evidence_kind: product-test
method: automated
revision: b0d0b5d75a8f4b666dde9f437f593143c3614b90
content: 5eeff7f6b75cd076203c7323f888502e085c40b771c64919a8978ab15a8db736
environment: local-macos-arm64
criteria: [DRIVE-19]
requirements: [REQ-cli-011]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "Built and run at b6fcd0e9 in a throwaway worktree; one run cannot rule out a rarer flake, and t38b saw a different one once (HUP: the command chough started survived)"
observed: 2026-11-12T17:09:40Z
actor:
  - agent: claude-code
    terminal: t382
---
**Green rerun of the verify-lane flake at b6fcd0e9 (t38b's install-time stand-in warm-up 1888f4f5 and stub PIDs published by rename 2ee84412): 0 of 12 loaded runs failed (peak 1-minute load 93.9); t38b's flaky_lane.sh and lane_stress.py copied unchanged, under the heavy-tests lock: the cli, change, stop and cases test binaries built with Rust 1.96.0, each run 3 times at once (12 runs) beside 30 CPU busy loops and 4 writers of 32 MiB with F_FULLFSYNC.**

Recorded as passed in local-macos-arm64 at revision b0d0b5d75a8f, observed 2026-11-12T17:09:40Z.
