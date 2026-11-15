---
kind: evidence
id: 2026-11-15-tc-cli-native-workflow-bundle-consistency-9d9153
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
revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
environment: local-macos-arm64
fingerprint: sha256:6a2630ff8aad00762fc16c8573a5129433f5253fa75dc4337fd2462ad242ee2f
criteria: [FLOW-5, FLOW-13, LIFE-4, LIFE-6, LIFE-7]
requirements: [REQ-cli-029]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T21:09:06Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8b5bd53cab53, observed 2026-11-15T21:09:06Z. The raw log stays in the machine-local store.
