---
kind: evidence
id: 2026-11-13-tc-cli-accept-next-reruns-cases-not-current-65b151
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:985c8a9f053be15fa3fefb75fd51951ece53377d38eaa8b37c436ecfd946edb4
criteria: [FLOW-2, LIFE-7]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:53:26Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_accept_next_names_rerunning_a_case_that_is_not_current_before_the_review_gate for TC-cli-accept-next-reruns-cases-not-current: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:53:26Z. The raw log stays in the machine-local store.
