---
kind: evidence
id: 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly-3f8156
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
revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
environment: local-macos-arm64
fingerprint: sha256:619c24eb0d0eb517c08a36414db933f505d02a6726c4782c2716d26b0f3ede56
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:35:45Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly for TC-cli-a-skill-file-it-cannot-write-is-refused-plainly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cece9573c0c, observed 2026-11-20T14:35:45Z. The raw log stays in the machine-local store.
