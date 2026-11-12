---
kind: evidence
id: 2026-11-12-tc-cli-next-with-no-native-change-says-only-none-recorded-30b630
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
duration_seconds: 1
revision: cca98758c52eb3f7b42d1f1caf751fc0e5a500c1
content: 94d89ecbe5a0f2d293255d074ccc5b065282faf8c8e453fc4e4ec5f4f093b88f
environment: local-macos-arm64
fingerprint: sha256:0aa6c6c0fcf336cb046e12fa340c5a16e8d479f3ed48e3f870f074b3070c0cbd
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:03:11Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_with_no_native_change_says_only_that_none_is_recorded for TC-cli-next-with-no-native-change-says-only-none-recorded: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cca98758c52e, observed 2026-11-12T21:03:11Z. The raw log stays in the machine-local store.
