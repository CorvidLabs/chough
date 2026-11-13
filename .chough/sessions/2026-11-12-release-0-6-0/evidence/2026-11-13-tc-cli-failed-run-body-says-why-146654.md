---
kind: evidence
id: 2026-11-13-tc-cli-failed-run-body-says-why-146654
aliases: []
claim: "Ran cargo test --locked --test cases -- --exact a_failed_runs_body_says_the_expected_output_was_missing_before_its_key_lines for TC-cli-failed-run-body-says-why: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-failed-run-body-says-why]
case_digest: [149a7086b776e5da4e4d15b857fedbf7ff5317fbaa777ca230bc76ae4e701069]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cases
  - "--"
  - "--exact"
  - a_failed_runs_body_says_the_expected_output_was_missing_before_its_key_lines
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:ddbb69816582caa98f2fcf40c07515902e2eb9ff5d7c6bbf400cd04080213a71
criteria: [LIFE-7]
requirements: [REQ-cli-017]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:51:19Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact a_failed_runs_body_says_the_expected_output_was_missing_before_its_key_lines for TC-cli-failed-run-body-says-why: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:51:19Z. The raw log stays in the machine-local store.
