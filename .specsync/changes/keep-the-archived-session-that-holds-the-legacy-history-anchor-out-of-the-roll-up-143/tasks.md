---
change: keep-the-archived-session-that-holds-the-legacy-history-anchor-out-of-the-roll-up-143
artifact: tasks
---

# Tasks

- [x] Capture Leif's KNOW-29 amendment in `hi/knowledge.md` with its source.
- [x] Model a single anchor holder in the KNOW-29 fixture, and add the failing test `the_roll_up_keeps_the_session_that_holds_the_legacy_anchor`.
- [ ] In `roll_up`, leave out the archived session whose `history_cursor.legacy` is set before ranking, so it never moves and does not count toward the five.
- [ ] Update `docs/cli.md` and `CHANGELOG.md`, add the case, and run verify on Linux.
