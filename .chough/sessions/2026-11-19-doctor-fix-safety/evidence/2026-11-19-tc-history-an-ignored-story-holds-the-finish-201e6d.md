---
kind: evidence
id: 2026-11-19-tc-history-an-ignored-story-holds-the-finish-201e6d
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-ignored-story-holds-the-finish]
case_digest: [86e02ab9da1d4f5559805d2d6126a8bc190ccf64d1956cc991f5101bbc274b87]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_ignored_story_holds_the_finish_and_names_git_add_f
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:6c9f6068a655ae53c4a7c11fe4d26f86e0c07e53e096c708813806cfdcc63bbc
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:14Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:14Z. The raw log stays in the machine-local store.
