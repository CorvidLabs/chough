---
kind: evidence
id: 2026-11-19-tc-cli-the-archive-line-runs-as-printed-5b625b
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
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:b125451cba21cd07709b86debd86197d649a4e3ea597ac5a5d1466a351d795f2
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:31Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:31Z. The raw log stays in the machine-local store.
