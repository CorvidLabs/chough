---
kind: evidence
id: 2026-11-12-tc-records-migration-docs-name-every-key
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact the_migration_docs_name_every_key_the_preview_and_apply_print for TC-records-migration-docs-name-every-key: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migration-docs-name-every-key]
case_digest: [c6d117fb12738b2917a4f7db7f39eb90db5065e1cfa89c5a2308f0864251b9af]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - the_migration_docs_name_every_key_the_preview_and_apply_print
exit_code: 101
tests_run: 1
duration_seconds: 12
revision: e005da2e408386cbc35a21498f87135c42952257
content: ea3944c1edfdad8431229957114e4716be4f731c5c63f406c4ae3b3d649b5470
environment: local-macos-arm64
fingerprint: sha256:7271250f5bbdaf04253677b42a5482f92a602e4920f87a7c6789c26b6e48f873
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:34:37Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_migration_docs_name_every_key_the_preview_and_apply_print for TC-records-migration-docs-name-every-key: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision e005da2e4083, observed 2026-11-12T16:34:37Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_migration_docs_name_every_key_the_preview_and_apply_print stdout ----
- thread 'the_migration_docs_name_every_key_the_preview_and_apply_print' (919260987) panicked at tests/migrate.rs:1418:13:
- the Migration section does not name the preview's `apply`
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 19 filtered out; finished in 7.30s
