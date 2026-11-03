---
change: search-a-module-context-md-for-its-lessons-heading-at-any-size-in-chough-next-as-req-cli-034-requires-instead-of
artifact: tasks
---

# Tasks

- [x] Failing test first: `tests/next.rs::next_searches_a_context_md_larger_than_4_mib`
  failed at b680ef6 with note "larger than 4194304 bytes, not searched" for both modules
- [x] `lessons_heading` in src/main.rs reads the file line by line at any size;
  `LESSON_SOURCE_LIMIT` removed
- [x] Case TC-cli-next-searches-large-context-md, docs/cli.md, CHANGELOG.md and the
  specs/cli/context.md lesson no longer describe `next` as a 4 MiB exception
