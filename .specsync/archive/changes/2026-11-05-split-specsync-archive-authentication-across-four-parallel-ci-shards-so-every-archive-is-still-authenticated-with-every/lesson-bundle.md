# Lesson bundle: split-specsync-archive-authentication-across-four-parallel-ci-shards-so-every-archive-is-still-authenticated-with-every

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Split SpecSync Archive authentication across four parallel CI shards so every archive is still authenticated with every check unchanged within the unchanged 10-minute job limit
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: .github/scripts/check-specsync-archive.py, .github/scripts/test-specsync-archive.py, .github/workflows/ci.yml, CHANGELOG.md
- **Acceptance**: With --shard K/N each shard authenticates the sorted archives at positions K, K+N and so on, so N shards read every archive exactly once; shard 1 also walks every PR revision and refuses an omitted archive; every shard refuses an active change, an unhealthy audit, a wrong SpecSync version and an invalid result; --shard defaults to 1/1 with output unchanged; ci.yml runs four shards, each with the unchanged 10-minute limit; the new tests fail before the change and the existing fixtures pass; the hosted shards pass on this PR.

## Evidence

- Verification commit: `8c02e6780c4128d04c067ae7757c98ff48478252`
- Base commit: `166a8f52caa772fa6bb33361e195772f0005e2f4`
- Verified by: `specsync check --spec cli`

## From the change's context.md

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

## From the change's testing.md

# Testing

`python3 .github/scripts/test-specsync-archive.py` (the `CI gate fixtures` job and
`fledge run ci-archive-fixtures`) gains executor tests for shard partitioning and
argument validation, and real-native fixture tests run with `--shard`. A shard
authenticates only its own archives. Shard 1 still refuses an archive removed during
the PR, and every shard refuses an active change. The new tests fail before the
change. The existing executor and fixture tests pass unchanged, including the 1/1
output "N archive(s) authenticated". The hosted `SpecSync Archive (K/4)` jobs on
this PR must each pass within 10 minutes.

## Where these lessons go

- `specs/cli/context.md`
