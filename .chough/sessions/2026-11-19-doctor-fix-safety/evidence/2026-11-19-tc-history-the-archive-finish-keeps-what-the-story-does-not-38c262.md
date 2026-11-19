---
kind: evidence
id: 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-38c262
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
revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
environment: local-macos-arm64
fingerprint: sha256:8c523b66c1fc68718f18738dbcddbbcb6f3b559550e61d9a4df684b56922e9f1
criteria: [DRIVE-17]
requirements: [REQ-history-012, REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:31:26Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 44aec05c95e2, observed 2026-11-19T14:31:26Z. The raw log stays in the machine-local store.
