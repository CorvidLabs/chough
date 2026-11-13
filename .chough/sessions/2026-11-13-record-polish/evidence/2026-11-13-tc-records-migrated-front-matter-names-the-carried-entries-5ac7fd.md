---
kind: evidence
id: 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-5ac7fd
aliases: []
claim: "Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrated-front-matter-names-the-carried-entries]
case_digest: [31d3e1b6c6c9b6299ba3da51213934d7bb063b3a4911db955f6741f6fe01930b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate_evidence
  - "--"
  - "--exact"
  - migrated_front_matter_names_the_carried_entries
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 3453e6e8a2edd34624c6c89b620ed8b262aa3a2c
content: 13e8fbe68ad3139b1c422a706d9abd7d321e4259afebf7c54f9ab96bea045d8e
environment: local-macos-arm64
fingerprint: sha256:9c0424c05f5c17c52d120faa14243618564a75f8f90c15bf0a6dbbfd70397b09
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:07:46Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3453e6e8a2ed, observed 2026-11-13T06:07:46Z. The raw log stays in the machine-local store.
