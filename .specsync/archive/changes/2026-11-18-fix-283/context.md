---
change: fix-283
artifact: context
---

# Context

GitHub #283, seen in CorvidLabs/jackdaw#155. `chough cases check` refuses a retired case whose `fledge-task` binding names a task since removed from `fledge.toml`. In `check_links` the task-existence check runs before the retired-case early return. Removing the task is often why the case was retired, so a repo had to keep a dead task or rewrite the retired case's historical binding.

Leif's decision (2026-11-18, planning interview, recorded on #283 and relayed by hc t4c as the scope approval) is to skip the check for retired cases. `cases check` returns early for a retired case before checking its `fledge-task` binding, so the original binding stays as history.

A retired case already never runs: `find_checked` refuses it. No requirement states the old order, so this is a no-spec change.
