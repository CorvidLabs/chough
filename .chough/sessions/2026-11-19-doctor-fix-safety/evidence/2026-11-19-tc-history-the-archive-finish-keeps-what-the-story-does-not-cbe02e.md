---
kind: evidence
id: 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-cbe02e
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-the-archive-finish-keeps-what-the-story-does-not-hold]
case_digest: [7416481d236addab0948f663b50ee255bee4899d1a4fef1357dc85ec8049d528]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - the_archive_finish_keeps_what_the_story_does_not_hold
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:6aa0128c030ad421f5979981ad29995eebbd9238924967150164e35747412b14
criteria: [DRIVE-17]
requirements: [REQ-history-012, REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:39Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:39Z. The raw log stays in the machine-local store.
