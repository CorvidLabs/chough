---
kind: evidence
id: 2026-11-12-tc-cli-next-at-test-after-every-case-passed-names-stage-4123c0
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:219019929c7242c94ad9e13d5088efc151117a6beec89c333c005af7962e8a20
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:25:53Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run for TC-cli-next-at-test-after-every-case-passed-names-stage-advance: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:25:53Z. The raw log stays in the machine-local store.
