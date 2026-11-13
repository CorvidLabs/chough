---
kind: evidence
id: 2026-11-13-tc-cli-next-at-test-after-every-case-passed-names-stage
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run for TC-cli-next-at-test-after-every-case-passed-names-stage-advance: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-at-test-after-every-case-passed-names-stage-advance]
case_digest: [b69ef226634ba10d38ffb9797057a4670a0a4ea6313c7090c55d9be7f79f4a1f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:4f021c12feee994aa25cda2129201b41595ccae666c0c9881880e15fbafa6741
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:23:30Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run for TC-cli-next-at-test-after-every-case-passed-names-stage-advance: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:23:30Z. The raw log stays in the machine-local store.
