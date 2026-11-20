---
kind: evidence
id: 2026-11-20-tc-records-migrate-a-new-bug-rebuilds-the-index-in-place-c8b356
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place for TC-records-migrate-a-new-bug-rebuilds-the-index-in-place: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-a-new-bug-rebuilds-the-index-in-place]
case_digest: [99c3ddff20c586e5ad229f40b496f086bdeed59ca2988decaa488c3b21eddd83]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place
exit_code: 0
tests_run: 1
duration_seconds: 5
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:a399f5c2e04d4a8c793d242c3372d0c1d696c8473a4946b0a96c5bb0702d77d4
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:59:43Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place for TC-records-migrate-a-new-bug-rebuilds-the-index-in-place: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T03:59:43Z. The raw log stays in the machine-local store.
