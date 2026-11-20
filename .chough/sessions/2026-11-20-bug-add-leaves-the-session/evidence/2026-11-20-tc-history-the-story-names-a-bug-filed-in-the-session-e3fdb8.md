---
kind: evidence
id: 2026-11-20-tc-history-the-story-names-a-bug-filed-in-the-session-e3fdb8
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
revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
environment: local-macos-arm64
fingerprint: sha256:0eeaadca9626c38254f9779bc3e0cbd3a52061f28c63f26749cd3307b3830001
criteria: [LIFE-5]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:34:49Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact the_story_names_a_bug_filed_in_the_session for TC-history-the-story-names-a-bug-filed-in-the-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 04c4b86f441c, observed 2026-11-20T16:34:49Z. The raw log stays in the machine-local store.
