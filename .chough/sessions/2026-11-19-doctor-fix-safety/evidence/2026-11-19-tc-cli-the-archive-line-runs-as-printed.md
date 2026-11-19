---
kind: evidence
id: 2026-11-19-tc-cli-the-archive-line-runs-as-printed
aliases: []
claim: "Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-the-archive-line-runs-as-printed]
case_digest: [413ca9241d303fc7cca2d3290c0406c7a4a723929d546d0dfef5bf5520ee6980]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - the_archive_line_runs_as_printed_with_other_archives_waiting
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
environment: local-macos-arm64
fingerprint: sha256:a61f9b204a43e828c91d6cac1473b5083e6588d2e1c0e7f05baa907dd52da949
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:32:08Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 44aec05c95e2, observed 2026-11-19T14:32:08Z. The raw log stays in the machine-local store.
