---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-an-entry-that-is-no-receipt
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
duration_seconds: 2
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:14c6bd855dd868d460bd0cb43571e5bb7f649b92a7f9983a4797b6490eb9c336
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:23:55Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting for TC-records-migrate-refuses-an-entry-that-is-no-receipt: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:23:55Z. The raw log stays in the machine-local store.
