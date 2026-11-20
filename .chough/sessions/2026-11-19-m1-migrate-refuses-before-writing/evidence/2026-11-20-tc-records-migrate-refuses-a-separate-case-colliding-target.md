---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-a-separate-case-colliding-target
aliases: []
claim: "Ran cargo test --locked --test migrate_knowledge -- --exact a_separate_case_colliding_target_is_replaced_only_as_previewed for TC-records-migrate-refuses-a-separate-case-colliding-target: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-refuses-a-separate-case-colliding-target]
case_digest: [88dc76561c5a6932090b7d2af1867c1b84a3f9fdf78cae929276c8560ca7e523]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate_knowledge
  - "--"
  - "--exact"
  - a_separate_case_colliding_target_is_replaced_only_as_previewed
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:968094ce8c664a26f5193163c3b729b6a21b1d0402efd36a80aa94b94c5a5ddd
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:24:29Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate_knowledge -- --exact a_separate_case_colliding_target_is_replaced_only_as_previewed for TC-records-migrate-refuses-a-separate-case-colliding-target: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:24:29Z. The raw log stays in the machine-local store.
