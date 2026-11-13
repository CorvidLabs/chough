---
kind: evidence
id: 2026-11-13-tc-cli-next-and-status-reasons-name-no-trace-id-5c02cb
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:36d33549fdf42790a3768d44eb3a3ffd117da585f059e46646224bdc449b94c7
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:44:51Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_and_status_reasons_name_no_trace_id for TC-cli-next-and-status-reasons-name-no-trace-id: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:44:51Z. The raw log stays in the machine-local store.
