---
kind: evidence
id: 2026-11-12-tc-records-migration-names-skill-update-for-an-outdated-copy-5ce0c9
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact the_migration_follow_ups_name_skill_update_while_the_skill_copy_is_not_current for TC-records-migration-names-skill-update-for-an-outdated-copy: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migration-names-skill-update-for-an-outdated-copy]
case_digest: [1e2d985012553b96ec09fec3d230ca0ee0d36fd14039a2f9d21285a03b07056f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - the_migration_follow_ups_name_skill_update_while_the_skill_copy_is_not_current
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: f618819ba23c3f8e65889d8f2871ea26a7a1cf81
content: 685d17b46d297a43c78012dededcb09f0048647d30d26e9c935cb460fd549e1c
environment: local-macos-arm64
fingerprint: sha256:ae024e4380367c9b633de6f53e57e7c1ade24c39339e386091da17e07dcdc403
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:49:58Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_migration_follow_ups_name_skill_update_while_the_skill_copy_is_not_current for TC-records-migration-names-skill-update-for-an-outdated-copy: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f618819ba23c, observed 2026-11-12T23:49:58Z. The raw log stays in the machine-local store.
