---
kind: evidence
id: 2026-11-13-tc-cli-accept-next-reruns-no-retired-case-f2cc98
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_accept_next_names_no_rerun_of_a_retired_case for TC-cli-accept-next-reruns-no-retired-case: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-accept-next-reruns-no-retired-case]
case_digest: [033ba8d0f10d1b171066bd46228fbb171623e904567f76602dc2be2b6689d3c2]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_accept_next_names_no_rerun_of_a_retired_case
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:6ed0fd57e27723edbf53505c0b7308ec545126fcc2a45439d166f76d323e56f1
criteria: [FLOW-13, FLOW-23]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:54:11Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_accept_next_names_no_rerun_of_a_retired_case for TC-cli-accept-next-reruns-no-retired-case: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:54:11Z. The raw log stays in the machine-local store.
