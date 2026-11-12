---
kind: evidence
id: 2026-11-12-tc-cli-gate-list-names-the-ship-for-a-verifying-gate-21e743
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
duration_seconds: 1
revision: cca98758c52eb3f7b42d1f1caf751fc0e5a500c1
content: 94d89ecbe5a0f2d293255d074ccc5b065282faf8c8e453fc4e4ec5f4f093b88f
environment: local-macos-arm64
fingerprint: sha256:093df45fafe9c27aaccbeebd03caf1a518e6193b62b9604c012a83b10a1e16b7
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:03:41Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why for TC-cli-gate-list-names-the-ship-for-a-verifying-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cca98758c52e, observed 2026-11-12T21:03:41Z. The raw log stays in the machine-local store.
