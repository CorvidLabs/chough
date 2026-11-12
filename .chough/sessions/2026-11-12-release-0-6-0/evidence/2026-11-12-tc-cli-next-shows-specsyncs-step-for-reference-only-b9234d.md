---
kind: evidence
id: 2026-11-12-tc-cli-next-shows-specsyncs-step-for-reference-only-b9234d
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:fd84d5e7fa445ea1119005a6ee253b5099d153fb1efea6ab64fd578b6d6159f5
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:25:28Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_plan_after_approval_next_shows_specsyncs_step_for_reference_only for TC-cli-next-shows-specsyncs-step-for-reference-only: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:25:28Z. The raw log stays in the machine-local store.
