---
kind: evidence
id: 2026-11-17-tc-cli-skill-routine-fast-path
aliases: []
claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-skill-routine-fast-path: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-skill-routine-fast-path]
case_digest: [e62cde548f87528c33e0efbbdba70c250a0b8084297df78bb645445f5c909b22]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - package
  - "--"
  - "--exact"
  - the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:83148c6f5fa455ec9f39a3ddef7d790c8334225122ec032963806c70afb33222
criteria: [LIFE-22]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:39Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-skill-routine-fast-path: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:39Z. The raw log stays in the machine-local store.
