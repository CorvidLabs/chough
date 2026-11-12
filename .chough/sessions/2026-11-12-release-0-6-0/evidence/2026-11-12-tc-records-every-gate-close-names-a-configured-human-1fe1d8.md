---
kind: evidence
id: 2026-11-12-tc-records-every-gate-close-names-a-configured-human-1fe1d8
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
duration_seconds: 2
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:2c19c99abfb7494f8a6bb78e198c2cf69d3d88e47e1905a8a64e70671c03f491
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:21:02Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact every_gate_close_names_a_configured_human for TC-records-every-gate-close-names-a-configured-human: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:21:02Z. The raw log stays in the machine-local store.
