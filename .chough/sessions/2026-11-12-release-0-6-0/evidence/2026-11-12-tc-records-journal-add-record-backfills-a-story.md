---
kind: evidence
id: 2026-11-12-tc-records-journal-add-record-backfills-a-story
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact journal_add_record_backfills_an_archived_story for TC-records-journal-add-record-backfills-a-story: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-journal-add-record-backfills-a-story]
case_digest: [05bb42fb4b37d6f62c5916713ea6adae491247f6f03abfe2b0fdb6381e291559]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - journal_add_record_backfills_an_archived_story
exit_code: 101
tests_run: 1
duration_seconds: 7
revision: 423169dee4924d188777c611349ce0e0318820a8
content: d6225f745d500fe2fe4b1988410ab21474daacab529a7e82b06c53e516869afa
environment: local-macos-arm64
fingerprint: sha256:f860dce098c5c16ece95cc594e0bbfe326705bd11ed20bd938af184a8c9ca437
criteria: [KNOW-33]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:03:44Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact journal_add_record_backfills_an_archived_story for TC-records-journal-add-record-backfills-a-story: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 423169dee492, observed 2026-11-12T16:03:44Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- journal_add_record_backfills_an_archived_story stdout ----
- thread 'journal_add_record_backfills_an_archived_story' (918523053) panicked at tests/journal.rs:1282:5:
- error: unexpected argument '--record' found
- Usage: chough journal add [OPTIONS] --kind <KIND> --title <TITLE> --body <BODY>
- For more information, try '--help'.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out; finished in 6.64s
