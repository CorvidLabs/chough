---
change: a-session-archives-itself-at-its-own-move-into-done-155
artifact: context
---

# Context

Issue #155, filed at the orc's request on 2026-11-06. Leif chose option D the same day.

`session new` and `session use` move every other done session into the archive (REQ-history-006). A done session merged to `main` stays open there, so the next `session new` in any worktree archives it again, and that move lands in an unrelated change. This happened four times overnight in spec-sync worktrees:
- ss-testgaps swept the move into a local commit;
- ss-contracts restored `s5-881.json` by hand;
- twice more before that.

Option D: a session archives itself at its own move into done, in its own pull request. It then reaches `main` already archived, and no other worktree moves it. This follows DRIVE-13 ("each closes at done and is archived") and DRIVE-3 ("automatic archiving of a finished session").

The one-time sweep: this change's own `session new` archives the done sessions now open on `main`, so after it merges none are left.
