---
kind: evidence
id: 2026-11-23-tc-cli-grouped-gates-native-refusal
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact native_refusal_keeps_the_mixed_group_pending_and_a_safe_retry_closes_both for TC-cli-grouped-gates-native-refusal: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-grouped-gates-native-refusal]
case_digest: [8845b2661b27080bbc0774c7525886934481fa45e33aedff159bb5e890704c90]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - native_refusal_keeps_the_mixed_group_pending_and_a_safe_retry_closes_both
exit_code: 0
tests_run: 1
duration_seconds: 4
revision: b7fbb1c9d8e3c3d2c39513490920c5a37e5c093f
content: 564cbdf60b148772ee9546ddee6008c971ca697da6924c70263090540a883bdf
environment: local-macos-arm64
fingerprint: sha256:277ac49031bd21cca3e892c322ecfe893491c009132ee25bcfe40cc5af429638
criteria: [FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-23T15:35:58Z
actor:
  - terminal: t14v
---
**Ran cargo test --locked --test gates -- --exact native_refusal_keeps_the_mixed_group_pending_and_a_safe_retry_closes_both for TC-cli-grouped-gates-native-refusal: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b7fbb1c9d8e3, observed 2026-11-23T15:35:58Z. The raw log stays in the machine-local store.
