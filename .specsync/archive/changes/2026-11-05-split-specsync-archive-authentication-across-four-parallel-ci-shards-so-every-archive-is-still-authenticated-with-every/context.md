---
change: split-specsync-archive-authentication-across-four-parallel-ci-shards-so-every-archive-is-still-authenticated-with-every
artifact: context
---

# Context

The hosted `SpecSync Archive` job has a 10-minute limit. It took about 7 minutes on
#101 (13 commits), 8 on #92 (26), 9m51s on #77 (42), and on #93 (91 commits) it was
cancelled at the 10-minute limit with every other check green. Profiling the script on
#97's branch (152 commits, 78 archives) showed the per-revision roster walk costs about
0.9 s per commit, while `specsync change status <id>` for each archived change costs
about 26 to 35 seconds inside SpecSync (no Git subprocesses). Authenticating all 78
archives with 4 CPU-bound workers is the bulk of the job, and it grows with every
archive.

Leif chose "Make the check faster" on 2026-11-04, after "Raise it to 30 min" and
"Merge past the timeout" were offered. DRIVE-10 requires every archive to be
authenticated with every check unchanged, within the unchanged job limit. Skipping
archives that are unchanged since the base was considered and rejected, because it
would no longer authenticate every archive. The reads are therefore split across
parallel runners instead.

Out of scope: SpecSync's own per-status cost (an upstream matter), branch protection,
and the 10-minute limit, which stays.
