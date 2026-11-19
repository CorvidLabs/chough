---
kind: evidence
id: 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed-0c0669
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
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:a9ef8670d5b74e01f7b8c1ff75e37aa6e6d70dac34ab4ca5b98ee8f193f21a68
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:07Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:07Z. The raw log stays in the machine-local store.
