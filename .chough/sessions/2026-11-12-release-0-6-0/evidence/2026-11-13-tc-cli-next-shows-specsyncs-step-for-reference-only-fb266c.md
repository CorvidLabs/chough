---
kind: evidence
id: 2026-11-13-tc-cli-next-shows-specsyncs-step-for-reference-only-fb266c
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:001b05257ac0a0352b82953cce6526177a9c1171aa2b45abb2ffd08b8b9aa738
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:49:23Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_plan_after_approval_next_shows_specsyncs_step_for_reference_only for TC-cli-next-shows-specsyncs-step-for-reference-only: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:49:23Z. The raw log stays in the machine-local store.
