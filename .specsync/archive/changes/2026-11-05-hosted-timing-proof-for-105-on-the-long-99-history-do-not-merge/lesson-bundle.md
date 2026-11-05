# Lesson bundle: hosted-timing-proof-for-105-on-the-long-99-history-do-not-merge

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Hosted timing proof for 105 on the long 99 history (do not merge)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: .github/scripts/check-specsync-archive.py, .github/scripts/test-specsync-archive.py, .github/workflows/ci.yml
- **Acceptance**: The three proof paths are covered by this change and match #118 head e18e0941; with nothing relaxed the four hosted SpecSync Archive shards run on the 361-revision #99 history and their durations and counts are posted on #105.

## Evidence

- Verification commit: `069e840911d1bc41515fac72428bc24556b6e8dc`
- Base commit: `d0480d65f9420068d17f74488a5c74bf0118450e`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Draft PR #119 is a proof only. It will be closed unmerged, and its branches deleted.

It is #99's final head `6a9bfad6` plus one commit (`d0480d65`), which copies three files from #118's head `e18e0941`:
- `.github/scripts/check-specsync-archive.py`
- `.github/scripts/test-specsync-archive.py`
- `.github/workflows/ci.yml`

The aim is a hosted run of the four `SpecSync Archive` shards on issue #105's recorded long candidate: 361 PR revisions over trusted base `8b7f8bf6`, with 86 archives.

The first hosted run, 42128424556, refused correctly at the native audit, because no change covered those paths. Leif chose "A: add a native change" (2026-11-05 16:22 MST, via the orc): cover the three paths, with nothing relaxed. This change adds one archive (87).

After Leif's review and finalization, outside this change:
- collect the four hosted shard timings and post them on #105;
- close #119 unmerged;
- delete its branch and the temporary base branch.

## From the change's testing.md

# Testing

- The three files are byte-identical to #118's head `e18e0941`, where the archive fixtures pass (25 tests, OK, on Linux) and every hosted check passed. Run `python3 .github/scripts/test-specsync-archive.py` here too; the hosted `CI gate fixtures` job also runs it.
- The measurement is the hosted `SpecSync Archive (K/4)` jobs on this branch: each job's duration, and each shard's walked and authenticated counts.
- The other jobs run #99's old product code. They are outside this proof.

## Where these lessons go

- `specs/cli/context.md`
