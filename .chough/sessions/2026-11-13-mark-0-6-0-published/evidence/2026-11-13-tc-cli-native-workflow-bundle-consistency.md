---
kind: evidence
id: 2026-11-13-tc-cli-native-workflow-bundle-consistency
aliases: []
claim: "Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-native-workflow-bundle-consistency]
case_digest: [519ac533c4882bb4da71e97a9cae2d9ec1c2a93425652f94f90ec555939a8e72]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - workflow_authority
  - "--"
  - "--exact"
  - native_writer_examples_and_installed_workflows_are_consistent
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 443c63ab65b747f47248ad2166f8fb299da94a41
content: b056982fd067aff750b535081ae04bbaf4da1742935833f4a5a8559c6a739214
environment: local-macos-arm64
fingerprint: sha256:6973dee0413442e569a2efe11b3460c69e26a2849389082a2b475bd599fad134
criteria: [FLOW-5, FLOW-13, LIFE-4, LIFE-6, LIFE-7]
requirements: [REQ-cli-029]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:55:07Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 443c63ab65b7, observed 2026-11-13T01:55:07Z. The raw log stays in the machine-local store.
