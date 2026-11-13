---
kind: evidence
id: 2026-11-13-tc-cli-accept-next-reruns-no-retired-case-ba5a0a
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
duration_seconds: 3
revision: 17d60d515ce277feac2b02ea8e828c98f5f44bb8
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:360aef6eaa05c9cfe8ba89518591e20517b617c05d41989c90888f713897ba74
criteria: [FLOW-13, FLOW-23]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:32:24Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_accept_next_names_no_rerun_of_a_retired_case for TC-cli-accept-next-reruns-no-retired-case: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17d60d515ce2, observed 2026-11-13T00:32:24Z. The raw log stays in the machine-local store.
