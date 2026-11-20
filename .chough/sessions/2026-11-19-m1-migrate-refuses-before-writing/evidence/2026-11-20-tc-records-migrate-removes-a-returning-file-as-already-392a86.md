---
kind: evidence
id: 2026-11-20-tc-records-migrate-removes-a-returning-file-as-already-392a86
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
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:8b04481c2924f97a205e1c5447ed1a7c9647b2447d8077e4f5e067e7607f0029
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:35:47Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated for TC-records-migrate-removes-a-returning-file-as-already-migrated: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:35:47Z. The raw log stays in the machine-local store.
