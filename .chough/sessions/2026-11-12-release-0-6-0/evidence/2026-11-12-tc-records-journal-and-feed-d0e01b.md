---
kind: evidence
id: 2026-11-12-tc-records-journal-and-feed-d0e01b
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
revision: 69b60a68d925da1c3a5654890bd62627599666bd
content: 18d24a04ea352eed30462b2a6d05214a9659b6d958ad33db5ee89f1b546be238
environment: local-macos-arm64
fingerprint: sha256:e4e6f3c579e909c3228e6314d0f44377c8f7d086eab5198d39f2ee4048e7550f
criteria: [KNOW-33, KNOW-30, KNOW-31]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:19:55Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 69b60a68d925, observed 2026-11-12T16:19:55Z. The raw log stays in the machine-local store.
