---
kind: evidence
id: 2026-11-13-tc-cli-gate-list-names-the-ship-for-a-verifying-gate-d2d1e1
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:1cd83ae57cde6cfa57c7b18e65cfbb979f27ce84cd4779cf75567de31f8104fc
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:46:03Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why for TC-cli-gate-list-names-the-ship-for-a-verifying-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:46:03Z. The raw log stays in the machine-local store.
