---
kind: evidence
id: 2026-11-18-tc-cli-bug-add-refuses-before-writing-when-the-session-is-full
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
revision: be74e8fde1a456a3dba7f4e54927e1ad55d56692
content: 9c836aa5279e28f997f91039668afd364b851446489c7f3052bac8d38239ca4b
environment: local-macos-arm64
fingerprint: sha256:0f905c38379cf456d1f70f9d7211b4c18f8d86d3f821e0bbef8cf1fbba7fc262
criteria: [LIFE-5]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:20:45Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact bug_add_refuses_before_writing_when_the_session_is_full for TC-cli-bug-add-refuses-before-writing-when-the-session-is-full: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision be74e8fde1a4, observed 2026-11-18T23:20:45Z. The raw log stays in the machine-local store.
