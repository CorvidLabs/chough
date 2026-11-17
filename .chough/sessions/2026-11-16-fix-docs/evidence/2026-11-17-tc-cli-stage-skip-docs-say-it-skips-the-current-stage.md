---
kind: evidence
id: 2026-11-17-tc-cli-stage-skip-docs-say-it-skips-the-current-stage
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design for TC-cli-stage-skip-docs-say-it-skips-the-current-stage: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-stage-skip-docs-say-it-skips-the-current-stage]
case_digest: [3d6f5c15e943a0c26a1fb5d37e5be324a5a58f319890b5879729edb6ff45d13a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:f80686a3b08eb1d27fe8b02ab18719be5587cd9d4d0ee64ac3338e5cd5cca44d
criteria: [FLOW-2]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:32Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design for TC-cli-stage-skip-docs-say-it-skips-the-current-stage: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:32Z. The raw log stays in the machine-local store.
