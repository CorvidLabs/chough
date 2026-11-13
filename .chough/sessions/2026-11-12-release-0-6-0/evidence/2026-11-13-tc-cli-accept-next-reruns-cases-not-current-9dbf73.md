---
kind: evidence
id: 2026-11-13-tc-cli-accept-next-reruns-cases-not-current-9dbf73
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:29b69d0bfe45a5364c0a7a499ac758b1e6cd373a3f99ff28d7016be052d78658
criteria: [FLOW-2, LIFE-7]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:21:29Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_accept_next_names_rerunning_a_case_that_is_not_current_before_the_review_gate for TC-cli-accept-next-reruns-cases-not-current: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:21:29Z. The raw log stays in the machine-local store.
