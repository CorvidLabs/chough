---
kind: evidence
id: 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly-400c1d
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
revision: 29fb39131c6435612182f52929b94e69ab0dcba3
content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
environment: local-macos-arm64
fingerprint: sha256:c47cc9e805ec8447d87b6c85b6018ab70b0b9cf025f5189d26d1164a5f7d8f39
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:19:25Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly for TC-cli-a-skill-file-it-cannot-write-is-refused-plainly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 29fb39131c64, observed 2026-11-20T14:19:25Z. The raw log stays in the machine-local store.
