---
kind: evidence
id: 2026-11-20-tc-cli-the-archive-line-runs-as-printed
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
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:2be04fa072138bed17e48fac598c0e14fb1de8659d1a745e719ab413f1ed0693
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:48:11Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:48:11Z. The raw log stays in the machine-local store.
