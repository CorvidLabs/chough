---
kind: evidence
id: 2026-11-19-tc-history-next-finishes-an-ignored-story-as-printed-4b7c95
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact next_finishes_an_ignored_story_as_printed for TC-history-next-finishes-an-ignored-story-as-printed: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-next-finishes-an-ignored-story-as-printed]
case_digest: [7f40de6b91730e7a1a72c62501c17629ca1672d92fac8f4ad1cfe15e6fa9976e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - next_finishes_an_ignored_story_as_printed
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:824d6939c34663e431e301ffddd87a07832986fb30a86088aaa26521f8a5e1c3
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:36Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact next_finishes_an_ignored_story_as_printed for TC-history-next-finishes-an-ignored-story-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:36Z. The raw log stays in the machine-local store.
