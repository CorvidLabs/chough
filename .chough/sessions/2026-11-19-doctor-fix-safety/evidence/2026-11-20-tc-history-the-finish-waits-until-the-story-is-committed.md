---
kind: evidence
id: 2026-11-20-tc-history-the-finish-waits-until-the-story-is-committed
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
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:80e51b74fda1424d6410745e438e58a8b405a7e1e148f8a171b91ce487888528
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:48:53Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:48:53Z. The raw log stays in the machine-local store.
