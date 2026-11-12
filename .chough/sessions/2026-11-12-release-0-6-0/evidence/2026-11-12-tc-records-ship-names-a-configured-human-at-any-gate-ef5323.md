---
kind: evidence
id: 2026-11-12-tc-records-ship-names-a-configured-human-at-any-gate-ef5323
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
revision: c53da06cc673269174e8c292895f977d091bc88b
content: 03df8cca3a1a59d6c22f7023c528bb2e162676aef0e142cb83ca1221bdb1f179
environment: local-macos-arm64
fingerprint: sha256:1cea2e1a8a4012566fb94dfd7f3ddc844fc4f31d1729a3ec36cf69ce58f53f19
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:09:48Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test change -- --exact ship_refuses_an_unknown_human_at_a_gate_that_drives_nothing for TC-records-ship-names-a-configured-human-at-any-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c53da06cc673, observed 2026-11-12T20:09:48Z. The raw log stays in the machine-local store.
