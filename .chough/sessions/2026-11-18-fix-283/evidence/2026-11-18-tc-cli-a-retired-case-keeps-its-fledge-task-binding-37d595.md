---
kind: evidence
id: 2026-11-18-tc-cli-a-retired-case-keeps-its-fledge-task-binding-37d595
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
revision: a07b42d6cc1c27e6621a87c0481d136c80aeaa09
content: 321b1d3f70699ec4feae61b1b4fe896d20a0162452dc97199e252db638dc0676
environment: local-macos-arm64
fingerprint: sha256:064276f7e9f349d44c83cd843f24acb7c8043c511f372fc3228bf2475a284d94
criteria: [LIFE-8]
requirements: [REQ-cli-009]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:13:40Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cases -- --exact a_retired_case_keeps_a_binding_to_a_removed_fledge_task for TC-cli-a-retired-case-keeps-its-fledge-task-binding: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a07b42d6cc1c, observed 2026-11-18T23:13:40Z. The raw log stays in the machine-local store.
