---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-a-returning-legacy-checkpoint
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
duration_seconds: 0
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:5ca1f592fe54e3eac080b29c2e8bdb9a5cea913b13a62c70e66fb74d2a96a4ed
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:24:21Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_returning_legacy_checkpoint_holding_another_session_is_refused for TC-records-migrate-refuses-a-returning-legacy-checkpoint: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:24:21Z. The raw log stays in the machine-local store.
