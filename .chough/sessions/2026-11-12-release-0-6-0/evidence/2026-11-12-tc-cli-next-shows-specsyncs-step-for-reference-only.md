---
kind: evidence
id: 2026-11-12-tc-cli-next-shows-specsyncs-step-for-reference-only
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_plan_after_approval_next_shows_specsyncs_step_for_reference_only for TC-cli-next-shows-specsyncs-step-for-reference-only: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 3
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
content: dcb4f087dd1af97765710e6f3754c942490fbfd0fd5d504ff31fe35cbbd50233
environment: local-macos-arm64
fingerprint: sha256:47ad7f990301240ca025adc2113c31a82a855c1f318b0a241a203168520977ea
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:55:37Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_plan_after_approval_next_shows_specsyncs_step_for_reference_only for TC-cli-next-shows-specsyncs-step-for-reference-only: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 7fb91195b8f2, observed 2026-11-12T22:55:37Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- at_plan_after_approval_next_shows_specsyncs_step_for_reference_only stdout ----
- thread 'at_plan_after_approval_next_shows_specsyncs_step_for_reference_only' (927164178) panicked at tests/next.rs:2825:5:
- assertion `left == right` failed
-   left: "  Native next action: run `specsync change check after-approval --commit`"
-  right: "  SpecSync's own next step, for reference (the step to take now is Next): run `specsync change check after-approval --commit`"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 32 filtered out; finished in 1.16s
