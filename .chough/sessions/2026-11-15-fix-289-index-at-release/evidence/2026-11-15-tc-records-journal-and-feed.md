---
kind: evidence
id: 2026-11-15-tc-records-journal-and-feed
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-journal-and-feed]
case_digest: [e841dbd34c7ad1f50eea02debd501bf9ef7e39e64f6b75256a5c650edec731c2]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - done_writes_a_short_post_and_the_feed_lists_it
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:82e0956340845387c0abe3084caa641a844d5db7035f458a9ce786ad55886c63
criteria: [KNOW-33, KNOW-30, KNOW-31]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:13Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:13Z. The raw log stays in the machine-local store.
