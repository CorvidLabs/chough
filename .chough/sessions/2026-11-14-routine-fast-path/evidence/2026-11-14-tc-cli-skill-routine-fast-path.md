---
kind: evidence
id: 2026-11-14-tc-cli-skill-routine-fast-path
aliases: []
claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-skill-routine-fast-path: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-skill-routine-fast-path]
case_digest: [e62cde548f87528c33e0efbbdba70c250a0b8084297df78bb645445f5c909b22]
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
revision: 94af1a8a87582e6ba23acb16bcf400fc7f2a06d7
content: 85b5101049bc36f93e4aed36ee8f88857e43de5e9a4a08f3f78d57c1df434d3b
environment: local-macos-arm64
fingerprint: sha256:b033f776afcc0aa468dc702a451524bf80aeca3fee06a22338bd0920f979b5d7
criteria: [LIFE-22]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-14T15:59:48Z
actor:
  - agent: claude-code
---
**Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-skill-routine-fast-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 94af1a8a8758, observed 2026-11-14T15:59:48Z. The raw log stays in the machine-local store.
