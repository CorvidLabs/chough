---
kind: evidence
id: 2026-11-15-tc-records-done-touches-only-its-own-post
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed for TC-records-done-touches-only-its-own-post: not-run (exit 0, tests run 0)."
result: not-run
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-done-touches-only-its-own-post]
case_digest: [356b2d6b5332add74202731b95dcb54c533480dea4f4dc51b3e2ec86691b22af]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - done_changes_no_knowledge_file_but_its_own_post_and_the_feed
exit_code: 0
tests_run: 0
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:c0836d3345fdef0b5827d198386deb9cb8443b950a39ed445c8bd5daf3eae898
criteria: [KNOW-30, KNOW-31, KNOW-33]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations:
  - no test matched the binding; nothing was asserted
  - inherited environment variables were not recorded
observed: 2026-11-15T20:22:14Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed for TC-records-done-touches-only-its-own-post: not-run (exit 0, tests run 0).**

Recorded as not-run in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:14Z. The raw log stays in the machine-local store.

## Key failing lines

- test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 19 filtered out; finished in 0.00s
