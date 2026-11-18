---
kind: evidence
id: 2026-11-18-tc-records-migrate-refuses-a-separate-case-colliding-target-406fa9
aliases: []
claim: "Ran cargo test --locked --test migrate_knowledge -- --exact a_separate_file_at_a_case_colliding_target_refuses_before_any_write for TC-records-migrate-refuses-a-separate-case-colliding-target: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-refuses-a-separate-case-colliding-target]
case_digest: [34d6123e61eb37e076ff207bfb1e630d5f9e8c558531947e9d8c9fbc83b74700]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate_knowledge
  - "--"
  - "--exact"
  - a_separate_file_at_a_case_colliding_target_refuses_before_any_write
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 34c68f6f9f6ee7e0d0b8c7d717daee36c044ee87
content: fa5ad1b3c26bda7f3f1380203a3b963960b40c750add7816ce158d9a168baf38
environment: local-linux-aarch64
fingerprint: sha256:d3574401928333cb8869bfbf9fae21e8163cbb186866c6f2b741b18c5f92aa68
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:25:12Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test migrate_knowledge -- --exact a_separate_file_at_a_case_colliding_target_refuses_before_any_write for TC-records-migrate-refuses-a-separate-case-colliding-target: passed (exit 0, tests run 1).**

Recorded as passed in local-linux-aarch64 at revision 34c68f6f9f6e, observed 2026-11-18T19:25:12Z. The raw log stays in the machine-local store.
