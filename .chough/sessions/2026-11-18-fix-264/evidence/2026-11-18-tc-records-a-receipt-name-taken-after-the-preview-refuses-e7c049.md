---
kind: evidence
id: 2026-11-18-tc-records-a-receipt-name-taken-after-the-preview-refuses-e7c049
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
revision: 75865bab698e97524d4f36fd67c17a15ea4803a1
content: 28e558987941084f772ef8b48c5d465d3fdc025c02bb36ac8194e42236a13abb
environment: local-macos-arm64
fingerprint: sha256:dab720ce419434b2cd9fe7956db19d48e0631f3b8b45811f3beb5b88bd9f614c
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:55:35Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_receipt_name_taken_after_the_preview_refuses_before_any_write for TC-records-a-receipt-name-taken-after-the-preview-refuses-before-any-write: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 75865bab698e, observed 2026-11-18T19:55:35Z. The raw log stays in the machine-local store.
