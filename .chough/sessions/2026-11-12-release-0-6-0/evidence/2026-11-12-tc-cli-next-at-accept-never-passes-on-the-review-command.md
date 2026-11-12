---
kind: evidence
id: 2026-11-12-tc-cli-next-at-accept-never-passes-on-the-review-command
aliases: []
claim: "Ran cargo test --locked --test next -- --exact next_at_accept_never_passes_on_specsyncs_review_command for TC-cli-next-at-accept-never-passes-on-the-review-command: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-at-accept-never-passes-on-the-review-command]
case_digest: [b749a4c42f9c473861686845e1be5eb5ac04876a213bb7fa0c27ca7e163c4066]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - next_at_accept_never_passes_on_specsyncs_review_command
exit_code: 101
tests_run: 1
duration_seconds: 4
revision: 24f9ca8d6b42d6075bd2e529272aa9904f44cb31
content: 13e29a70682a32a4fa2f14f71c908107c1f689464dfc6ea468468660b836a397
environment: local-macos-arm64
fingerprint: sha256:64a245a2471f832697343d3fd3f9ac27094299612a01358b4a72e956210fbbf0
criteria: [FLOW-13, FLOW-23]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:53:00Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_at_accept_never_passes_on_specsyncs_review_command for TC-cli-next-at-accept-never-passes-on-the-review-command: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 24f9ca8d6b42, observed 2026-11-12T21:53:00Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- next_at_accept_never_passes_on_specsyncs_review_command stdout ----
- thread 'next_at_accept_never_passes_on_specsyncs_review_command' (924890873) panicked at tests/next.rs:2225:5:
- SpecSync's review command is passed on at accept: ["Reaching done needs native state archived; it is verifying. Native next action: run `specsync change review review-change --reviewer <human>` after the PR's scoped review passes. Run `specsync change status review-change`.", "run `specsync change review review-change --reviewer <human>` after the PR's scoped review passes", "Reaching done needs native state archived; it is verifying. Native next action: run `specsync change review review-change --reviewer <human>` after the PR's scoped review passes. Run `specsync change status review-change`.", "{\"summary\":{\"id\":\"review-change\",\"state\":\"verifying\",\"next_action\":\"run `specsync change review review-change --reviewer <human>` after the PR's scoped review passes\",\"correction_valid\":true,\"approval_valid\":true},\"change\":{\"affected_specs\":[],\"no_spec_change\":true}}\n", "run `specsync change review review-change --reviewer <human>` after the PR's scoped review passes", "Objective: Ship a focused change\nStage: accept (active, iteration 1)\nNative change: review-change: verifying (read observed; approval_valid true, correction_valid true, terminal evidence unknown)\n  Native next action: run `specsync change review review-change --reviewer <human>` after the PR's scoped review passes\n  Approved deltas: no semantic deltas (no_spec_change)\nNative check: no attempt recorded\n  Whether the last native check matches this tree is unknown: SpecSync's status gave no readiness; run `specsync change check review-change` to be sure.\nNext: Waiting for a human decision: Implementation review of review-change. Do not decide it for them; continue only independent work within scope. When they decide, record their words with `choug
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 30 filtered out; finished in 2.00s
