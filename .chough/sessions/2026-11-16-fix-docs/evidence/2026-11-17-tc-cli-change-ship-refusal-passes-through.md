---
kind: evidence
id: 2026-11-17-tc-cli-change-ship-refusal-passes-through
aliases: []
claim: "Ran cargo test --locked --test change -- --exact a_specsync_ship_refusal_passes_through_and_records_nothing for TC-cli-change-ship-refusal-passes-through: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ship-refusal-passes-through]
case_digest: [8d36a2db3675681d9e15f929006412212e49feddbe3a17a7cca9f19a154df5c8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - a_specsync_ship_refusal_passes_through_and_records_nothing
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:63fd82a737788d44c54866ff2e53fa4e29a5f58fc12bce7ac51dcc82116e0434
criteria: [DRIVE-20]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:09Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test change -- --exact a_specsync_ship_refusal_passes_through_and_records_nothing for TC-cli-change-ship-refusal-passes-through: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:09Z. The raw log stays in the machine-local store.
