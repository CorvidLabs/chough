---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-fast-path-72c3c8
aliases: []
claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-fast-path]
case_digest: [82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - package
  - "--"
  - "--exact"
  - the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e42c96fec77c56feff65256be9e1d5deb1f34742
content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
environment: local-macos-arm64
fingerprint: sha256:325a356fb51aca5a23a4e7deb133e54f3290f08024af8d74e356df9326885915
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:12:30Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e42c96fec77c, observed 2026-11-19T04:12:30Z. The raw log stays in the machine-local store.
