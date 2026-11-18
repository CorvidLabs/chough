---
kind: evidence
id: 2026-11-18-tc-records-a-receipt-name-taken-after-the-preview-refuses
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_receipt_name_taken_after_the_preview_refuses_before_any_write for TC-records-a-receipt-name-taken-after-the-preview-refuses-before-any-write: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-receipt-name-taken-after-the-preview-refuses-before-any-write]
case_digest: [ec61ec89e4c07de2c81200c24d17e28f5874ee281998a4e6abd7e51bf51688f1]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_receipt_name_taken_after_the_preview_refuses_before_any_write
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 47bfcd1bd75af6df73258d3c6bd20aba692d4441
content: 568cd36dc6884a0f83742cb710d7a9515705fa9e314f241dc64af0d95c089499
environment: local-macos-arm64
fingerprint: sha256:c9e9968d0862d5bfbdb45425e10d41611bf9ee063990a29ee403c145a4eef699
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:39:24Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_receipt_name_taken_after_the_preview_refuses_before_any_write for TC-records-a-receipt-name-taken-after-the-preview-refuses-before-any-write: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 47bfcd1bd75a, observed 2026-11-18T19:39:24Z. The raw log stays in the machine-local store.
