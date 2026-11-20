---
kind: evidence
id: 2026-11-20-tc-history-the-story-names-a-bug-filed-in-the-session
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact the_story_names_a_bug_filed_in_the_session for TC-history-the-story-names-a-bug-filed-in-the-session: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-the-story-names-a-bug-filed-in-the-session]
case_digest: [cbcf83f40b37912bb08aa22baa458ea469ac3453dd3169aa8d67f039fcfc00ab]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - the_story_names_a_bug_filed_in_the_session
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
environment: local-macos-arm64
fingerprint: sha256:2a38035de23e1b8c3988781548595a766f90ef74906b8da1f1a51f3d345a5770
criteria: [LIFE-5]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T15:48:36Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact the_story_names_a_bug_filed_in_the_session for TC-history-the-story-names-a-bug-filed-in-the-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 2f99dcfb42f1, observed 2026-11-20T15:48:36Z. The raw log stays in the machine-local store.
