---
change: fix-272-uninitialised-root
artifact: context
---

# Context

The agent review of #302 (fix for #272) found a path the merged fix does not cover. `chough journal add` also works in a project that has no `.chough/` directory, with default preferences. The fix called the helper `lock_project`, which takes no lock when `.chough/` is absent, so on such a project `journal add` still runs unlocked. The reviewer ran three storms of twelve adds on fresh roots; one feed listed 11 posts. That is on main at 7aed03f.

REQ-cli-040 names `journal add` among the writers that hold the lock and makes no exception for an uninitialised project. Leif chose, on 2026-11-17, to fix forward by taking the lock there too, not by refusing such a project. Taking the lock creates `.chough/local/` with its `.gitignore`, as any first writer does.

No requirement text changes.
