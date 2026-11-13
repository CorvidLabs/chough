---
kind: evidence
id: 2026-11-13-tc-cli-next-with-no-native-change-says-only-none-recorded
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:0435e49ec8d07909bc1d7508343178b25596d3c84e46e15f24ccd36635e698f8
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:19:11Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_with_no_native_change_says_only_that_none_is_recorded for TC-cli-next-with-no-native-change-says-only-none-recorded: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:19:11Z. The raw log stays in the machine-local store.
