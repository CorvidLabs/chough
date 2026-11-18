---
kind: evidence
id: 2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason-28a08b
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
revision: f15b163a812788a521661d411695a41255e92736
content: 1b4b3a8c8d262b5d7566cdcd2aa42bcd126c24b2bf4e8bfd4e0d2d4bee4f2c96
environment: local-macos-arm64
fingerprint: sha256:698069e5534d8c9b8394a0653ecc7a9690ac1f7701b8fb8c6224b3645a341a25
criteria: [DRIVE-16]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:45:20Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test doctor -- --exact doctor_decision_fixes_need_a_reason_and_record_no_human_decision for TC-history-a-withdrawn-stale-gate-keeps-doctors-reason: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f15b163a8127, observed 2026-11-18T20:45:20Z. The raw log stays in the machine-local store.
