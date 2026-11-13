---
kind: evidence
id: 2026-11-13-tc-cli-accept-next-reruns-no-retired-case-b7691b
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:654b428d814b5d36dd41df3d54b54d49860b7acdcedaf06af684229fe0a4f7cd
criteria: [FLOW-13, FLOW-23]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:22:21Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_accept_next_names_no_rerun_of_a_retired_case for TC-cli-accept-next-reruns-no-retired-case: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:22:21Z. The raw log stays in the machine-local store.
