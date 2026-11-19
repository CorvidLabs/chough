---
kind: evidence
id: 2026-11-19-tc-cli-the-printed-command-commits-the-story-5bc003
aliases: []
claim: "Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-the-printed-command-commits-the-story]
case_digest: [b75f11fceab686a56c7bb4889f42258c3e5427cf589f33f7ba10823be063e9fa]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - the_printed_command_commits_the_story_with_or_without_a_journal_post
exit_code: 0
tests_run: 1
duration_seconds: 5
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:a7ac975baeb1c9da52e36cfb273e1614c45051e22eb7e1160696365edd19f21d
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:50:28Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:50:28Z. The raw log stays in the machine-local store.
