---
kind: evidence
id: 2026-11-17-tc-cli-change-ship-closes-gate-and-moves-to-done
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_with_the_review_relay_ships_closes_the_gate_and_moves_to_done for TC-cli-change-ship-closes-gate-and-moves-to-done: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ship-closes-gate-and-moves-to-done]
case_digest: [09da149a1168d019339682b0dbf57a61b91469e84ed3945ec956d2f12df1bce7]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_with_the_review_relay_ships_closes_the_gate_and_moves_to_done
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:b60485bbdcca14a41624da1e4dc033ec1a2e3d7c3622888fb076cfa1155a16df
criteria: [DRIVE-20, FLOW-16]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:09Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test change -- --exact ship_with_the_review_relay_ships_closes_the_gate_and_moves_to_done for TC-cli-change-ship-closes-gate-and-moves-to-done: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:09Z. The raw log stays in the machine-local store.
