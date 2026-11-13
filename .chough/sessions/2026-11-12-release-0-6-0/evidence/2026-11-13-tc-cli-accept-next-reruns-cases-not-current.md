---
kind: evidence
id: 2026-11-13-tc-cli-accept-next-reruns-cases-not-current
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_accept_next_names_rerunning_a_case_that_is_not_current_before_the_review_gate for TC-cli-accept-next-reruns-cases-not-current: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-accept-next-reruns-cases-not-current]
case_digest: [45b18fb29f2e09ae23d6f4067c0435bc1c9a8f21d373a8f6984c2b442d24b767]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_accept_next_names_rerunning_a_case_that_is_not_current_before_the_review_gate
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:d5209a2811cee28d4dd6cd5a00339eb9e031e1a876cb2917a387306c45caa182
criteria: [FLOW-2, LIFE-7]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:26:08Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_accept_next_names_rerunning_a_case_that_is_not_current_before_the_review_gate for TC-cli-accept-next-reruns-cases-not-current: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:26:08Z. The raw log stays in the machine-local store.
