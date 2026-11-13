---
kind: evidence
id: 2026-11-13-tc-records-migrate-evidence-verified-lines-link-entries
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:833c4265b5507b26419c9f6f3f7899baf7438b2d1ddaa0bb279900169c1d0594
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:25:05Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_sessions_verified_lines_link_the_carried_entries for TC-records-migrate-evidence-verified-lines-link-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:25:05Z. The raw log stays in the machine-local store.
