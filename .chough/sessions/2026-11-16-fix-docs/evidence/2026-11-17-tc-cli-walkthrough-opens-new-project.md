---
kind: evidence
id: 2026-11-17-tc-cli-walkthrough-opens-new-project
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact the_walkthrough_begins_with_the_new_project_setup_path for TC-cli-walkthrough-opens-new-project: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-walkthrough-opens-new-project]
case_digest: [fcc0af3572fabbb389e19012fcaad1d387c15c061fd4490d08abcb4614803e1a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - the_walkthrough_begins_with_the_new_project_setup_path
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:9880111b5308f055370dab6b9375ffd07c86acda27cb5782e00ac3fda91e1e86
criteria: [LIFE-9, LIFE-6]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:38Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test cli -- --exact the_walkthrough_begins_with_the_new_project_setup_path for TC-cli-walkthrough-opens-new-project: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:38Z. The raw log stays in the machine-local store.
