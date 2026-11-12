---
kind: evidence
id: 2026-11-12-tc-records-journal-post-uses-the-title-and-summary
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact the_post_uses_the_title_and_summary_the_agent_set for TC-records-journal-post-uses-the-title-and-summary: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-journal-post-uses-the-title-and-summary]
case_digest: [8d2a80ed3098d093d6da397a39bb07247a432b24a744f830df428f934cdc804d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - the_post_uses_the_title_and_summary_the_agent_set
exit_code: 101
tests_run: 1
duration_seconds: 39
revision: 423169dee4924d188777c611349ce0e0318820a8
content: d6225f745d500fe2fe4b1988410ab21474daacab529a7e82b06c53e516869afa
environment: local-macos-arm64
fingerprint: sha256:deb5b760ac34577075907903fe676816e7f6a1992e36c72010da0785abef2498
criteria: [KNOW-33]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:03:18Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact the_post_uses_the_title_and_summary_the_agent_set for TC-records-journal-post-uses-the-title-and-summary: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 423169dee492, observed 2026-11-12T16:03:18Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_post_uses_the_title_and_summary_the_agent_set stdout ----
- thread 'the_post_uses_the_title_and_summary_the_agent_set' (918506357) panicked at tests/journal.rs:1166:9:
- ["checkpoint", "--title", "ttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttt"]: error: unexpected argument '--title' found
-   tip: a similar argument exists: '--file'
- Usage: chough checkpoint --file <FILES>
- For more information, try '--help'.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out; finished in 16.39s
