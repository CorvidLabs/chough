---
kind: evidence
id: 2026-11-20-tc-records-a-close-gate-value-is-named-by-what-it-holds
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
revision: 8ead84204edc55fcf5c30db35fb0beb6b7290cb2
content: 6880a79a758fb134e06561e2e779a9d6a9025ddc5484976b625f5c52cc4b2dea
environment: local-macos-arm64
fingerprint: sha256:2b52e559ca725b8c4c24358ee972213e894949dacf4f598b869339e48fddbe8b
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:34:02Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test change -- --exact change_ship_names_a_close_gate_value_by_what_it_holds for TC-records-a-close-gate-value-is-named-by-what-it-holds: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8ead84204edc, observed 2026-11-20T17:34:02Z. The raw log stays in the machine-local store.
