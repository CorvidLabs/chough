---
kind: evidence
id: 2026-11-12-tc-cli-migrate-preview-with-nothing-to-migrate
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_preview_with_no_0_5_x_record_says_nothing_to_migrate_and_gives_no_digest for TC-cli-migrate-preview-with-nothing-to-migrate: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-migrate-preview-with-nothing-to-migrate]
case_digest: [1840d7e76d8607f1bc5a35ad5706cd5502807c148c76634cafa8891f6253ae46]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_preview_with_no_0_5_x_record_says_nothing_to_migrate_and_gives_no_digest
exit_code: 101
tests_run: 1
duration_seconds: 8
revision: a0152789eec8d548d98dbd11f86b0818ad98f494
content: 0ef70fa8fed5b907a0059b2d0f7320b32f187bc45fae18ee147c4deb03536246
environment: local-macos-arm64
fingerprint: sha256:3c913170148c0fb7ce7ad156cbbf41750c785a13f9fd3bb1b2de9992b788a69e
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:25:25Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact a_preview_with_no_0_5_x_record_says_nothing_to_migrate_and_gives_no_digest for TC-cli-migrate-preview-with-nothing-to-migrate: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision a0152789eec8, observed 2026-11-12T20:25:25Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_preview_with_no_0_5_x_record_says_nothing_to_migrate_and_gives_no_digest stdout ----
- thread 'a_preview_with_no_0_5_x_record_says_nothing_to_migrate_and_gives_no_digest' (922829802) panicked at tests/migrate.rs:1512:13:
- migrated: gives `plan_digest`: {
-   "aliases": {},
-   "apply": "chough migrate --apply 0b5561c390d150a617684d2eb6a0309eaa715eda4c147555ad22ca99e4cdde42 --consent \"<who consented, their words and where>\"",
-   "becomes": {},
-   "follow_ups": [
-     "Declare Chough's merge drivers so generated files and bugs merge without a hand-resolved conflict (REQ-cli-047): add `.chough/bugs/*.md merge=chough-bug`, `knowledge/index.md merge=chough-generated`, `knowledge/journal/feed.xml merge=chough-generated` to `.gitattributes` and commit it, then run `chough doctor --fix` in each clone, which sets `merge.chough-bug.driver` and `merge.chough-generated.driver`."
-   ],
-   "gaps": [],
-   "git_history": "Git history still holds what earlier commits recorded; this migration does not rewrite it.",
-   "kept": {},
-   "kept_with_leaks": {},
-   "plan_digest": "0b5561c390d150a617684d2eb6a0309eaa715eda4c147555ad22ca99e4cdde42",
-   "redactions": [],
-   "removed": {},
-   "schema_version": 1,
-   "sources": {},
-   "unaccounted": [],
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 23 filtered out; finished in 6.89s
