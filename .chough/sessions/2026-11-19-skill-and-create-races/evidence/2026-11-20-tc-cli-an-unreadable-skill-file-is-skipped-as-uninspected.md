---
kind: evidence
id: 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected
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
revision: 63057710d3223425b0db8d988717f2c657e5035b
content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
environment: local-macos-arm64
fingerprint: sha256:5376fdd1603c85631016e0046a4862ebe8c073f19de844338806000b6bb5e262
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T09:20:37Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 63057710d322, observed 2026-11-20T09:20:37Z. The raw log stays in the machine-local store.
