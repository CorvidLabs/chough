---
kind: evidence
id: 2026-11-13-tc-records-ship-names-a-configured-human-at-any-gate
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_refuses_an_unknown_human_at_a_gate_that_drives_nothing for TC-records-ship-names-a-configured-human-at-any-gate: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-ship-names-a-configured-human-at-any-gate]
case_digest: [acac730fbb7744c80420055278f73fe45ee3ee07babd08614a747617b70e2c7d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_refuses_an_unknown_human_at_a_gate_that_drives_nothing
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:dcc3f5726f8a2bdf6875228e503d3a1f6af0f71134a1b06f37ed821f72507838
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:17:13Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test change -- --exact ship_refuses_an_unknown_human_at_a_gate_that_drives_nothing for TC-records-ship-names-a-configured-human-at-any-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:17:13Z. The raw log stays in the machine-local store.
