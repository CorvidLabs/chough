---
kind: evidence
id: 2026-11-12-tc-cli-next-and-status-reasons-name-no-trace-id-d0f752
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
duration_seconds: 1
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:8757e072d46898e90c4e64c368d0f22f92f986be39b1af87c632c959d1745c73
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:22:49Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_and_status_reasons_name_no_trace_id for TC-cli-next-and-status-reasons-name-no-trace-id: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:22:49Z. The raw log stays in the machine-local store.
