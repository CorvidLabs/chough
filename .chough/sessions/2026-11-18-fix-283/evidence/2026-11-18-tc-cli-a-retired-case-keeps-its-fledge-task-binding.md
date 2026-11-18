---
kind: evidence
id: 2026-11-18-tc-cli-a-retired-case-keeps-its-fledge-task-binding
aliases: []
claim: "Ran cargo test --locked --test cases -- --exact a_retired_case_keeps_a_binding_to_a_removed_fledge_task for TC-cli-a-retired-case-keeps-its-fledge-task-binding: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-retired-case-keeps-its-fledge-task-binding]
case_digest: [27cf0c0e564bc8c9a17a3e077fdd6b470acf0fabbcfe7edfad534dee01965587]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cases
  - "--"
  - "--exact"
  - a_retired_case_keeps_a_binding_to_a_removed_fledge_task
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
content: 353ae1dd4fd50667fd10c9628aae0071737ffaf880fe60b2c81971150439a84d
environment: local-macos-arm64
fingerprint: sha256:031c7e2c1ef83a24626635ddcb1cff0ce2f7dfa37c049c946f306e05ab2643b9
criteria: [LIFE-8]
requirements: [REQ-cli-009]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:15:50Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cases -- --exact a_retired_case_keeps_a_binding_to_a_removed_fledge_task for TC-cli-a-retired-case-keeps-its-fledge-task-binding: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e47b4cfaabe2, observed 2026-11-18T20:15:50Z. The raw log stays in the machine-local store.
