---
kind: evidence
id: 2026-11-13-tc-cli-scope-gate-summary-says-adds-and-modifies-5bbd77
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
duration_seconds: 5
revision: 8e0feedecb1cbe101c723067a5c300d2cbad07fc
content: c0f0b00cf919edc060e13a53052e75f763bf06a5e72946b6e63ee37d4860871c
environment: local-macos-arm64
fingerprint: sha256:633e843f50b8c1cc20bb0f75e5bbd2c18283ee0e016161dfc8ae7b90fb76847f
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:25:50Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_says_what_each_delta_adds_and_what_it_modifies for TC-cli-scope-gate-summary-says-adds-and-modifies: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8e0feedecb1c, observed 2026-11-13T02:25:50Z. The raw log stays in the machine-local store.
