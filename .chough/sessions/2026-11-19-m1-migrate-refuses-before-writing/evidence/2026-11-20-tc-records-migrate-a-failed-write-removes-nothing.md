---
kind: evidence
id: 2026-11-20-tc-records-migrate-a-failed-write-removes-nothing
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
duration_seconds: 1
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:db63009502dbf5a78d4d3fbf92c4ecce61283057b01003085217da5f263ff5f3
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:24:24Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_apply_whose_planned_write_fails_leaves_every_0_5_x_file for TC-records-migrate-a-failed-write-removes-nothing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:24:24Z. The raw log stays in the machine-local store.
