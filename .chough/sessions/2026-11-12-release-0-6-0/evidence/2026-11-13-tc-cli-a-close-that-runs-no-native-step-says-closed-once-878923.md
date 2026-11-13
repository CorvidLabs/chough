---
kind: evidence
id: 2026-11-13-tc-cli-a-close-that-runs-no-native-step-says-closed-once-878923
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:a43382e061c6706d4ad034c5cd06763b193dacc177b4ee3a22e152023d96db5d
criteria: [FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:10:26Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_closed_once for TC-cli-a-close-that-runs-no-native-step-says-closed-once: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:10:26Z. The raw log stays in the machine-local store.
