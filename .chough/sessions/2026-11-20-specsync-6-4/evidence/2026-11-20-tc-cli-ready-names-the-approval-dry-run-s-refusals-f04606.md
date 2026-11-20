---
kind: evidence
id: 2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals-f04606
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
revision: 6571ed05beecf7230bf1ca054a511498a03a3569
content: 3b0af0eee772b0e16f075859206fe29bfddd0832bdb3bdb8464467cea6477800
environment: local-macos-arm64
fingerprint: sha256:9964f9e601b2876811dc40dba1cff347fb3dee82a3eb91dc464b610df52f94fb
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T22:22:58Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_refusal_of_the_approval_dry_run_is_named_with_its_fix for TC-cli-ready-names-the-approval-dry-run-s-refusals: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6571ed05beec, observed 2026-11-20T22:22:58Z. The raw log stays in the machine-local store.
