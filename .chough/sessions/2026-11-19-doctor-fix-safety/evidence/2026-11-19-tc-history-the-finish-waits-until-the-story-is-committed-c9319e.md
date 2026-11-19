---
kind: evidence
id: 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed-c9319e
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
duration_seconds: 9
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:25026e1758dca3d0c3c13d3cd6832192d39c208d6c5654d77e65f3d9ab5cc08a
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:10:35Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:10:35Z. The raw log stays in the machine-local store.
