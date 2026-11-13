---
kind: evidence
id: 2026-11-13-tc-records-migrate-evidence-verified-lines-link-entries-06743e
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:4d09bad98b50d172dd34616036e047ddf62e4997621a041c09afd0e14eeffb80
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:19:57Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_sessions_verified_lines_link_the_carried_entries for TC-records-migrate-evidence-verified-lines-link-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:19:57Z. The raw log stays in the machine-local store.
