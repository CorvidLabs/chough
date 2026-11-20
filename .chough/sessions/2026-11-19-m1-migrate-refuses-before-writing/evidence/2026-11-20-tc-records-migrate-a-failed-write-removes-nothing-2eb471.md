---
kind: evidence
id: 2026-11-20-tc-records-migrate-a-failed-write-removes-nothing-2eb471
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
duration_seconds: 4
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:dfc3cc50df5709593f7cab5fe4d5bbc2d585ead8f7afbaace14aa8d8016a7fd5
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:00:00Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact an_apply_whose_planned_write_fails_leaves_every_0_5_x_file for TC-records-migrate-a-failed-write-removes-nothing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T04:00:00Z. The raw log stays in the machine-local store.
