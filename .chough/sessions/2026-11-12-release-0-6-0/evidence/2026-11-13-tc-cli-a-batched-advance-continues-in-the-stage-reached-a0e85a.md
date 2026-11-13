---
kind: evidence
id: 2026-11-13-tc-cli-a-batched-advance-continues-in-the-stage-reached-a0e85a
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:ce156a79be378b127ae8a62c731621b03de827eaa998cdf3ff916e0e8bbf20b7
criteria: [FLOW-13, FLOW-2]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:22:03Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_batched_advance_leaves_continue_for_the_stage_it_reaches for TC-cli-a-batched-advance-continues-in-the-stage-reached: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:22:03Z. The raw log stays in the machine-local store.
