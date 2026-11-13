---
kind: evidence
id: 2026-11-13-tc-cli-gate-list-names-the-ship-for-a-verifying-gate
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why for TC-cli-gate-list-names-the-ship-for-a-verifying-gate: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-list-names-the-ship-for-a-verifying-gate]
case_digest: [7c6dcca7865896309780ac2c8587318876e678380ef07f5c99f1dff3a2743cfd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:cd38d7fb423ee361b2bdda7c664da375d93488a8c6da1fd4175e9eaad5af0531
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:20:20Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why for TC-cli-gate-list-names-the-ship-for-a-verifying-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:20:20Z. The raw log stays in the machine-local store.
