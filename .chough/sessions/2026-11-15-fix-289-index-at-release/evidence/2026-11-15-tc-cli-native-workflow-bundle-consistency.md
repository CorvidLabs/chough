---
kind: evidence
id: 2026-11-15-tc-cli-native-workflow-bundle-consistency
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
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:9c731137b83c3ab5c46f176c15f7626bed358a76edb92a071be574ffc4e37f15
criteria: [FLOW-5, FLOW-13, LIFE-4, LIFE-6, LIFE-7]
requirements: [REQ-cli-029]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:10Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:10Z. The raw log stays in the machine-local store.
