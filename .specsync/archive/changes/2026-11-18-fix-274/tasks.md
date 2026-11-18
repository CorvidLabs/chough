---
change: fix-274
artifact: tasks
---

# Tasks

- [x] Add tests that fail on main: a refused `bug update` stops the exercise where it happens, and no `$chough` or `specsync` output is piped straight into `jq`.
- [x] Capture each such command's output before `jq` reads it, in `examples/feature-bug-loop.sh`.
- [x] Add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
