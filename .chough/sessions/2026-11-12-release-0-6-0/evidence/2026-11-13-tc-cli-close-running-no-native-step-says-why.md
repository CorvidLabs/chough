---
kind: evidence
id: 2026-11-13-tc-cli-close-running-no-native-step-says-why
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_why for TC-cli-close-running-no-native-step-says-why: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-close-running-no-native-step-says-why]
case_digest: [f562db109f888aef651e02e3a46dadf0f472f59740bf8326177e02bb0661701c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - a_close_that_runs_no_native_step_says_why
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:26b62a661baab64e4896ae987f557475e35168ea8be592cf29d8ead970ff2444
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:17:23Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_why for TC-cli-close-running-no-native-step-says-why: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:17:23Z. The raw log stays in the machine-local store.
