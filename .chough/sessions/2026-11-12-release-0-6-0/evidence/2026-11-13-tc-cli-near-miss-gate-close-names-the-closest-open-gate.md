---
kind: evidence
id: 2026-11-13-tc-cli-near-miss-gate-close-names-the-closest-open-gate
aliases: []
claim: "Ran cargo test --locked --test relay -- --exact a_near_miss_gate_close_names_the_closest_open_gate for TC-cli-near-miss-gate-close-names-the-closest-open-gate: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-near-miss-gate-close-names-the-closest-open-gate]
case_digest: [006dc3d493a282c6a24f9210a48831f759e21c98f701b8a0a909efbed1c9f4a9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - relay
  - "--"
  - "--exact"
  - a_near_miss_gate_close_names_the_closest_open_gate
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:ecffd934c992281f165741e84fd7c5e18dfba457283d2b7cd300ee0de51b32ff
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:23:19Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test relay -- --exact a_near_miss_gate_close_names_the_closest_open_gate for TC-cli-near-miss-gate-close-names-the-closest-open-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:23:19Z. The raw log stays in the machine-local store.
