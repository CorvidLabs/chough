---
kind: evidence
id: 2026-11-19-tc-history-an-ignored-story-holds-the-finish
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
duration_seconds: 5
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:5c7315ffb857e81fc5beae4397e483af98f01f288d61c2fc82c001c152dfbbb6
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:49:47Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:49:47Z. The raw log stays in the machine-local store.
