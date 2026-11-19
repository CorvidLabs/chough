---
kind: evidence
id: 2026-11-19-tc-cli-the-printed-command-commits-the-story-a2638f
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
duration_seconds: 1
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:23521c9a82ce9ac3fec2e376ef3bd39b6c39f1b82fd26796b8a195e86df88c68
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:34Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:34Z. The raw log stays in the machine-local store.
