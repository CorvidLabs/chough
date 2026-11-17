---
kind: evidence
id: 2026-11-17-tc-cli-change-ship-names-remaining-step
aliases: []
claim: "Ran cargo test --locked --test change -- --exact a_refused_done_move_after_ship_names_the_remaining_step for TC-cli-change-ship-names-remaining-step: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ship-names-remaining-step]
case_digest: [ab4ef2960b9be1a3b5efa10224978d131dd9a1845e55e0928c7c269f6ca57261]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - a_refused_done_move_after_ship_names_the_remaining_step
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:b88a9b209b7473fc9ace73e44dcaf622471d601771e67eb752a4bd3e38eded96
criteria: [DRIVE-20]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:14Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test change -- --exact a_refused_done_move_after_ship_names_the_remaining_step for TC-cli-change-ship-names-remaining-step: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:14Z. The raw log stays in the machine-local store.
