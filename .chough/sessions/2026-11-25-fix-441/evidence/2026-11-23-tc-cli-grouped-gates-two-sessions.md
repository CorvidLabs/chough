---
kind: evidence
id: 2026-11-23-tc-cli-grouped-gates-two-sessions
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_group_spanning_two_sessions_leaves_both_records_pending for TC-cli-grouped-gates-two-sessions: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-grouped-gates-two-sessions]
case_digest: [6a48ee10d19429f3e48c73b7b8042bb93f06489efc4a604c09cf37ab3fd6c578]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_group_spanning_two_sessions_leaves_both_records_pending
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: b7fbb1c9d8e3c3d2c39513490920c5a37e5c093f
content: 564cbdf60b148772ee9546ddee6008c971ca697da6924c70263090540a883bdf
environment: local-macos-arm64
fingerprint: sha256:a356d8595b678fd094046175c6c5eef1edd85536b7fd38a201eecf512394e925
criteria: [FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-23T15:35:47Z
actor:
  - terminal: t14v
---
**Ran cargo test --locked --test gates -- --exact a_group_spanning_two_sessions_leaves_both_records_pending for TC-cli-grouped-gates-two-sessions: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b7fbb1c9d8e3, observed 2026-11-23T15:35:47Z. The raw log stays in the machine-local store.
