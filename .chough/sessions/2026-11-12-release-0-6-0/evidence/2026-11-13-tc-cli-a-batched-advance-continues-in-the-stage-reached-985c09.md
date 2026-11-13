---
kind: evidence
id: 2026-11-13-tc-cli-a-batched-advance-continues-in-the-stage-reached-985c09
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact a_batched_advance_leaves_continue_for_the_stage_it_reaches for TC-cli-a-batched-advance-continues-in-the-stage-reached: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:a10d68b4a303fc23be0e233ccf83f05afce0312bf0a09a554fc7cd570aea46c5
criteria: [FLOW-13, FLOW-2]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:26:31Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_batched_advance_leaves_continue_for_the_stage_it_reaches for TC-cli-a-batched-advance-continues-in-the-stage-reached: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:26:31Z. The raw log stays in the machine-local store.
