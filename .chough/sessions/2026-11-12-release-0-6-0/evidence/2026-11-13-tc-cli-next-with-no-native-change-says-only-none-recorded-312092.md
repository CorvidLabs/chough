---
kind: evidence
id: 2026-11-13-tc-cli-next-with-no-native-change-says-only-none-recorded-312092
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:43984abd3726d5ec8b42d15ab4f0e72c33ffe03d5daeaa134fbfa77870dfbcf1
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:10:44Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_with_no_native_change_says_only_that_none_is_recorded for TC-cli-next-with-no-native-change-says-only-none-recorded: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:10:44Z. The raw log stays in the machine-local store.
