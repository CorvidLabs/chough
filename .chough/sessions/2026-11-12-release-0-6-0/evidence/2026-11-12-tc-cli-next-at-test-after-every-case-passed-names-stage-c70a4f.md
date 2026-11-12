---
kind: evidence
id: 2026-11-12-tc-cli-next-at-test-after-every-case-passed-names-stage-c70a4f
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
revision: e0131c098169579c13f2498db93514969f149f33
content: f90f0ac78534841119984ed8ae2779eb2acada66c1c412a9c38a7acfd38b6442
environment: local-macos-arm64
fingerprint: sha256:5a56b6cc3c7fffdc7cb7387aab10eac0b0a87a35846b41448a8e3420e4955356
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:02:11Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run for TC-cli-next-at-test-after-every-case-passed-names-stage-advance: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e0131c098169, observed 2026-11-12T23:02:11Z. The raw log stays in the machine-local store.
