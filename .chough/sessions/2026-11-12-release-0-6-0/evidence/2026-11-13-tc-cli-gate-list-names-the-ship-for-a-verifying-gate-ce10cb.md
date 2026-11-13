---
kind: evidence
id: 2026-11-13-tc-cli-gate-list-names-the-ship-for-a-verifying-gate-ce10cb
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:415f3db5d33129359066daf34f233a4c6d0ae461bb6ae93d48b6de17e893ec84
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:12:31Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why for TC-cli-gate-list-names-the-ship-for-a-verifying-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:12:31Z. The raw log stays in the machine-local store.
