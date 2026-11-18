---
change: fix-263
artifact: testing
---

# Testing

`tests/migrate_knowledge.rs::a_separate_file_at_a_case_colliding_target_refuses_before_any_write`
puts a junk `knowledge/index.md` beside the 0.5.x `knowledge/INDEX.md` and
applies the plan. It expects `refused:target-exists` naming the file and a tree
that is byte for byte unchanged. On a case-insensitive file system the state
cannot exist, so the test says so and stops; it runs for real on Linux, in the
`chough-linux:3` image locally and in CI.

A unit test, `two_names_differing_only_in_case_are_one_file_only_when_the_folder_lists_one`,
covers the file-system check on both kinds of file system.

The existing case-only rename and interrupted-apply tests still pass, so macOS
migrations are unchanged.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-009 | the two tests above and the case in `.chough/cases/records.migrate-case-colliding-target.cases.toml` |
