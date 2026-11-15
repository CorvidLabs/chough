---
kind: evidence
id: 2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed-b4eb41
aliases: []
claim: "Ran cargo test --locked --test librarian_trim -- --exact done_defers_the_knowledge_index_refresh_when_nothing_is_trimmed for TC-cli-done-refreshes-knowledge-untrimmed: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-done-refreshes-knowledge-untrimmed]
case_digest: [b7ad95278fb3adca8592e60b1beae40fb60a70a81abd9026e590ffbd389d6ca3]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - librarian_trim
  - "--"
  - "--exact"
  - done_defers_the_knowledge_index_refresh_when_nothing_is_trimmed
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:d5ff80c5c944fd04edada23296150ae59647b3b32ec71211ca9a4830e795fda7
criteria: [KNOW-1, KNOW-5]
requirements: [REQ-cli-043]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:43Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test librarian_trim -- --exact done_defers_the_knowledge_index_refresh_when_nothing_is_trimmed for TC-cli-done-refreshes-knowledge-untrimmed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:43Z. The raw log stays in the machine-local store.
