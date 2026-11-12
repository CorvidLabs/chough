---
kind: evidence
id: 2026-11-12-tc-records-every-gate-close-names-a-configured-human-f93c42
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact every_gate_close_names_a_configured_human for TC-records-every-gate-close-names-a-configured-human: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-every-gate-close-names-a-configured-human]
case_digest: [0cc23c38572566605fb9a9bdc0daed35c6d6a3de5e4db2aac8e74afd4945bf42]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - every_gate_close_names_a_configured_human
exit_code: 0
tests_run: 1
duration_seconds: 4
revision: f5d8ed79a3dce604e1248e0a449155e616f5ea52
content: a32e5e6960e5f8ec19a5fd5d9759b0ec61636ff51c6d20ba0608b9b70d260e21
environment: local-macos-arm64
fingerprint: sha256:aaf96d95f2565b8eecaec12c22848c08be6e61a66dc5284dcc29cbf2236d3df1
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:06:12Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact every_gate_close_names_a_configured_human for TC-records-every-gate-close-names-a-configured-human: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f5d8ed79a3dc, observed 2026-11-12T20:06:12Z. The raw log stays in the machine-local store.
