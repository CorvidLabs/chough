---
kind: evidence
id: 2026-11-17-tc-cli-native-workflow-bundle-consistency
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
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:470018b43bed0b819051ade71ec315478609edffa048e73320c58b6e56f9ecb3
criteria: [FLOW-5, FLOW-13, LIFE-4, LIFE-6, LIFE-7]
requirements: [REQ-cli-029]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:07Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:07Z. The raw log stays in the machine-local store.
