---
kind: evidence
id: 2026-11-23-tc-cli-grouped-gates-revised-scope
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_matching_historical_relay_does_not_block_a_new_scope_approval for TC-cli-grouped-gates-revised-scope: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-grouped-gates-revised-scope]
case_digest: [e778c7b16eaae90c1d2d5a55b864951ebe378f312dcfc431300a0ede2f5e238a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_matching_historical_relay_does_not_block_a_new_scope_approval
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: b7fbb1c9d8e3c3d2c39513490920c5a37e5c093f
content: 564cbdf60b148772ee9546ddee6008c971ca697da6924c70263090540a883bdf
environment: local-macos-arm64
fingerprint: sha256:4a2ca568151d3e6057eb61163b9e217b536e5fbd019e159c887224dfa6c9bd88
criteria: [FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-23T15:36:08Z
actor:
  - terminal: t14v
---
**Ran cargo test --locked --test gates -- --exact a_matching_historical_relay_does_not_block_a_new_scope_approval for TC-cli-grouped-gates-revised-scope: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b7fbb1c9d8e3, observed 2026-11-23T15:36:08Z. The raw log stays in the machine-local store.
