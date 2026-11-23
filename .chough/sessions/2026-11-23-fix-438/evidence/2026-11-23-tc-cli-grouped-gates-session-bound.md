---
kind: evidence
id: 2026-11-23-tc-cli-grouped-gates-session-bound
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_grouped_reply_is_atomic_at_the_real_session_byte_cap_and_can_retry for TC-cli-grouped-gates-session-bound: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-grouped-gates-session-bound]
case_digest: [f88f051c120ec77c232383b0fda3c605cdb46c6257f26033c1a13a0a110f517e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_grouped_reply_is_atomic_at_the_real_session_byte_cap_and_can_retry
exit_code: 0
tests_run: 1
duration_seconds: 32
revision: b7fbb1c9d8e3c3d2c39513490920c5a37e5c093f
content: 564cbdf60b148772ee9546ddee6008c971ca697da6924c70263090540a883bdf
environment: local-macos-arm64
fingerprint: sha256:b7e9cd553c67771943fd217598936269f3abdef7942d4757359ee1678707c8e7
criteria: [FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-23T15:35:44Z
actor:
  - terminal: t14v
---
**Ran cargo test --locked --test gates -- --exact a_grouped_reply_is_atomic_at_the_real_session_byte_cap_and_can_retry for TC-cli-grouped-gates-session-bound: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b7fbb1c9d8e3, observed 2026-11-23T15:35:44Z. The raw log stays in the machine-local store.
