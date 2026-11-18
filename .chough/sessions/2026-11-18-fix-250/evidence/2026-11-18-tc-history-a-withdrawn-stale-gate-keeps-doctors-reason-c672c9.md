---
kind: evidence
id: 2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason-c672c9
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
duration_seconds: 1
revision: 5a90b43641ad6275fbea8ce10c3f88bc2e263f90
content: 6fb721e2c9d0bd4cd2a65692c6ba9e36edf7c6f7eaeddda6ed3c1cd453108a40
environment: local-macos-arm64
fingerprint: sha256:8a63b44418008ce084b50c9e4aed4883c77aceecf616c4fff94c8f3a12ef50a3
criteria: [DRIVE-16]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:38:07Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test doctor -- --exact doctor_decision_fixes_need_a_reason_and_record_no_human_decision for TC-history-a-withdrawn-stale-gate-keeps-doctors-reason: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5a90b43641ad, observed 2026-11-18T20:38:07Z. The raw log stays in the machine-local store.
