---
kind: evidence
id: 2026-11-12-tc-cli-docs-show-a-new-project-and-a-delta-342bc4
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
revision: f2635147c748d65c715c74b8ce2016d3b5ddf743
content: a8afa668621f592636850caf105e70821a49cf99455387d151b453ea52af955c
environment: local-macos-arm64
fingerprint: sha256:213885903600a504aadf508681df02722b1c44b1786b3f6853fe0cf32ecccbaf
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:32:42Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact a_new_project_path_and_the_delta_format_are_documented for TC-cli-docs-show-a-new-project-and-a-delta: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f2635147c748, observed 2026-11-12T17:32:42Z. The raw log stays in the machine-local store.
