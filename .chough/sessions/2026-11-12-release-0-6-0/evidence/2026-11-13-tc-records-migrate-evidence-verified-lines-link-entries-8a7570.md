---
kind: evidence
id: 2026-11-13-tc-records-migrate-evidence-verified-lines-link-entries-8a7570
aliases: []
claim: "Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_sessions_verified_lines_link_the_carried_entries for TC-records-migrate-evidence-verified-lines-link-entries: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-evidence-verified-lines-link-entries]
case_digest: [315413e3ec932f7b304b5bd66a15e95c81885c0378e9970f506ddb0617779436]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate_evidence
  - "--"
  - "--exact"
  - a_migrated_sessions_verified_lines_link_the_carried_entries
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:18eea4193b26e5891db9df9ebd3855ff9a6d4184399961864d982912fb7949a2
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:52:05Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_sessions_verified_lines_link_the_carried_entries for TC-records-migrate-evidence-verified-lines-link-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:52:05Z. The raw log stays in the machine-local store.
