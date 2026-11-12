---
kind: evidence
id: 2026-11-12-tc-records-migration-docs-name-every-key-7c6439
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact the_migration_docs_name_every_key_the_preview_and_apply_print for TC-records-migration-docs-name-every-key: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 0af25ea1480d1d0ce7eadd189f3a8b0c8b520fd2
content: 5511f9a2ec985a55c1f2ae5978bc45fe1b06f0386f884799ca8962145d552f86
environment: local-macos-arm64
fingerprint: sha256:08c53515ccc5e02c55ec8b0062ce1ee47c05b1ce1ddc1531cbe205c7b48272cf
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:19:53Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_migration_docs_name_every_key_the_preview_and_apply_print for TC-records-migration-docs-name-every-key: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0af25ea1480d, observed 2026-11-12T17:19:53Z. The raw log stays in the machine-local store.
