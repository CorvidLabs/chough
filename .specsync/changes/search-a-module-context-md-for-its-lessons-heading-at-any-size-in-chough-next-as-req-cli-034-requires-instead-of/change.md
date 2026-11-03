---
id: search-a-module-context-md-for-its-lessons-heading-at-any-size-in-chough-next-as-req-cli-034-requires-instead-of
state: approved
type: bug_fix
base_commit: 45f7012197e0aa5c2ffdb163cc0025fd09b37b1a
---

# Search a module context.md for its Lessons heading at any size in chough next, and remove next's named 4 MiB exception from REQ-cli-034 while library snapshot's exception stays

## Intent

Search a module context.md for its Lessons heading at any size in chough next, and remove next's named 4 MiB exception from REQ-cli-034 while library snapshot's exception stays

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- `chough next` reports the 1-based line of `## Lessons` in a module context.md larger than 4 MiB, CRLF included, and `no Lessons section` for one without it, never a size note
- tests/next.rs::next_searches_a_context_md_larger_than_4_mib fails before the fix and passes after it, and the existing lessons pointer test still passes

## No-spec Rationale

Not applicable
