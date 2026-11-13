---
kind: evidence
id: 2026-11-13-tc-cli-a-close-that-runs-no-native-step-says-closed-once
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:ff95e043fefda1049a426cf266c0574c8737d0043c179ee5aee2c9d9345044ea
criteria: [FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:19:00Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_closed_once for TC-cli-a-close-that-runs-no-native-step-says-closed-once: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:19:00Z. The raw log stays in the machine-local store.
