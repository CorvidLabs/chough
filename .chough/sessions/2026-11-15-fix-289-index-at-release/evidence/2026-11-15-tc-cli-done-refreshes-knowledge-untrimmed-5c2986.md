---
kind: evidence
id: 2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed-5c2986
aliases: []
claim: "Ran cargo test --locked --test librarian_trim -- --exact done_refreshes_the_knowledge_index_when_nothing_is_trimmed for TC-cli-done-refreshes-knowledge-untrimmed: not-run (exit 0, tests run 0)."
result: not-run
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-done-refreshes-knowledge-untrimmed]
case_digest: [dc137cbd0ff74dfce469363e89061612bbfd028a9802935c94cfff77148e9ecf]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - librarian_trim
  - "--"
  - "--exact"
  - done_refreshes_the_knowledge_index_when_nothing_is_trimmed
exit_code: 0
tests_run: 0
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:f7fcea1e017cf0665f4a7f4ddea8246516dcc1879077371f416c6f2cc9c9f687
criteria: [KNOW-1, KNOW-5]
requirements: [REQ-cli-043]
contracts: [specs/cli/cli.spec.md]
limitations:
  - no test matched the binding; nothing was asserted
  - inherited environment variables were not recorded
observed: 2026-11-15T20:22:20Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test librarian_trim -- --exact done_refreshes_the_knowledge_index_when_nothing_is_trimmed for TC-cli-done-refreshes-knowledge-untrimmed: not-run (exit 0, tests run 0).**

Recorded as not-run in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:20Z. The raw log stays in the machine-local store.

## Key failing lines

- test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
