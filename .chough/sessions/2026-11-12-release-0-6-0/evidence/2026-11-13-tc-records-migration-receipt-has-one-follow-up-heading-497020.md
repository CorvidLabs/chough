---
kind: evidence
id: 2026-11-13-tc-records-migration-receipt-has-one-follow-up-heading-497020
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_receipt_has_one_follow_up_heading_for_every_follow_up for TC-records-migration-receipt-has-one-follow-up-heading: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migration-receipt-has-one-follow-up-heading]
case_digest: [29c6ed5b0a8d0afca628cf4d0bbd9db2fcc386ebb634617c090e2a9b8e7e8096]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_receipt_has_one_follow_up_heading_for_every_follow_up
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:e870fa7711a8ab3fb279c67628435da6e3f175a6c25db7189058ff4168938ae6
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:10:10Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact a_receipt_has_one_follow_up_heading_for_every_follow_up for TC-records-migration-receipt-has-one-follow-up-heading: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:10:10Z. The raw log stays in the machine-local store.
