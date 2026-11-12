---
kind: evidence
id: 2026-11-12-tc-cli-delivery-kinds-kept-dry-run-not-5f607b
aliases: []
claim: "Ran cargo test --locked --test reduce -- --exact delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not for TC-cli-delivery-kinds-kept-dry-run-not: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-delivery-kinds-kept-dry-run-not]
case_digest: [f75fe061aa795f379c4f0a6aefcc1d7dd2f81cceacab5b27d81200420734b1eb]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - reduce
  - "--"
  - "--exact"
  - delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: ac0cafef56b124ae2047f3f9c9798fa216c40551
content: b7604bbec7da005916f03fc30fbfe1cb2327c04a826f4446bfaa33fc39551f81
environment: local-macos-arm64
fingerprint: sha256:632cb8af8c717cf4020f5d2f55d2f7d213615e5b71391a9120b852587e4ef1ea
criteria: [FLOW-11, LIFE-7]
requirements: [REQ-cli-014]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:41:37Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test reduce -- --exact delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not for TC-cli-delivery-kinds-kept-dry-run-not: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ac0cafef56b1, observed 2026-11-12T17:41:37Z. The raw log stays in the machine-local store.
