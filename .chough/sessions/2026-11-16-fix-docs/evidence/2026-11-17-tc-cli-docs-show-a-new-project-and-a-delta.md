---
kind: evidence
id: 2026-11-17-tc-cli-docs-show-a-new-project-and-a-delta
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact a_new_project_path_and_the_delta_format_are_documented for TC-cli-docs-show-a-new-project-and-a-delta: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-show-a-new-project-and-a-delta]
case_digest: [893a075c612c13f8c8075047dd64d476c55bb967f8eded8fd8205aee7c193b9a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - a_new_project_path_and_the_delta_format_are_documented
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:7df3df96b871c820b72281c9f3527e6519b0ffbae5a4140ef72206cd3945e27b
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:29Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact a_new_project_path_and_the_delta_format_are_documented for TC-cli-docs-show-a-new-project-and-a-delta: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:29Z. The raw log stays in the machine-local store.
