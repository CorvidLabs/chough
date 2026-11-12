---
kind: evidence
id: 2026-11-12-tc-records-migration-names-skill-update-for-an-outdated-copy
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact the_migration_follow_ups_name_skill_update_while_the_skill_copy_is_not_current for TC-records-migration-names-skill-update-for-an-outdated-copy: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 3
revision: 730aba9f2157cb64d9df9c89788b5df975ad6e9d
content: 1c4f407c8784eb8b298a8337feb1bf3ce8a34c5bd3774e002b037f307e191ac7
environment: local-macos-arm64
fingerprint: sha256:7244b1133c9e1d21aab5c34e98f97145f704c5c39379beb78f10216084b38fd8
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:48:09Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_migration_follow_ups_name_skill_update_while_the_skill_copy_is_not_current for TC-records-migration-names-skill-update-for-an-outdated-copy: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 730aba9f2157, observed 2026-11-12T23:48:09Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_migration_follow_ups_name_skill_update_while_the_skill_copy_is_not_current stdout ----
- thread 'the_migration_follow_ups_name_skill_update_while_the_skill_copy_is_not_current' (928381282) panicked at tests/migrate.rs:1674:9:
- assertion `left == right` failed: Some("v0.5.0"): ["Declare Chough's merge drivers so generated files and bugs merge without a hand-resolved conflict (REQ-cli-047): add `.chough/bugs/*.md merge=chough-bug`, `knowledge/index.md merge=chough-generated`, `knowledge/journal/feed.xml merge=chough-generated` to `.gitattributes` and commit it, then run `chough doctor --fix` in each clone, which sets `merge.chough-bug.driver` and `merge.chough-generated.driver`."]
-   left: false
-  right: true
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 25 filtered out; finished in 1.80s
