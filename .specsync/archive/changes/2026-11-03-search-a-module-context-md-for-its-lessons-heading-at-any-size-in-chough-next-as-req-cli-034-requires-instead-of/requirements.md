---
change: search-a-module-context-md-for-its-lessons-heading-at-any-size-in-chough-next-as-req-cli-034-requires-instead-of
artifact: requirements
---

# Requirements

MODIFIES REQ-cli-034 (deltas/cli.md). `chough next` now searches a module's
`context.md` for its `## Lessons` heading at any size, so the requirement no longer
names `next` as a known 4 MiB exception, and it gains the acceptance line for that
search. `chough library snapshot`'s 4 MiB refusal stays a named exception for its own
later change. All other REQ-cli-034 wording is unchanged.
