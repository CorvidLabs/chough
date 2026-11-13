---
kind: evidence
id: 2026-11-13-tc-cli-next-shows-specsyncs-step-for-reference-only-e7b125
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_plan_after_approval_next_shows_specsyncs_step_for_reference_only for TC-cli-next-shows-specsyncs-step-for-reference-only: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-shows-specsyncs-step-for-reference-only]
case_digest: [f81a96a72f967a2d1c00a5be3dd97a66c21771fb203ab62af5ef87f453628867]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_plan_after_approval_next_shows_specsyncs_step_for_reference_only
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:1c8e278ae836507b80a6278a534ab2fbb61e5b0c759cb2c442c3117612651ea7
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:16:39Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_plan_after_approval_next_shows_specsyncs_step_for_reference_only for TC-cli-next-shows-specsyncs-step-for-reference-only: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:16:39Z. The raw log stays in the machine-local store.
