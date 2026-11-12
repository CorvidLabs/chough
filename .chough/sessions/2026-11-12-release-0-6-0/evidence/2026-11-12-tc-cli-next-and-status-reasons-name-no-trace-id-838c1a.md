---
kind: evidence
id: 2026-11-12-tc-cli-next-and-status-reasons-name-no-trace-id-838c1a
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
revision: 06407b98e650d73a951f505369cb7c694765ffde
content: 20c9710c0555251958526a8d2c4296aeac3f42743b807b3fecd9dd54614bf1da
environment: local-macos-arm64
fingerprint: sha256:8546c26037edfa4d25e92585872b1fbf5f2dab4fc93c9996efb953246d344d00
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:20:07Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_and_status_reasons_name_no_trace_id for TC-cli-next-and-status-reasons-name-no-trace-id: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 06407b98e650, observed 2026-11-12T21:20:07Z. The raw log stays in the machine-local store.
