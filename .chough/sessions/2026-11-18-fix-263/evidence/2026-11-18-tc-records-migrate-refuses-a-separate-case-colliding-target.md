---
kind: evidence
id: 2026-11-18-tc-records-migrate-refuses-a-separate-case-colliding-target
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
duration_seconds: 1
revision: 746cc53f41ca9bfb7ee17a2e0601be0cb5cb9151
content: 2db3928fe2c093d311e600a53dcb347d3c31cd6c83c6c537dfa4ca5389b25042
environment: local-linux-aarch64
fingerprint: sha256:a13cebd46a5468e9e29d108aa351d0adce32babbdbfa4e223b13831cf84ca7d6
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T15:29:05Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test migrate_knowledge -- --exact a_separate_file_at_a_case_colliding_target_refuses_before_any_write for TC-records-migrate-refuses-a-separate-case-colliding-target: passed (exit 0, tests run 1).**

Recorded as passed in local-linux-aarch64 at revision 746cc53f41ca, observed 2026-11-18T15:29:05Z. The raw log stays in the machine-local store.
