---
kind: evidence
id: 2026-11-23-tc-cli-grouped-gates-native-preflight
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact mixed_group_checks_full_capacity_before_running_the_native_step for TC-cli-grouped-gates-native-preflight: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-grouped-gates-native-preflight]
case_digest: [7df608588e2eaa06d6f36ba91712b726c0ea9189fcccffe75be1170f72e472ed]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - mixed_group_checks_full_capacity_before_running_the_native_step
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: b7fbb1c9d8e3c3d2c39513490920c5a37e5c093f
content: 564cbdf60b148772ee9546ddee6008c971ca697da6924c70263090540a883bdf
environment: local-macos-arm64
fingerprint: sha256:03addacd3061d3527b0a8db069597fb0e95589ebc2c1a80f31f5a31e2f6ca263
criteria: [FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-23T15:35:52Z
actor:
  - terminal: t14v
---
**Ran cargo test --locked --test gates -- --exact mixed_group_checks_full_capacity_before_running_the_native_step for TC-cli-grouped-gates-native-preflight: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b7fbb1c9d8e3, observed 2026-11-23T15:35:52Z. The raw log stays in the machine-local store.
