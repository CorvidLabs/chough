---
kind: evidence
id: 2026-11-23-tc-cli-grouped-gates-direct-write-history
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact grouped_close_records_prior_native_writes_once_and_before_either_close for TC-cli-grouped-gates-direct-write-history: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-grouped-gates-direct-write-history]
case_digest: [a9d6ac9ea6186f4c81ddca42fa6e964568175dd460550769ee716373b69d6299]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - grouped_close_records_prior_native_writes_once_and_before_either_close
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: b7fbb1c9d8e3c3d2c39513490920c5a37e5c093f
content: 564cbdf60b148772ee9546ddee6008c971ca697da6924c70263090540a883bdf
environment: local-macos-arm64
fingerprint: sha256:924eabb4060f67f012ba93bc270c111429052faf15683b09ba32da46da0acb9a
criteria: [FLOW-20]
requirements: [REQ-cli-061, REQ-cli-060]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-23T15:36:13Z
actor:
  - terminal: t14v
---
**Ran cargo test --locked --test gates -- --exact grouped_close_records_prior_native_writes_once_and_before_either_close for TC-cli-grouped-gates-direct-write-history: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b7fbb1c9d8e3, observed 2026-11-23T15:36:13Z. The raw log stays in the machine-local store.
