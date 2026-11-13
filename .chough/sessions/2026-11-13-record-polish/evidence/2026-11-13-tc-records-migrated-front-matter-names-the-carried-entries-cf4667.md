---
kind: evidence
id: 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-cf4667
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
duration_seconds: 5
revision: a1f4735a820dacdb102a4ef4d9c1bda53e31f7bb
content: 65b5bda886fd3fbadb3834aeb5a77124da7d7b9a0d3f00f12ba7d6040b8ce43b
environment: local-macos-arm64
fingerprint: sha256:70bb3ddc40409c467b76f3fa436a6d34a30f94bb72dd3b6ac02cbb0966b285a3
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:25:46Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a1f4735a820d, observed 2026-11-13T04:25:46Z. The raw log stays in the machine-local store.
