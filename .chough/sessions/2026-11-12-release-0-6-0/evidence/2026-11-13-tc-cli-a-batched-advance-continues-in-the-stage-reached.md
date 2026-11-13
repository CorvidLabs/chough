---
kind: evidence
id: 2026-11-13-tc-cli-a-batched-advance-continues-in-the-stage-reached
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact a_batched_advance_leaves_continue_for_the_stage_it_reaches for TC-cli-a-batched-advance-continues-in-the-stage-reached: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-batched-advance-continues-in-the-stage-reached]
case_digest: [475a339eeaebbcd4ec9d9f6a2c0d174e6f4dec372b92119b6bd3c3843108616e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - a_batched_advance_leaves_continue_for_the_stage_it_reaches
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 11a54de9b19220c0ffe73a263de91f652ef11278
content: bef9ac8f6df535e64640e1b0f177c0fcbc0567df9bdd6172a976be132efb7eaf
environment: local-macos-arm64
fingerprint: sha256:5e52ac96ead742a57b283e8bc9b85cfb9e3d6564b4662241c34129349c604e13
criteria: [FLOW-13, FLOW-2]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:04:44Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_batched_advance_leaves_continue_for_the_stage_it_reaches for TC-cli-a-batched-advance-continues-in-the-stage-reached: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 11a54de9b192, observed 2026-11-13T00:04:44Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_batched_advance_leaves_continue_for_the_stage_it_reaches stdout ----
- thread 'a_batched_advance_leaves_continue_for_the_stage_it_reaches' (928845797) panicked at tests/humans.rs:810:5:
- assertion `left == right` failed
-   left: "Continue build"
-  right: "Continue test"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out; finished in 1.51s
