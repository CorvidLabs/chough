---
kind: evidence
id: 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly for TC-cli-a-skill-file-it-cannot-write-is-refused-plainly: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-skill-file-it-cannot-write-is-refused-plainly]
case_digest: [f4ac5530827e43d00ea38628952c7a73e90c99a30c9c9ecde6ca2d77b62a7b39]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - a_skill_file_it_cannot_write_is_refused_plainly
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 63057710d3223425b0db8d988717f2c657e5035b
content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
environment: local-macos-arm64
fingerprint: sha256:619c24eb0d0eb517c08a36414db933f505d02a6726c4782c2716d26b0f3ede56
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T09:20:36Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly for TC-cli-a-skill-file-it-cannot-write-is-refused-plainly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 63057710d322, observed 2026-11-20T09:20:36Z. The raw log stays in the machine-local store.
