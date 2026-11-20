---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-an-edited-committed-receipt-f27657
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored for TC-records-migrate-refuses-an-edited-committed-receipt: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-refuses-an-edited-committed-receipt]
case_digest: [365b8b4c3b4cd60cec0c86242516373eedbe64be6894863ce037fa128aa1e103]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - an_edited_committed_receipt_is_refused_until_committed_or_restored
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:89edc8dbe630341866e4a6a395cc01b7b138ed9de9700782c7278fe2e6ced72f
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:35:50Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored for TC-records-migrate-refuses-an-edited-committed-receipt: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:35:50Z. The raw log stays in the machine-local store.
