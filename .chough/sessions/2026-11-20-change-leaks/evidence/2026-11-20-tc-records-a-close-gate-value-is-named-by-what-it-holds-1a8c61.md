---
kind: evidence
id: 2026-11-20-tc-records-a-close-gate-value-is-named-by-what-it-holds-1a8c61
aliases: []
claim: "Ran cargo test --locked --test change -- --exact change_ship_names_a_close_gate_value_by_what_it_holds for TC-records-a-close-gate-value-is-named-by-what-it-holds: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-close-gate-value-is-named-by-what-it-holds]
case_digest: [11bd6ccb32063c522912aa0289696c3baed152d01df546e78d86be20ff2caf19]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - change_ship_names_a_close_gate_value_by_what_it_holds
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: f34dcc8391b47cd2b32c04a89933af75497bc177
content: 9c46ada16719ee7f5bc1b93a1452cb6032aad8f285116a7b70082427b6284a88
environment: local-macos-arm64
fingerprint: sha256:a8b894a3dd9b63f0341cabfcab2602e676dbec91ef43738b0c62fb45da311cca
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T22:15:41Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test change -- --exact change_ship_names_a_close_gate_value_by_what_it_holds for TC-records-a-close-gate-value-is-named-by-what-it-holds: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f34dcc8391b4, observed 2026-11-20T22:15:41Z. The raw log stays in the machine-local store.
