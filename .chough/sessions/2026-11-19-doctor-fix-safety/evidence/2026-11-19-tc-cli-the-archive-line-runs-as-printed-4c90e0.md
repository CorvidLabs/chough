---
kind: evidence
id: 2026-11-19-tc-cli-the-archive-line-runs-as-printed-4c90e0
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
duration_seconds: 10
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:3eacb545f6aaeda32a1043a39528017634eb1882557cc9f75902653e0ac9b1f6
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:13:04Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:13:04Z. The raw log stays in the machine-local store.
