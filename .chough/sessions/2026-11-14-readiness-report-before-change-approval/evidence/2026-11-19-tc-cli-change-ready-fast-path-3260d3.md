---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-fast-path-3260d3
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
revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
environment: local-macos-arm64
fingerprint: sha256:0841d35ca782bb9b9729b720497617ea1d00741b27b7c478862aadf9ab5330a5
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:14:35Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5b9c1b4ad6f2, observed 2026-11-19T15:14:35Z. The raw log stays in the machine-local store.
