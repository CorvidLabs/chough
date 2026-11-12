---
kind: evidence
id: 2026-11-12-tc-records-every-gate-close-names-a-configured-human-9b34bf
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
duration_seconds: 1
revision: c53da06cc673269174e8c292895f977d091bc88b
content: 03df8cca3a1a59d6c22f7023c528bb2e162676aef0e142cb83ca1221bdb1f179
environment: local-macos-arm64
fingerprint: sha256:88db725ee33dc6633a1ec4435d01085b235c4c6246ce0c4580b653af3de31b3b
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:09:44Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact every_gate_close_names_a_configured_human for TC-records-every-gate-close-names-a-configured-human: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c53da06cc673, observed 2026-11-12T20:09:44Z. The raw log stays in the machine-local store.
