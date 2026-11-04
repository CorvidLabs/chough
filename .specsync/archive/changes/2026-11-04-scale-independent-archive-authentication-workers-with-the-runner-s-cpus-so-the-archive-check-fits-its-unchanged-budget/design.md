---
change: scale-independent-archive-authentication-workers-with-the-runner-s-cpus-so-the-archive-check-fits-its-unchanged-budget
artifact: design
---

# Design

- `archive_workers()` returns `max(2, min(os.cpu_count() or 2, 8))`.
- `authenticate_archives` uses `ThreadPoolExecutor(max_workers=archive_workers())`. Nothing else in the executor changes: it prechecks presence, submits in sorted ID order, reports errors in ID order and joins every read before leaving the context.
- No other function changes.
