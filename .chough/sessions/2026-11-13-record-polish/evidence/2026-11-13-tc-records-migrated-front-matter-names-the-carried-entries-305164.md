---
kind: evidence
id: 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-305164
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
revision: f9b22f2215f9f61de538bbc17c45829385438cba
content: 84a021f93c1361a396a48300717a2b72419f9657749cffc84f701402a1bc2d9a
environment: local-macos-arm64
fingerprint: sha256:7d38e7a41900f22f49c5bda58018093f542e4604b53c1f9050b60d9d9bf979f0
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:30:21Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f9b22f2215f9, observed 2026-11-13T03:30:21Z. The raw log stays in the machine-local store.
