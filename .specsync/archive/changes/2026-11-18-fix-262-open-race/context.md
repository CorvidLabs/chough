---
change: fix-262-open-race
artifact: context
---

# Context

The agent review of #305 (fix for #262) found a window the fix leaves open. The bounded record reader checks that a path is a regular file and then opens it by path, as two separate steps. If the file is replaced with a FIFO between them, the check passes and the open still blocks for ever. The reviewer reproduced it by replacing `session.md` after the check: `session list` ran past a three second deadline. `session shelve` has the same shape for `session.md` and for the files it lists.

fix-262 is already reviewed and archived, so this is its own change on the same branch and pull request. Leif chose on 2026-11-17 to fix this before #305 merges. No requirement text changes.
