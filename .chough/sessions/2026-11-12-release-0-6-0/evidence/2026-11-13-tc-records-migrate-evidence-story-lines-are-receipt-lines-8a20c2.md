---
kind: evidence
id: 2026-11-13-tc-records-migrate-evidence-story-lines-are-receipt-lines-8a20c2
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:741adc624bf63b9f805e6c2950d1ce90e012f85b519d7ac5921b7d88cf077f7b
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:52:20Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_storys_verified_lines_are_receipt_lines for TC-records-migrate-evidence-story-lines-are-receipt-lines: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:52:20Z. The raw log stays in the machine-local store.
