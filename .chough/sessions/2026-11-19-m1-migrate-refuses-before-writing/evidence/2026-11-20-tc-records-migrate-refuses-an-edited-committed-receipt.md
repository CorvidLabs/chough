---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-an-edited-committed-receipt
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
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:4cbb794903418a255399d67d5cdcd4f7ac968200ba9b87cfb7543bd5457c08b7
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:23:47Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored for TC-records-migrate-refuses-an-edited-committed-receipt: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:23:47Z. The raw log stays in the machine-local store.
