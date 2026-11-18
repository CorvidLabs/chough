---
change: fix-271
artifact: tasks
---

# Tasks

- [x] Add tests that fail on main: a pinned shell writes to its own session whatever the pointer says, and a pin to a session that is not open refuses.
- [x] Add the global `--session` option and `CHOUGH_SESSION`, resolved in `sessions::active`, leaving `init`, `inbox add`, `session new` and `session use` as they are.
- [x] Document them in `docs/cli.md`; update the five hc templates and append their versions in `integrations/templates.toml`.
- [x] Add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
