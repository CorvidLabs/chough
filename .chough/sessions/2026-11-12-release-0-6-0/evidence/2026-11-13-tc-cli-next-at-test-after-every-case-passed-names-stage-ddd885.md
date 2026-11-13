---
kind: evidence
id: 2026-11-13-tc-cli-next-at-test-after-every-case-passed-names-stage-ddd885
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:25b5fed0c38da7ecd88d257e7a72befe05c2826c6ec2cf38c8de484ae4b95cc4
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:17:32Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run for TC-cli-next-at-test-after-every-case-passed-names-stage-advance: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:17:32Z. The raw log stays in the machine-local store.
