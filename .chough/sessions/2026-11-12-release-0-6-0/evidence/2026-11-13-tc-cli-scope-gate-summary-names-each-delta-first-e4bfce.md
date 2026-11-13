---
kind: evidence
id: 2026-11-13-tc-cli-scope-gate-summary-names-each-delta-first-e4bfce
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_each_delta_first_with_its_requirements for TC-cli-scope-gate-summary-names-each-delta-first: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-scope-gate-summary-names-each-delta-first]
case_digest: [4a2d1c45218f0e0dfb72c5e4f6488080990be96638ee6943e2b77d4fdb233e4b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_scope_gate_summary_names_each_delta_first_with_its_requirements
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:ac34cedd1e4cd119c41d7f879aee48ea578d11844c8da73b25d55b04bf6b45ca
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:50:21Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_each_delta_first_with_its_requirements for TC-cli-scope-gate-summary-names-each-delta-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:50:21Z. The raw log stays in the machine-local store.
