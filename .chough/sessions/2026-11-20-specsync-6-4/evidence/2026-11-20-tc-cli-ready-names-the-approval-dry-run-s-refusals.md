---
kind: evidence
id: 2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals
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
revision: e8d2ecc43843abdac87b9c439fcbd58f991d112b
content: 8ae2b83a6c451ac9ac8c50f3ac993bc64bf2201b907353f186b31caddaab6902
environment: local-macos-arm64
fingerprint: sha256:3a80c2969a80f765ea33b45fbc795fdbc1286ba76bafbf9f48cd44966ae122d9
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:12:43Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_refusal_of_the_approval_dry_run_is_named_with_its_fix for TC-cli-ready-names-the-approval-dry-run-s-refusals: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e8d2ecc43843, observed 2026-11-20T19:12:43Z. The raw log stays in the machine-local store.
