---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-a-returning-legacy-checkpoint-3cc83a
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_returning_legacy_checkpoint_holding_another_session_is_refused for TC-records-migrate-refuses-a-returning-legacy-checkpoint: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-refuses-a-returning-legacy-checkpoint]
case_digest: [2718a4355329e350b2d940a2ce154c38a5a41d0ce93402442a815a48b58854d0]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_returning_legacy_checkpoint_holding_another_session_is_refused
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:1ddf47d9b03fcf15802ef242e9867da6915806b648c2e44b5fce60bd8403a1b3
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:36:38Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_returning_legacy_checkpoint_holding_another_session_is_refused for TC-records-migrate-refuses-a-returning-legacy-checkpoint: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:36:38Z. The raw log stays in the machine-local store.
