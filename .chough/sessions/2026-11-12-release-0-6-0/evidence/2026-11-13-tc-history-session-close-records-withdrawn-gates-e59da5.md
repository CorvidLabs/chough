---
kind: evidence
id: 2026-11-13-tc-history-session-close-records-withdrawn-gates-e59da5
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact session_close_records_the_gates_it_withdraws for TC-history-session-close-records-withdrawn-gates: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-session-close-records-withdrawn-gates]
case_digest: [c4977c8214f983edfae431e9f8aa07532b708b053ed2d102ab5ee83385de500f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - session_close_records_the_gates_it_withdraws
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:a454042d431e6719744f7b10a9b391493fff2279a6f0ff438d0eb26d4a66506b
criteria: [DRIVE-16, FLOW-13]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:15:11Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact session_close_records_the_gates_it_withdraws for TC-history-session-close-records-withdrawn-gates: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:15:11Z. The raw log stays in the machine-local store.
