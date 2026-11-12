---
kind: evidence
id: 2026-11-12-tc-cli-near-miss-gate-close-names-the-closest-open-gate-660b00
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
duration_seconds: 0
revision: e0131c098169579c13f2498db93514969f149f33
content: f90f0ac78534841119984ed8ae2779eb2acada66c1c412a9c38a7acfd38b6442
environment: local-macos-arm64
fingerprint: sha256:c662406365a886c827a213df9cabe39573a3873497f295f865f22862db03f182
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:02:05Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test relay -- --exact a_near_miss_gate_close_names_the_closest_open_gate for TC-cli-near-miss-gate-close-names-the-closest-open-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e0131c098169, observed 2026-11-12T23:02:05Z. The raw log stays in the machine-local store.
