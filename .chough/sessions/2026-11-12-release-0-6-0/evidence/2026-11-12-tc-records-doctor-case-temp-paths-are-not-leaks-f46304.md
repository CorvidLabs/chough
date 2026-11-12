---
kind: evidence
id: 2026-11-12-tc-records-doctor-case-temp-paths-are-not-leaks-f46304
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact a_temporary_path_in_a_case_definition_is_not_a_leak for TC-records-doctor-case-temp-paths-are-not-leaks: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-doctor-case-temp-paths-are-not-leaks]
case_digest: [370f3cf8905f4572b0e450a0a4c8bf13736e848adc9cc6df334808b927bde0fc]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - a_temporary_path_in_a_case_definition_is_not_a_leak
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 69b60a68d925da1c3a5654890bd62627599666bd
content: 18d24a04ea352eed30462b2a6d05214a9659b6d958ad33db5ee89f1b546be238
environment: local-macos-arm64
fingerprint: sha256:60eb7de980d3555030d09dc545fe030d5820e6661364e41caf75d305396a6d1f
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:19:53Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test doctor -- --exact a_temporary_path_in_a_case_definition_is_not_a_leak for TC-records-doctor-case-temp-paths-are-not-leaks: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 69b60a68d925, observed 2026-11-12T16:19:53Z. The raw log stays in the machine-local store.
