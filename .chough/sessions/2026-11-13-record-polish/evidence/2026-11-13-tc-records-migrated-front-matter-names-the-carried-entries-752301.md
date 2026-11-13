---
kind: evidence
id: 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-752301
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
revision: 6b7cd4766a52829847a23d03d0d93262d70feeee
content: 49a2ab3c28dbd1ec2012ee1e52591ba22d6e01e8918fc4976e1aaf668257c7a5
environment: local-macos-arm64
fingerprint: sha256:85d377312bc920e7a09beffac7edec3581ac69ff57d30074a8f8acf4a327e65d
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:12:24Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6b7cd4766a52, observed 2026-11-13T03:12:24Z. The raw log stays in the machine-local store.
