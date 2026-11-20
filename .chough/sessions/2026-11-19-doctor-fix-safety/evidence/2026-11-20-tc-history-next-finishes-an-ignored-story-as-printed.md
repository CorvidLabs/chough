---
kind: evidence
id: 2026-11-20-tc-history-next-finishes-an-ignored-story-as-printed
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
duration_seconds: 1
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:e21f89c4479d3ede73eb8c9e74b31cec1b9ff5b6eb005c7db10e892f5e5d184e
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:49:27Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact next_finishes_an_ignored_story_as_printed for TC-history-next-finishes-an-ignored-story-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:49:27Z. The raw log stays in the machine-local store.
