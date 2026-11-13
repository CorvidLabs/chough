---
kind: evidence
id: 2026-11-13-tc-cli-a-batched-advance-continues-in-the-stage-reached-b5f4d8
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
duration_seconds: 2
revision: e1ada30f9b26806a0af33da57027616ba4e2cb5b
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:e3403a181fd0b001139406b13b76f5aac6799b306ccead7c2d087dd08baaf1f3
criteria: [FLOW-13, FLOW-2]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:07:39Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_batched_advance_leaves_continue_for_the_stage_it_reaches for TC-cli-a-batched-advance-continues-in-the-stage-reached: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e1ada30f9b26, observed 2026-11-13T00:07:39Z. The raw log stays in the machine-local store.
