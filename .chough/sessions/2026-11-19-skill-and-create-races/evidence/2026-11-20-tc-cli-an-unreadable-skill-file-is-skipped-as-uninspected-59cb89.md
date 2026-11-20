---
kind: evidence
id: 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected-59cb89
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected]
case_digest: [086bcec2745fd2bf5bacaac4374db24720b2860179bb63f9fb10168a7c418e49]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - an_unreadable_skill_file_is_skipped_as_uninspected
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
environment: local-macos-arm64
fingerprint: sha256:331ad919a2aeb157c5c2785b24967fdd2c693939d16898a66d410439274b9082
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:35:47Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cece9573c0c, observed 2026-11-20T14:35:47Z. The raw log stays in the machine-local store.
