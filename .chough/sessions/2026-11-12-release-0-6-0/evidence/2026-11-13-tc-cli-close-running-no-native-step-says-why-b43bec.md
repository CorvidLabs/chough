---
kind: evidence
id: 2026-11-13-tc-cli-close-running-no-native-step-says-why-b43bec
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:a2f33539269cf5f68f5a6941f1f82fc19ac4ccd9d67f280cacb4564590bafabf
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:42:23Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_why for TC-cli-close-running-no-native-step-says-why: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:42:23Z. The raw log stays in the machine-local store.
