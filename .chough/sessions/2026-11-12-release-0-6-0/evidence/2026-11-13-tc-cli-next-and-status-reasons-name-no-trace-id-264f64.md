---
kind: evidence
id: 2026-11-13-tc-cli-next-and-status-reasons-name-no-trace-id-264f64
aliases: []
claim: "Ran cargo test --locked --test next -- --exact next_and_status_reasons_name_no_trace_id for TC-cli-next-and-status-reasons-name-no-trace-id: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-and-status-reasons-name-no-trace-id]
case_digest: [c1b8719fd99cd1139582c7205c4fda82f746a4fdfcb338cbc3694e73381b3ee9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - next_and_status_reasons_name_no_trace_id
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:7fbc2da33423fc1ffb674d4021838c31b88a0835d4a8b2bdc28c0063207afeac
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:11:01Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_and_status_reasons_name_no_trace_id for TC-cli-next-and-status-reasons-name-no-trace-id: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:11:01Z. The raw log stays in the machine-local store.
