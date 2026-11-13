---
kind: evidence
id: 2026-11-13-tc-cli-scope-gate-summary-says-adds-and-modifies-ae4e21
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
revision: 4c6e1123c572bbcdd7e4773dac3978c26a38165f
content: 45274f8cb4df179d48551c089c09125f35538fe2807481ec7d30db8d80ab42c6
environment: local-macos-arm64
fingerprint: sha256:ad3805633ee1a16f4a6e09949191b5d4da9851a874fe15e899268fd07ff25033
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:39:31Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_says_what_each_delta_adds_and_what_it_modifies for TC-cli-scope-gate-summary-says-adds-and-modifies: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c6e1123c572, observed 2026-11-13T02:39:31Z. The raw log stays in the machine-local store.
