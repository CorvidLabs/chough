---
kind: evidence
id: 2026-11-18-tc-records-a-second-same-day-migration-names-its-own-receipt
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
duration_seconds: 1
revision: 47bfcd1bd75af6df73258d3c6bd20aba692d4441
content: 568cd36dc6884a0f83742cb710d7a9515705fa9e314f241dc64af0d95c089499
environment: local-macos-arm64
fingerprint: sha256:f921a60e05561e7e3611765f588e69ab37f51fe925a750e55e8c579aafb733dc
criteria: [LIFE-20, DRIVE-27]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:39:21Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_second_same_day_migration_names_its_own_receipt for TC-records-a-second-same-day-migration-names-its-own-receipt: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 47bfcd1bd75a, observed 2026-11-18T19:39:21Z. The raw log stays in the machine-local store.
