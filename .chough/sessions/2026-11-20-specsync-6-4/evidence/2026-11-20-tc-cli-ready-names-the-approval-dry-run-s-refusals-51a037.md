---
kind: evidence
id: 2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals-51a037
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
revision: 600a34b4a7212970c8fa4d39958da116e4f2b9b8
content: f288da34b6490daffc348266c23831006988e58d530541024eaaa57cc05f99ec
environment: local-macos-arm64
fingerprint: sha256:d177cd235501743aedc7deb53a325930671951bb2b94bdc4da9baab3129f26ee
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T23:41:27Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_refusal_of_the_approval_dry_run_is_named_with_its_fix for TC-cli-ready-names-the-approval-dry-run-s-refusals: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 600a34b4a721, observed 2026-11-20T23:41:27Z. The raw log stays in the machine-local store.
