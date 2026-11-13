---
kind: evidence
id: 2026-11-13-tc-cli-next-with-no-native-change-says-only-none-recorded-304af9
aliases: []
claim: "Ran cargo test --locked --test next -- --exact next_with_no_native_change_says_only_that_none_is_recorded for TC-cli-next-with-no-native-change-says-only-none-recorded: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-with-no-native-change-says-only-none-recorded]
case_digest: [ff9ea039288d96b4c88d8ecbe32f2d0826b193e7e17585e2056f78a79036fb4e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - next_with_no_native_change_says_only_that_none_is_recorded
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:404a8c7d8477a09254ac5ce3d11a019938bce148ee5ca41be0464a4636b632d4
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:44:37Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_with_no_native_change_says_only_that_none_is_recorded for TC-cli-next-with-no-native-change-says-only-none-recorded: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:44:37Z. The raw log stays in the machine-local store.
