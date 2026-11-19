---
kind: evidence
id: 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-ab3fd3
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
duration_seconds: 11
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:466b45300dde2a942d99dd59a765f759642ebf0da9a7c2389f7db78205c1e6e6
criteria: [DRIVE-17]
requirements: [REQ-history-012, REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:09:08Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:09:08Z. The raw log stays in the machine-local store.
