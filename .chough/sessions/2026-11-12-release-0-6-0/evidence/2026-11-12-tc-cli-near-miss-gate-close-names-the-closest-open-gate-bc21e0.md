---
kind: evidence
id: 2026-11-12-tc-cli-near-miss-gate-close-names-the-closest-open-gate-bc21e0
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
duration_seconds: 1
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:a167590fd32bb05efd44f072ee744d631956720224ac882ad52ab0409e004954
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:25:45Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test relay -- --exact a_near_miss_gate_close_names_the_closest_open_gate for TC-cli-near-miss-gate-close-names-the-closest-open-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:25:45Z. The raw log stays in the machine-local store.
