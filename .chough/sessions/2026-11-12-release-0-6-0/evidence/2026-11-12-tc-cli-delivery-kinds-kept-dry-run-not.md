---
kind: evidence
id: 2026-11-12-tc-cli-delivery-kinds-kept-dry-run-not
aliases: []
claim: "Ran cargo test --locked --test reduce -- --exact delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not for TC-cli-delivery-kinds-kept-dry-run-not: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: e8bde99c60a7555434694e8ffadf1bb28fa25fda
content: 3ee3e75bcaf0a92f7bc5ca41f59bd421498d8a26342f52e2d390ef2dd4377b54
environment: local-macos-arm64
fingerprint: sha256:3e2846dddc1c53f8654e7386de8b2a65f71a13a8646acc25170bfdac417d0174
criteria: [FLOW-11, LIFE-7]
requirements: [REQ-cli-014]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:40:25Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test reduce -- --exact delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not for TC-cli-delivery-kinds-kept-dry-run-not: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision e8bde99c60a7, observed 2026-11-12T17:40:25Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not stdout ----
- thread 'delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not' (921027486) panicked at tests/reduce.rs:137:9:
- dry-run: the receipt was copied into the vault
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out; finished in 1.84s
