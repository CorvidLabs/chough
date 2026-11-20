---
kind: evidence
id: 2026-11-20-tc-records-migrate-refuses-a-returning-legacy-checkpoint-68d47f
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
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:21e4919ed47d1581e7c29850b334e1fe3f1c0da722151033ae0bf8b3baf27919
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:59:54Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_returning_legacy_checkpoint_holding_another_session_is_refused for TC-records-migrate-refuses-a-returning-legacy-checkpoint: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T03:59:54Z. The raw log stays in the machine-local store.
