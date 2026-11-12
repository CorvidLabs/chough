---
kind: evidence
id: 2026-11-12-tc-cli-close-running-no-native-step-says-why-012a8a
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
revision: c53da06cc673269174e8c292895f977d091bc88b
content: 03df8cca3a1a59d6c22f7023c528bb2e162676aef0e142cb83ca1221bdb1f179
environment: local-macos-arm64
fingerprint: sha256:9fee002bca3c8a5a8dc67fe3b4e711ea5d18b49743428750cdada68fb4849cdc
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:09:36Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_why for TC-cli-close-running-no-native-step-says-why: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c53da06cc673, observed 2026-11-12T20:09:36Z. The raw log stays in the machine-local store.
