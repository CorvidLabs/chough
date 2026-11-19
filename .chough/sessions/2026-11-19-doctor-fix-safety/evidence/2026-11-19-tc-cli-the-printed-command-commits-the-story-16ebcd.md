---
kind: evidence
id: 2026-11-19-tc-cli-the-printed-command-commits-the-story-16ebcd
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
duration_seconds: 12
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:402b54b6bf510eebd309d8a3fb2a20bfc10cb5a7fc6d0cfbe2529e5ee83ecc55
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:13:20Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:13:20Z. The raw log stays in the machine-local store.
