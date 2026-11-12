---
kind: evidence
id: 2026-11-12-tc-cli-next-at-accept-while-approved-names-the-review
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_accept_while_approved_next_names_the_review_preparation_in_order for TC-cli-next-at-accept-while-approved-names-the-review-preparation: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-at-accept-while-approved-names-the-review-preparation]
case_digest: [2103832197b41ced815f01a93a6f498e0c980390e7c5b3ca851ce23164533493]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_accept_while_approved_next_names_the_review_preparation_in_order
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
content: ab5a1023f46b5eb925e66eb0451bfdf545f677a3592abbfceebad0d2f81c46c0
environment: local-macos-arm64
fingerprint: sha256:d45884605ca436fa6d8dff7a8ade8cd816085501d612276322bdfc380eb12571
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:01:45Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_accept_while_approved_next_names_the_review_preparation_in_order for TC-cli-next-at-accept-while-approved-names-the-review-preparation: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 5195bd677425, observed 2026-11-12T21:01:45Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- at_accept_while_approved_next_names_the_review_preparation_in_order stdout ----
- thread 'at_accept_while_approved_next_names_the_review_preparation_in_order' (924187441) panicked at tests/next.rs:2637:13:
- approved: Gate not open (refused:native-approved): Reaching done needs native state archived; it is approved.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 29 filtered out; finished in 1.20s
