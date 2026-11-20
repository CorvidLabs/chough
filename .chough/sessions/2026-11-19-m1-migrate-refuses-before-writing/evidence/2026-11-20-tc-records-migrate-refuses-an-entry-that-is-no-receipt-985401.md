---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-an-entry-that-is-no-receipt-985401
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting for TC-records-migrate-refuses-an-entry-that-is-no-receipt: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-refuses-an-entry-that-is-no-receipt]
case_digest: [0c9383d5653d9060bd4989b5a8834a33afbce388d238caee3b16dcc1ed87b2ef]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting
exit_code: 0
tests_run: 1
duration_seconds: 7
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:7b614277bc01f92e03e4c1b4a1aac1ffa1ef475beeafad216fe3932924469938
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:59:00Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting for TC-records-migrate-refuses-an-entry-that-is-no-receipt: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T03:59:00Z. The raw log stays in the machine-local store.
