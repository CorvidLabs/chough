---
kind: evidence
id: 2026-11-20-tc-history-the-archive-finish-keeps-what-the-story-does-not
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
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:eb0516b24dd32f7e49aca433e3bb8a6489ce49e312cd1af44950d72ea83d44d4
criteria: [DRIVE-17]
requirements: [REQ-history-012, REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:48:21Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:48:21Z. The raw log stays in the machine-local store.
