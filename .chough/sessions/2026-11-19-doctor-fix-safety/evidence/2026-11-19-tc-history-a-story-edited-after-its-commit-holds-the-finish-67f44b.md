---
kind: evidence
id: 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish-67f44b
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-story-edited-after-its-commit-holds-the-finish]
case_digest: [2caa4b3395afbcb07ad09017b5940a9f729d0b486ef2cf2abe9933fad7c32711]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_story_edited_after_its_commit_holds_the_finish
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:f371a08afed459439f1d167caeeea9b81f6ffea93d47f0cf8e3fc5b80743abc0
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:11Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:11Z. The raw log stays in the machine-local store.
