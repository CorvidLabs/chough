---
change: scale-independent-archive-authentication-workers-with-the-runner-s-cpus-so-the-archive-check-fits-its-unchanged-budget
artifact: tasks
---

# Tasks

1. Add DRIVE-10 to hi/drive.md, then run `hi index` and `hi check`.
2. Add failing-first tests for the worker count (CPU counts of 1, 3, 4, 64 and unknown) and the overlap at that count.
3. Implement `archive_workers()` and use it in `authenticate_archives`.
4. Measure the check on the #71 head with 2, 4 and 8 workers on the box.
5. Add a CHANGELOG entry, run `fledge lanes run verify` and `specsync change check --commit`.
6. Observe the hosted Archive job duration on this PR before merge.
