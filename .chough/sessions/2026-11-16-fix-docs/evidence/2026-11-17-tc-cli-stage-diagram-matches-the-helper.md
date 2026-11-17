---
kind: evidence
id: 2026-11-17-tc-cli-stage-diagram-matches-the-helper
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact stage_diagrams_draw_only_the_helpers_stages_and_moves for TC-cli-stage-diagram-matches-the-helper: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-stage-diagram-matches-the-helper]
case_digest: [195d94cc4c4af4256e3e2a8ae7ccdb72e67f12700ca6ab357f4ae6af77018051]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - stage_diagrams_draw_only_the_helpers_stages_and_moves
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:167747e116803a1cbac02afa89742c17b6c352acdf6b99bbedbfd830cce4b3ae
criteria: [FLOW-18, FLOW-2]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:23Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact stage_diagrams_draw_only_the_helpers_stages_and_moves for TC-cli-stage-diagram-matches-the-helper: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:23Z. The raw log stays in the machine-local store.
