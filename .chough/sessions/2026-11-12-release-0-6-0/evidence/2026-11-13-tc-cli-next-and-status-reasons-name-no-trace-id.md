---
kind: evidence
id: 2026-11-13-tc-cli-next-and-status-reasons-name-no-trace-id
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:1dbd5e2bcf80b99546fa2fa6072b4251b8b36c48f1698fa285926778865e7416
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:19:22Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_and_status_reasons_name_no_trace_id for TC-cli-next-and-status-reasons-name-no-trace-id: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:19:22Z. The raw log stays in the machine-local store.
