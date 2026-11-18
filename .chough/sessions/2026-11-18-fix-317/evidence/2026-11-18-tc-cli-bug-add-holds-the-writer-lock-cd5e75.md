---
kind: evidence
id: 2026-11-18-tc-cli-bug-add-holds-the-writer-lock-cd5e75
aliases: []
claim: "Ran cargo test --locked --test evidence -- --exact bug_add_holds_the_writer_lock_and_keeps_another_writers_update for TC-cli-bug-add-holds-the-writer-lock: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-bug-add-holds-the-writer-lock]
case_digest: [88b43a2e26a9350c6cfdd0ad3deba6c79784fb2ff7c4280abcbce8120c782701]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - evidence
  - "--"
  - "--exact"
  - bug_add_holds_the_writer_lock_and_keeps_another_writers_update
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: b95d9f26a97953489f7226afb5b7cb15f4ea3cf0
content: e21a3c2adc3859056d646219f5fd3c179463928e3b650abced2626f0f114eed4
environment: local-macos-arm64
fingerprint: sha256:fbad0a766c7abf930e37c49564a53f5dceed3f0fa652e61174291472563fcded
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T16:56:56Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test evidence -- --exact bug_add_holds_the_writer_lock_and_keeps_another_writers_update for TC-cli-bug-add-holds-the-writer-lock: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b95d9f26a979, observed 2026-11-18T16:56:56Z. The raw log stays in the machine-local store.
