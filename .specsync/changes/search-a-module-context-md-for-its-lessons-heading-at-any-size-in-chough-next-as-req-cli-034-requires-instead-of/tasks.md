---
change: search-a-module-context-md-for-its-lessons-heading-at-any-size-in-chough-next-as-req-cli-034-requires-instead-of
artifact: tasks
---

# Tasks

- [ ] Failing test first: `tests/next.rs::next_searches_a_context_md_larger_than_4_mib`
  fails at the current head with the 4 MiB "not searched" note
- [ ] `lessons_heading` in src/main.rs reads the file line by line at any size;
  `LESSON_SOURCE_LIMIT` removed
- [ ] Case TC-cli-next-searches-large-context-md, docs/cli.md, CHANGELOG.md and the
  specs/cli/context.md lesson no longer describe `next` as a 4 MiB exception
