---
kind: evidence
id: 2026-11-17-tc-cli-change-ship-checks-before-running
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_refuses_before_running_without_the_relay_or_outside_accept for TC-cli-change-ship-checks-before-running: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ship-checks-before-running]
case_digest: [160fa3d8fe09c2ef7923e584ec2c2380cb1a6e4c611f1a7dee77ca715c88adc2]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_refuses_before_running_without_the_relay_or_outside_accept
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:e3798bb5b11f6b8f52aa3e7204419b17498bad9808ae9387b301eb9724d048d8
criteria: [DRIVE-20, FLOW-16]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:13Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test change -- --exact ship_refuses_before_running_without_the_relay_or_outside_accept for TC-cli-change-ship-checks-before-running: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:13Z. The raw log stays in the machine-local store.
