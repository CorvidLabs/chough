---
kind: evidence
id: 2026-11-12-tc-cli-failed-run-body-says-why-ed1722
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
revision: 421a7034dfce0c052d48c4d1fb5abe041c272a46
content: 9e16bb2234c3aadbf27905868186d8ca01d209615750221de4f1f7201e15af54
environment: local-macos-arm64
fingerprint: sha256:f9eb9ac8477e077220d8321f3d9f0e02607bad7ff6534c04ecc154675f594d8a
criteria: [LIFE-7]
requirements: [REQ-cli-017]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:05:48Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact a_failed_runs_body_says_the_expected_output_was_missing_before_its_key_lines for TC-cli-failed-run-body-says-why: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 421a7034dfce, observed 2026-11-12T23:05:48Z. The raw log stays in the machine-local store.
