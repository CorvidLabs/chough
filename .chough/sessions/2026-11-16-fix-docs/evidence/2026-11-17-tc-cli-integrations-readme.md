---
kind: evidence
id: 2026-11-17-tc-cli-integrations-readme
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-readme]
case_digest: [8789bd2fb23f68bf4f254a914934273ebf75ccf35a2b5a3b0bb3cd0c336c0440]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - the_readme_names_every_template_and_says_chough_has_no_runtime_dependency
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:84d6b6b4aecb7cf186d65b884705f5e51941a06799528af89ce433e0e46b00e5
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:35Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:35Z. The raw log stays in the machine-local store.
