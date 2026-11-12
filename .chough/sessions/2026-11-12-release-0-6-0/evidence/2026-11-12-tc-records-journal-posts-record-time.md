---
kind: evidence
id: 2026-11-12-tc-records-journal-posts-record-time
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact posts_record_a_time_and_sort_newest_first_by_it for TC-records-journal-posts-record-time: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-journal-posts-record-time]
case_digest: [74ec8f28cd200721a2d67fef9434b49aae0a56e2cdc1e90db85085a4b456f503]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - posts_record_a_time_and_sort_newest_first_by_it
exit_code: 0
duration_seconds: 0
revision: 69b60a68d925da1c3a5654890bd62627599666bd
content: 18d24a04ea352eed30462b2a6d05214a9659b6d958ad33db5ee89f1b546be238
environment: local-macos-arm64
fingerprint: sha256:4e8971db9f073b78a70c0377703e90fe95c9c8eb88322249b5b3d23a472a1935
criteria: [KNOW-33]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T16:19:58Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact posts_record_a_time_and_sort_newest_first_by_it for TC-records-journal-posts-record-time: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 69b60a68d925, observed 2026-11-12T16:19:58Z. The raw log stays in the machine-local store.
