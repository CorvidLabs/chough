---
kind: evidence
id: 2026-11-13-tc-cli-failed-run-body-says-why-56eef8
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:a175f062866dab520bbde79c84a43d38fddcf221422a04eadb023afaec39397d
criteria: [LIFE-7]
requirements: [REQ-cli-017]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:19:01Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact a_failed_runs_body_says_the_expected_output_was_missing_before_its_key_lines for TC-cli-failed-run-body-says-why: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:19:01Z. The raw log stays in the machine-local store.
