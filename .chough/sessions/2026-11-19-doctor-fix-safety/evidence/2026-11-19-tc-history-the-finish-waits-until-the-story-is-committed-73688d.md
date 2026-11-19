---
kind: evidence
id: 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed-73688d
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
duration_seconds: 4
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:a34175570ba734df3e411015afeee5f28c039162aefc32ee4852dfd26b6b68a9
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:14:19Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:14:19Z. The raw log stays in the machine-local store.
