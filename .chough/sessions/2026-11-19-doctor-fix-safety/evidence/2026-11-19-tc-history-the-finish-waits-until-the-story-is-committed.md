---
kind: evidence
id: 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-the-finish-waits-until-the-story-is-committed]
case_digest: [8c422734c0abfe1244f590414ca6d32cb06ec08f39d05f89e13eff559232cec9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - the_finish_waits_until_the_story_and_post_are_committed
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
environment: local-macos-arm64
fingerprint: sha256:57e6c6e636ef360382f2c02bdf871422baf1f62292cc7caed06e85a39f9918f3
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:31:54Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 44aec05c95e2, observed 2026-11-19T14:31:54Z. The raw log stays in the machine-local store.
