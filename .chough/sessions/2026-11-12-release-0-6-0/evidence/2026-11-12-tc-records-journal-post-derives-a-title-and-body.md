---
kind: evidence
id: 2026-11-12-tc-records-journal-post-derives-a-title-and-body
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact without_a_title_or_summary_the_post_derives_both_from_the_story for TC-records-journal-post-derives-a-title-and-body: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-journal-post-derives-a-title-and-body]
case_digest: [06cccea745f8ce3a4b3adec1fc302844aeeeb3b1392f690b73e520f13889dc4d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - without_a_title_or_summary_the_post_derives_both_from_the_story
exit_code: 101
tests_run: 1
duration_seconds: 11
revision: 423169dee4924d188777c611349ce0e0318820a8
content: d6225f745d500fe2fe4b1988410ab21474daacab529a7e82b06c53e516869afa
environment: local-macos-arm64
fingerprint: sha256:9a2c5cc9bd2707f26bc06256deaaadfc792d8184f7d60f0024830d3a69094c81
criteria: [KNOW-33]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:03:34Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact without_a_title_or_summary_the_post_derives_both_from_the_story for TC-records-journal-post-derives-a-title-and-body: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 423169dee492, observed 2026-11-12T16:03:34Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- without_a_title_or_summary_the_post_derives_both_from_the_story stdout ----
- thread 'without_a_title_or_summary_the_post_derives_both_from_the_story' (918517139) panicked at tests/journal.rs:1225:5:
- assertion `left == right` failed: ---
- id: 2026-11-12-ship-human-first-records-as-one-change-readable-markdown-files
- title: "Ship human-first records as one change, readable Markdown files and one story per session: dogfooded here"
- type: journal
- tags: [chough/journal, chough/shipped]
- date: 2026-11-12
- time: 2026-11-12T16:03:29Z
- kind: shipped
- links:
-   - https://example.invalid/SYNTHETIC-org/SYNTHETIC-repo/pull/7
-   - .chough/archive/sessions/2026-11-12-ship-a-focused-change.md
-   - .specsync/archive/changes/2026-05-01-c1/
- record: .chough/archive/sessions/2026-11-12-ship-a-focused-change.md
- ---
- # Ship human-first records as one change, readable Markdown files and one story per session: dogfooded here
- Read more: [the pull request](https://example.invalid/SYNTHETIC-org/SYNTHETIC-repo/pull/7) · [the story](../../.chough/archive/sessions/2026-11-12-ship-a-focused-change.md) · [the archived change](../../.specsync/archive/changes/2026-05-01-c1/)
-   left: "Ship human-first records as one change, readable Markdown files and one story per session: dogfooded here"
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out; finished in 10.87s
