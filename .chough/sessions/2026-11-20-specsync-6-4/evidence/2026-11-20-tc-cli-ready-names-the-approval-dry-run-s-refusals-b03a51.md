---
kind: evidence
id: 2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals-b03a51
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_refusal_of_the_approval_dry_run_is_named_with_its_fix for TC-cli-ready-names-the-approval-dry-run-s-refusals: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ready-names-the-approval-dry-run-s-refusals]
case_digest: [20028824b6cb23e20b2a092fa81dc20bf1aa8cf26a2458438b6593f5e66c4610]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_refusal_of_the_approval_dry_run_is_named_with_its_fix
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: e5571fd735bf8c4c575ef11a69a63a8e14915bad
content: b4f463f5579645db964f839158eda5550438456bd20e553a4249913a565ca384
environment: local-macos-arm64
fingerprint: sha256:4d660e8c0bdd8fa1c2810e8bde65c9553c6b198b7f812389012405d8206961fa
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:40:21Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_refusal_of_the_approval_dry_run_is_named_with_its_fix for TC-cli-ready-names-the-approval-dry-run-s-refusals: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e5571fd735bf, observed 2026-11-20T19:40:21Z. The raw log stays in the machine-local store.
