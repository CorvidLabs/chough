---
kind: evidence
id: 2026-11-12-tc-records-migration-docs-name-every-key-8f1252
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
duration_seconds: 5
revision: 5f1c57c81296da2f1afd3180266844051680dcb3
content: d147d2faee47f0dbef9a0b63332105852ce094a35699f89beebee474e82c10c9
environment: local-macos-arm64
fingerprint: sha256:a6765ae2c07f1a13a2bf56c3976ef8eabbdf8fc247e53c59e005edc049260bac
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:36:39Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_migration_docs_name_every_key_the_preview_and_apply_print for TC-records-migration-docs-name-every-key: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5f1c57c81296, observed 2026-11-12T16:36:39Z. The raw log stays in the machine-local store.
