---
kind: evidence
id: 2026-11-19-tc-cli-the-printed-command-commits-the-story-e0f5f6
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
duration_seconds: 2
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:e02cadc16d9fff588947dffc4f241af95d6503ce4c05ab79cb1610d250639341
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:15:21Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:15:21Z. The raw log stays in the machine-local store.
