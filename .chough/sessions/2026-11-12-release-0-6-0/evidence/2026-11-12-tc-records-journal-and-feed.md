---
kind: evidence
id: 2026-11-12-tc-records-journal-and-feed
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: 423169dee4924d188777c611349ce0e0318820a8
content: d6225f745d500fe2fe4b1988410ab21474daacab529a7e82b06c53e516869afa
environment: local-macos-arm64
fingerprint: sha256:5ae90c1043a132acba33c2a3943740c15d4a39679b95168bdde0a81527cad375
criteria: [KNOW-33, KNOW-30, KNOW-31]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:03:54Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 423169dee492, observed 2026-11-12T16:03:54Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- done_writes_a_short_post_and_the_feed_lists_it stdout ----
- thread 'done_writes_a_short_post_and_the_feed_lists_it' (918528348) panicked at tests/journal.rs:283:28:
- no paragraph: # Ship a focused change
- Read more: [the pull request](https://example.invalid/SYNTHETIC-org/SYNTHETIC-repo/pull/7) · [the story](../../.chough/archive/sessions/2026-11-12-ship-a-focused-change.md) · [the archived change](../../.specsync/archive/changes/2026-05-01-c1/)
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.87s
