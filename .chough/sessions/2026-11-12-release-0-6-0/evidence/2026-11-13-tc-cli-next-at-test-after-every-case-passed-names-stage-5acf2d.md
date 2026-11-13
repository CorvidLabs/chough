---
kind: evidence
id: 2026-11-13-tc-cli-next-at-test-after-every-case-passed-names-stage-5acf2d
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:6cf4c5959249c4546cd8710b98a03f07324884ae2a5cca90c5bcdee1c44c7129
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:50:06Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_test_after_every_case_passed_next_names_stage_advance_and_no_superseded_run for TC-cli-next-at-test-after-every-case-passed-names-stage-advance: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:50:06Z. The raw log stays in the machine-local store.
