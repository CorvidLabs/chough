---
kind: evidence
id: 2026-11-18-tc-cli-bug-add-refuses-before-writing-when-the-session-is-full-259e47
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact bug_add_refuses_before_writing_when_the_session_is_full for TC-cli-bug-add-refuses-before-writing-when-the-session-is-full: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-bug-add-refuses-before-writing-when-the-session-is-full]
case_digest: [5fbcb526252ba3b500c8fc9ee04cee9b164a8cf40e1570923f9b372cc9001da9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - bug_add_refuses_before_writing_when_the_session_is_full
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 605a1dac66be7b2d80b2233cb9b135b1dee779d6
content: 2a61bed5154f861be0ac812c0d4a92fffef9f87932b6e27dd7e60aa0c8f780f8
environment: local-macos-arm64
fingerprint: sha256:76b30d7417818d4906748640139281bcf1b2205454a9a85caf1ff7f08dc0f93e
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:28:55Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact bug_add_refuses_before_writing_when_the_session_is_full for TC-cli-bug-add-refuses-before-writing-when-the-session-is-full: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 605a1dac66be, observed 2026-11-18T23:28:55Z. The raw log stays in the machine-local store.
