---
kind: evidence
id: 2026-11-13-tc-cli-failed-run-body-says-why
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:3c7e30b2c995ff3fc6ac483cd3c9ccef8332f507f2895759e0adefd92d5ffa32
criteria: [LIFE-7]
requirements: [REQ-cli-017]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:24:28Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact a_failed_runs_body_says_the_expected_output_was_missing_before_its_key_lines for TC-cli-failed-run-body-says-why: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:24:28Z. The raw log stays in the machine-local store.
