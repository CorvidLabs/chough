---
kind: evidence
id: 2026-11-12-tc-cli-a-close-that-runs-no-native-step-says-closed-once-e1bc50
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_closed_once for TC-cli-a-close-that-runs-no-native-step-says-closed-once: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-close-that-runs-no-native-step-says-closed-once]
case_digest: [20bad69c45c115e0fe183b0deaaf610064a8c89df019b15543e396d911056e38]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - a_close_that_runs_no_native_step_says_closed_once
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: cca98758c52eb3f7b42d1f1caf751fc0e5a500c1
content: 94d89ecbe5a0f2d293255d074ccc5b065282faf8c8e453fc4e4ec5f4f093b88f
environment: local-macos-arm64
fingerprint: sha256:68689124d4e298fdb70345f41cd45c60e5891a99d4f495000166af05938cb9e2
criteria: [FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:03:06Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_closed_once for TC-cli-a-close-that-runs-no-native-step-says-closed-once: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cca98758c52e, observed 2026-11-12T21:03:06Z. The raw log stays in the machine-local store.
