---
kind: evidence
id: 2026-11-20-tc-records-migrate-an-uncommitted-receipt-judges-nothing-d83a22
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does for TC-records-migrate-an-uncommitted-receipt-judges-nothing: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-an-uncommitted-receipt-judges-nothing]
case_digest: [b0e42920b60fac10644ef01752dd88c198e7f0b3a4c97573934f7846643694ca]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - an_uncommitted_receipt_judges_nothing_and_a_committed_one_does
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:13de8893b991a393dac32581ec68ac753b81dc6dc6917fcc79b0afc9dda6192d
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:35:54Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does for TC-records-migrate-an-uncommitted-receipt-judges-nothing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:35:54Z. The raw log stays in the machine-local store.
