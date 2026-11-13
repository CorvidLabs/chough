---
kind: evidence
id: 2026-11-13-tc-cli-next-shows-specsyncs-step-for-reference-only
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:396b43ab6adc22e4213f18fa5476dcff89c103fd33e90fae785d072cd3234d58
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:22:57Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_plan_after_approval_next_shows_specsyncs_step_for_reference_only for TC-cli-next-shows-specsyncs-step-for-reference-only: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:22:57Z. The raw log stays in the machine-local store.
