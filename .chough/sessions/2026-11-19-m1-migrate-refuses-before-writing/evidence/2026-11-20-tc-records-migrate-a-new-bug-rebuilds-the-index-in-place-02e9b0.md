---
kind: evidence
id: 2026-11-20-tc-records-migrate-a-new-bug-rebuilds-the-index-in-place-02e9b0
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
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:470b8d4ed4b470e1ba4845800e0fcb72799e315fde80785128798caf815ae9f1
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:36:26Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place for TC-records-migrate-a-new-bug-rebuilds-the-index-in-place: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:36:26Z. The raw log stays in the machine-local store.
