---
kind: evidence
id: 2026-11-13-tc-cli-scope-gate-summary-says-adds-and-modifies-52ebfd
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_says_what_each_delta_adds_and_what_it_modifies for TC-cli-scope-gate-summary-says-adds-and-modifies: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-scope-gate-summary-says-adds-and-modifies]
case_digest: [d377a0edb3b198fede5a257e3ee6f64658054dbf22a66b9584fa41d8dfd2948f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_scope_gate_summary_says_what_each_delta_adds_and_what_it_modifies
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 8e0feedecb1cbe101c723067a5c300d2cbad07fc
content: a4cfc78f0189db0c9bae41beff3e0201fb02123516fe566e98c9dd7f65c48295
environment: local-macos-arm64
fingerprint: sha256:4ddb749ae2550dd15323a46c55bffe89760f1a4aa01288daa52f1f7077688e70
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:34:09Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_says_what_each_delta_adds_and_what_it_modifies for TC-cli-scope-gate-summary-says-adds-and-modifies: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8e0feedecb1c, observed 2026-11-13T02:34:09Z. The raw log stays in the machine-local store.
