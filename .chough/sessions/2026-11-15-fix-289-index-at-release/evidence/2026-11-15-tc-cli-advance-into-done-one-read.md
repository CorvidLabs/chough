---
kind: evidence
id: 2026-11-15-tc-cli-advance-into-done-one-read
aliases: []
claim: "Ran cargo test --locked --test advance_to -- --exact advance_into_done_reads_native_status_once for TC-cli-advance-into-done-one-read: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-advance-into-done-one-read]
case_digest: [22cae0f9f046256865ba5ff8c24d78c1362420b7b22d9420c6d462c44c4530d3]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - advance_to
  - "--"
  - "--exact"
  - advance_into_done_reads_native_status_once
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:03942a8771d682838269c613b74eeaa890a5769a06532181cfbb9983db9ff81c
criteria: [DRIVE-2, DRIVE-9]
requirements: [REQ-cli-052]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:09Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test advance_to -- --exact advance_into_done_reads_native_status_once for TC-cli-advance-into-done-one-read: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:09Z. The raw log stays in the machine-local store.
