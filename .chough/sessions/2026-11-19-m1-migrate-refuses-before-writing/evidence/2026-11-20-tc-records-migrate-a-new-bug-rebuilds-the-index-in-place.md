---
kind: evidence
id: 2026-11-20-tc-records-migrate-a-new-bug-rebuilds-the-index-in-place
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
duration_seconds: 2
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:b9c12bfc5c5f141838371c27bfedf40e0f51a3f6e7294d4e6692a43c20a47686
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:24:15Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place for TC-records-migrate-a-new-bug-rebuilds-the-index-in-place: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:24:15Z. The raw log stays in the machine-local store.
