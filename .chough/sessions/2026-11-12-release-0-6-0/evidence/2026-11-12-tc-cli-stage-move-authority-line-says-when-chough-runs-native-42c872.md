---
kind: evidence
id: 2026-11-12-tc-cli-stage-move-authority-line-says-when-chough-runs-native-42c872
aliases: []
claim: "Ran cargo test --locked --test stages -- --exact every_stage_move_says_when_chough_runs_native_approval_or_review for TC-cli-stage-move-authority-line-says-when-chough-runs-native-steps: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-stage-move-authority-line-says-when-chough-runs-native-steps]
case_digest: [5096a003c3676001dd2b46920e1d507c97db01eac7c1547a58cc0495d60cf30e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - stages
  - "--"
  - "--exact"
  - every_stage_move_says_when_chough_runs_native_approval_or_review
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e8bde99c60a7555434694e8ffadf1bb28fa25fda
content: 3ee3e75bcaf0a92f7bc5ca41f59bd421498d8a26342f52e2d390ef2dd4377b54
environment: local-macos-arm64
fingerprint: sha256:3df9df4386f306a3909ab794e7d3b4863a171215de6090409224d4d107d9dc61
criteria: [LIFE-3]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:39:34Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test stages -- --exact every_stage_move_says_when_chough_runs_native_approval_or_review for TC-cli-stage-move-authority-line-says-when-chough-runs-native-steps: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e8bde99c60a7, observed 2026-11-12T17:39:34Z. The raw log stays in the machine-local store.
