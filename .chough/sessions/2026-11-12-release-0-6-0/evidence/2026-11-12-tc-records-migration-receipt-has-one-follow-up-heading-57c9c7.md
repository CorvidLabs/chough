---
kind: evidence
id: 2026-11-12-tc-records-migration-receipt-has-one-follow-up-heading-57c9c7
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:092def289c9dbd711401c2db727625cd43b812b84653f5146c16c39e812b8338
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:22:25Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact a_receipt_has_one_follow_up_heading_for_every_follow_up for TC-records-migration-receipt-has-one-follow-up-heading: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:22:25Z. The raw log stays in the machine-local store.
