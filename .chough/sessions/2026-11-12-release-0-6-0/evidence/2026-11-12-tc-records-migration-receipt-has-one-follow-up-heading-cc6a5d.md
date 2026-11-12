---
kind: evidence
id: 2026-11-12-tc-records-migration-receipt-has-one-follow-up-heading-cc6a5d
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
revision: 06407b98e650d73a951f505369cb7c694765ffde
content: 20c9710c0555251958526a8d2c4296aeac3f42743b807b3fecd9dd54614bf1da
environment: local-macos-arm64
fingerprint: sha256:a5524e521d009dc397d98dc435a85d528a0f31d7bab28d39a5d21499b04f9c06
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:19:50Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact a_receipt_has_one_follow_up_heading_for_every_follow_up for TC-records-migration-receipt-has-one-follow-up-heading: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 06407b98e650, observed 2026-11-12T21:19:50Z. The raw log stays in the machine-local store.
