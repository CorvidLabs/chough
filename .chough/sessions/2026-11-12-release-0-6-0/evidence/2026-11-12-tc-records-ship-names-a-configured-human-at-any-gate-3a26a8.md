---
kind: evidence
id: 2026-11-12-tc-records-ship-names-a-configured-human-at-any-gate-3a26a8
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
revision: 06407b98e650d73a951f505369cb7c694765ffde
content: 20c9710c0555251958526a8d2c4296aeac3f42743b807b3fecd9dd54614bf1da
environment: local-macos-arm64
fingerprint: sha256:8732fe60afc9e29bbe0e49a22b51d7976d15993b94cf9ac7f17147331776e9e0
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:18:58Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test change -- --exact ship_refuses_an_unknown_human_at_a_gate_that_drives_nothing for TC-records-ship-names-a-configured-human-at-any-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 06407b98e650, observed 2026-11-12T21:18:58Z. The raw log stays in the machine-local store.
