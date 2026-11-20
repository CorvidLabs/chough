---
kind: evidence
id: 2026-11-20-tc-records-migrate-removes-a-returning-file-as-already
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated for TC-records-migrate-removes-a-returning-file-as-already-migrated: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-removes-a-returning-file-as-already-migrated]
case_digest: [1dfb5e9be9cf0d8e6c2558dbdea62c5f17e77536335642d5e1ee559848fd4e39]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:933ad8c4199f41562eb3cc60872dad27ec37f7d7055619dc4d9709e2b6c9f067
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:23:44Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated for TC-records-migrate-removes-a-returning-file-as-already-migrated: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:23:44Z. The raw log stays in the machine-local store.
