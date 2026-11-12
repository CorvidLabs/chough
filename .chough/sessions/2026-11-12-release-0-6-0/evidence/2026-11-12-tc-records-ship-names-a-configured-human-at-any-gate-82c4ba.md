---
kind: evidence
id: 2026-11-12-tc-records-ship-names-a-configured-human-at-any-gate-82c4ba
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:67315705a0f68f1f0cb72b6ec499c62d09d87910298ae2e4641605da696dc4ee
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:21:10Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test change -- --exact ship_refuses_an_unknown_human_at_a_gate_that_drives_nothing for TC-records-ship-names-a-configured-human-at-any-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:21:10Z. The raw log stays in the machine-local store.
