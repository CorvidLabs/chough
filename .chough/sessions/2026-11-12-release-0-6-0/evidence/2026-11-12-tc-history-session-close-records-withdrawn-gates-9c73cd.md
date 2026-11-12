---
kind: evidence
id: 2026-11-12-tc-history-session-close-records-withdrawn-gates-9c73cd
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
duration_seconds: 2
revision: d2bfc0cbc9eac5434a098b07c0c0e6fc71749fab
content: c167f38981150103d574cdb2f6a2779658bc5798d5c97a9c2b7e7bfa71a9a497
environment: local-macos-arm64
fingerprint: sha256:64236e4df3560e3c26f1f241b9ce94b300552ee7a96ae4f489993cc08900b472
criteria: [DRIVE-16, FLOW-13]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:13:43Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact session_close_records_the_gates_it_withdraws for TC-history-session-close-records-withdrawn-gates: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d2bfc0cbc9ea, observed 2026-11-12T22:13:43Z. The raw log stays in the machine-local store.
