---
kind: evidence
id: 2026-11-23-tc-cli-grouped-gates-native-recovery
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact retry_after_native_success_and_local_write_failure_does_not_repeat_the_native_step for TC-cli-grouped-gates-native-recovery: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-grouped-gates-native-recovery]
case_digest: [d6070635a5b8e427600f974de28c9fea144ac9bd034c5de65113b7f3c064b6b3]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - retry_after_native_success_and_local_write_failure_does_not_repeat_the_native_step
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: b7fbb1c9d8e3c3d2c39513490920c5a37e5c093f
content: 564cbdf60b148772ee9546ddee6008c971ca697da6924c70263090540a883bdf
environment: local-macos-arm64
fingerprint: sha256:03224338ed67c92f04feb18b9a7efa2aab29079aaf87f762c397163c1c8d8709
criteria: [FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-23T15:36:03Z
actor:
  - terminal: t14v
---
**Ran cargo test --locked --test gates -- --exact retry_after_native_success_and_local_write_failure_does_not_repeat_the_native_step for TC-cli-grouped-gates-native-recovery: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b7fbb1c9d8e3, observed 2026-11-23T15:36:03Z. The raw log stays in the machine-local store.
