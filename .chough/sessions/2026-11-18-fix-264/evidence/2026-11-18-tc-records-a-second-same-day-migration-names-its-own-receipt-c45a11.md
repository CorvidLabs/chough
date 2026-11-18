---
kind: evidence
id: 2026-11-18-tc-records-a-second-same-day-migration-names-its-own-receipt-c45a11
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_second_same_day_migration_names_its_own_receipt for TC-records-a-second-same-day-migration-names-its-own-receipt: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-second-same-day-migration-names-its-own-receipt]
case_digest: [bebb426e60e8734dd18da8ba6557f50853b1b34d538e336dade2a041cb555ae9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_second_same_day_migration_names_its_own_receipt
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 75865bab698e97524d4f36fd67c17a15ea4803a1
content: 28e558987941084f772ef8b48c5d465d3fdc025c02bb36ac8194e42236a13abb
environment: local-macos-arm64
fingerprint: sha256:6202b7b2d595634cb46436411484ff43c7163b5988672ec51222c2d683c85e52
criteria: [LIFE-20, DRIVE-27]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:55:32Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_second_same_day_migration_names_its_own_receipt for TC-records-a-second-same-day-migration-names-its-own-receipt: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 75865bab698e, observed 2026-11-18T19:55:32Z. The raw log stays in the machine-local store.
