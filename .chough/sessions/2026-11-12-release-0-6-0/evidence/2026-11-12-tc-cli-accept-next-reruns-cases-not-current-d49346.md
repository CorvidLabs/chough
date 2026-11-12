---
kind: evidence
id: 2026-11-12-tc-cli-accept-next-reruns-cases-not-current-d49346
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
revision: 648ebb942879e453a90874b5e0ba3ea341515301
content: 1e3d4b1dc96fcd222b7b4c3cb1e922d5c0b93aa47190bb61424d2d99c9956761
environment: local-macos-arm64
fingerprint: sha256:24163b993c5a51da79f98a188366123a76fc1641b3fdd458563671ebc3fbb99f
criteria: [FLOW-2, LIFE-7]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:55:08Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_accept_next_names_rerunning_a_case_that_is_not_current_before_the_review_gate for TC-cli-accept-next-reruns-cases-not-current: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 648ebb942879, observed 2026-11-12T23:55:08Z. The raw log stays in the machine-local store.
