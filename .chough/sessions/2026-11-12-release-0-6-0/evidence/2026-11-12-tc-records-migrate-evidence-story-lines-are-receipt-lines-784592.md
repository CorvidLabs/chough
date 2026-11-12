---
kind: evidence
id: 2026-11-12-tc-records-migrate-evidence-story-lines-are-receipt-lines-784592
aliases: []
claim: "Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_storys_verified_lines_are_receipt_lines for TC-records-migrate-evidence-story-lines-are-receipt-lines: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-evidence-story-lines-are-receipt-lines]
case_digest: [b8b2042bb991bd9e9975b8d72dfb12c50c4cdb7c9c2ebbb8f0d4d45809a617ff]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate_evidence
  - "--"
  - "--exact"
  - a_migrated_storys_verified_lines_are_receipt_lines
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 421a7034dfce0c052d48c4d1fb5abe041c272a46
content: 9e16bb2234c3aadbf27905868186d8ca01d209615750221de4f1f7201e15af54
environment: local-macos-arm64
fingerprint: sha256:e30d59a32e093eeb7f762fba080324e8c9d1ced99f4e07be3b3177eade7dde5b
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:06:17Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_storys_verified_lines_are_receipt_lines for TC-records-migrate-evidence-story-lines-are-receipt-lines: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 421a7034dfce, observed 2026-11-12T23:06:17Z. The raw log stays in the machine-local store.
