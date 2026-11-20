---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-draft-fixes-24ebfb
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-draft-fixes]
case_digest: [cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: e5571fd735bf8c4c575ef11a69a63a8e14915bad
content: b4f463f5579645db964f839158eda5550438456bd20e553a4249913a565ca384
environment: local-macos-arm64
fingerprint: sha256:711b9e42d3f7d0bdbcae4f84edf42e667ca81cad983c5be73ef6fe7e4248c1a6
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:40:40Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e5571fd735bf, observed 2026-11-20T19:40:40Z. The raw log stays in the machine-local store.
