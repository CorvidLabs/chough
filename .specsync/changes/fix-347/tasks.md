---
change: fix-347
artifact: tasks
---

# Tasks

- [x] Add tests that fail on main: a new bug's Log links its evidence, and doctor relinks after archiving.
- [x] `bug add`, `bug update` and the automatic failure capture write Log evidence as relative links.
- [x] `chough doctor` reports `stale_bug_links`, and `--fix` appends a Log entry linking the archived story.
- [x] Document both in `docs/cli.md`; add the case definitions and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
