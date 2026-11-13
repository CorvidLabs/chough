---
kind: evidence
id: 2026-11-13-tc-records-every-gate-close-names-a-configured-human
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:1856e3b2759135c9cd70882694e879d40e31d82bf104b2cff772d1c8277d1565
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:17:02Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact every_gate_close_names_a_configured_human for TC-records-every-gate-close-names-a-configured-human: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:17:02Z. The raw log stays in the machine-local store.
