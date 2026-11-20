---
kind: evidence
id: 2026-11-20-tc-records-migrate-an-uncommitted-receipt-judges-nothing
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
duration_seconds: 2
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:046f7b5e06f99d5d9351acbd363d03a2cc2db854b150ca8694a269aa8135edea
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:23:51Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does for TC-records-migrate-an-uncommitted-receipt-judges-nothing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:23:51Z. The raw log stays in the machine-local store.
