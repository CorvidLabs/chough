---
kind: evidence
id: 2026-11-13-tc-records-migrate-evidence-story-lines-are-receipt-lines-520f97
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:d347e44f3c3b1d403aedcdb27975c99d1a9a5c790d4facf61ef0ede28b45b0e6
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:20:15Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_storys_verified_lines_are_receipt_lines for TC-records-migrate-evidence-story-lines-are-receipt-lines: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:20:15Z. The raw log stays in the machine-local store.
