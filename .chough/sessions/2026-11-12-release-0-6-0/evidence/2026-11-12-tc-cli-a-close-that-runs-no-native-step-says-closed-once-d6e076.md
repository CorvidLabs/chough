---
kind: evidence
id: 2026-11-12-tc-cli-a-close-that-runs-no-native-step-says-closed-once-d6e076
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
revision: 06407b98e650d73a951f505369cb7c694765ffde
content: 20c9710c0555251958526a8d2c4296aeac3f42743b807b3fecd9dd54614bf1da
environment: local-macos-arm64
fingerprint: sha256:4905022f2dc79d5b575d748f1abdc984fa51279ee677e346d619f422fd1edd16
criteria: [FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:19:55Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_closed_once for TC-cli-a-close-that-runs-no-native-step-says-closed-once: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 06407b98e650, observed 2026-11-12T21:19:55Z. The raw log stays in the machine-local store.
