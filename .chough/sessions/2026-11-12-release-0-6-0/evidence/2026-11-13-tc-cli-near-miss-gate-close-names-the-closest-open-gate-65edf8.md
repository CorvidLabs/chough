---
kind: evidence
id: 2026-11-13-tc-cli-near-miss-gate-close-names-the-closest-open-gate-65edf8
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:6babe346095ccba2e79c878a89961047650b496b87d3a47f09170c04ce1340b9
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:49:51Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test relay -- --exact a_near_miss_gate_close_names_the_closest_open_gate for TC-cli-near-miss-gate-close-names-the-closest-open-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:49:51Z. The raw log stays in the machine-local store.
