---
kind: evidence
id: 2026-11-17-tc-records-migration-docs-name-every-key
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
duration_seconds: 1
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:bec9a20436d2801c99006d0808c453785bdbee9c95e78c31069249bd969a0753
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:42Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test migrate -- --exact the_migration_docs_name_every_key_the_preview_and_apply_print for TC-records-migration-docs-name-every-key: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:42Z. The raw log stays in the machine-local store.
