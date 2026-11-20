---
kind: evidence
id: 2026-11-20-tc-cli-the-printed-command-commits-the-story
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
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:25ef800361649a5872bf736ff409e3f5c8a9a5051dd348c28eeef4bc435148ab
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:48:15Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:48:15Z. The raw log stays in the machine-local store.
