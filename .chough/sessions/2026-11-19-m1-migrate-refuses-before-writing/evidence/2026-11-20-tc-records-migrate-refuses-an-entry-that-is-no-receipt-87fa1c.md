---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-an-entry-that-is-no-receipt-87fa1c
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
duration_seconds: 1
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:71a0c6d20dd57c0f459da14caee5402af9e2ef2f2a7e175b5783ef4efdf4dbf6
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:35:57Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting for TC-records-migrate-refuses-an-entry-that-is-no-receipt: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:35:57Z. The raw log stays in the machine-local store.
