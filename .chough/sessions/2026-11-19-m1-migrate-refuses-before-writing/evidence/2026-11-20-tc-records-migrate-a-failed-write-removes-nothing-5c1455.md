---
kind: evidence
id: 2026-11-20-tc-records-migrate-a-failed-write-removes-nothing-5c1455
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact an_apply_whose_planned_write_fails_leaves_every_0_5_x_file for TC-records-migrate-a-failed-write-removes-nothing: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-a-failed-write-removes-nothing]
case_digest: [72d5932eebb5ce0f17c5d5cb6cd34a44f1ba11bd5ecf5ad8beed21026ce8b7d3]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - an_apply_whose_planned_write_fails_leaves_every_0_5_x_file
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:b8ab6c87826618471121b88084988d0ce3085f1d4983cef224c01bdbdd1111fa
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:36:43Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_apply_whose_planned_write_fails_leaves_every_0_5_x_file for TC-records-migrate-a-failed-write-removes-nothing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:36:43Z. The raw log stays in the machine-local store.
