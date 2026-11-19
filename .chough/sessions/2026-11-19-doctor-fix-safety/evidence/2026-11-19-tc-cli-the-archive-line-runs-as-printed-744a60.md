---
kind: evidence
id: 2026-11-19-tc-cli-the-archive-line-runs-as-printed-744a60
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
duration_seconds: 3
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:39b143c74091f989a6ef6e7e1e1f8209aaca6693504f2a4a293af04b44b5f9a8
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:50:20Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:50:20Z. The raw log stays in the machine-local store.
