---
kind: evidence
id: 2026-11-12-tc-records-migrate-evidence-verified-lines-link-entries-540c6d
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
duration_seconds: 1
revision: 421a7034dfce0c052d48c4d1fb5abe041c272a46
content: 9e16bb2234c3aadbf27905868186d8ca01d209615750221de4f1f7201e15af54
environment: local-macos-arm64
fingerprint: sha256:86aa430224e9cc3acd1c06eba4674ee747b09864ded12f587ecad5e604e68df3
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:06:10Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_sessions_verified_lines_link_the_carried_entries for TC-records-migrate-evidence-verified-lines-link-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 421a7034dfce, observed 2026-11-12T23:06:10Z. The raw log stays in the machine-local store.
