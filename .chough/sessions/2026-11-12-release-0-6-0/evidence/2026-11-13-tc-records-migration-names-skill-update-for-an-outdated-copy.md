---
kind: evidence
id: 2026-11-13-tc-records-migration-names-skill-update-for-an-outdated-copy
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:55f692261da24a347bc07014c6354261f67f5ce8e13cc906c5290af87319e1f7
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:25:54Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_migration_follow_ups_name_skill_update_while_the_skill_copy_is_not_current for TC-records-migration-names-skill-update-for-an-outdated-copy: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:25:54Z. The raw log stays in the machine-local store.
