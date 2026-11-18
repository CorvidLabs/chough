---
kind: evidence
id: 2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason-070019
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_decision_fixes_need_a_reason_and_record_no_human_decision for TC-history-a-withdrawn-stale-gate-keeps-doctors-reason: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-withdrawn-stale-gate-keeps-doctors-reason]
case_digest: [47f16784260598a9d008378c42aec8eddbededade3d65ba645402a6c3a37abfa]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_decision_fixes_need_a_reason_and_record_no_human_decision
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 67b4d40fef05fe5485f59c15da1cf33eec8dada9
content: 067657247f5e370a52f8956229e4f71751d2a86c5b5e353440ebd16eeae6c627
environment: local-macos-arm64
fingerprint: sha256:a36f8e706dd79a373e21d8830d50130c6b716749237a75a31f690b6964910343
criteria: [DRIVE-16]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T21:20:14Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test doctor -- --exact doctor_decision_fixes_need_a_reason_and_record_no_human_decision for TC-history-a-withdrawn-stale-gate-keeps-doctors-reason: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67b4d40fef05, observed 2026-11-18T21:20:14Z. The raw log stays in the machine-local store.
