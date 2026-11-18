---
kind: evidence
id: 2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason
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
revision: 09d0964f84966cfac446046fd3c0199e0652b060
content: 3b74f3fc1842a5faf92997e9dc3f2ae31bae5aa04358372bda0014ff48b392d1
environment: local-macos-arm64
fingerprint: sha256:8d7ba52be67c2b80ae1965ecefd0ca3deca69ae4cba9e0c7ad2b2379cb68ae34
criteria: [DRIVE-16]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:14:12Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test doctor -- --exact doctor_decision_fixes_need_a_reason_and_record_no_human_decision for TC-history-a-withdrawn-stale-gate-keeps-doctors-reason: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 09d0964f8496, observed 2026-11-18T20:14:12Z. The raw log stays in the machine-local store.
