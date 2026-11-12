---
kind: evidence
id: 2026-11-12-tc-records-journal-add-record-backfills-a-story-92c347
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact journal_add_record_backfills_an_archived_story for TC-records-journal-add-record-backfills-a-story: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 69b60a68d925da1c3a5654890bd62627599666bd
content: 18d24a04ea352eed30462b2a6d05214a9659b6d958ad33db5ee89f1b546be238
environment: local-macos-arm64
fingerprint: sha256:7b460eeb11b9338f8d30b896386e03fb8547395b3a13e3a10e099acfcb5c25a9
criteria: [KNOW-33]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:19:50Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact journal_add_record_backfills_an_archived_story for TC-records-journal-add-record-backfills-a-story: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 69b60a68d925, observed 2026-11-12T16:19:50Z. The raw log stays in the machine-local store.
