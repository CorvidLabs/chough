---
kind: evidence
id: 2026-11-20-tc-records-migrate-removes-a-returning-file-as-already-0dec3d
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
duration_seconds: 1
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:a2b9fc694cbbfc83846ca07a28e7b66c223494bee50b0972717529df68f1589f
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:58:38Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated for TC-records-migrate-removes-a-returning-file-as-already-migrated: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T03:58:38Z. The raw log stays in the machine-local store.
