---
kind: evidence
id: 2026-11-17-tc-cli-integrations-skill-file-exists-after-init
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-skill-file-exists-after-init]
case_digest: [23f650f2098c747e712edc8684d3bda735c787475ceb29c5d44401c9fe1ad543]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - each_templates_skill_file_exists_after_its_own_chough_init
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:b48e00fe1c46eaf79a61986dc4ed3ef41dab416163e1d0e9dc6ae145a2438c35
criteria: [LIFE-22, LIFE-24]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:36Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:36Z. The raw log stays in the machine-local store.
