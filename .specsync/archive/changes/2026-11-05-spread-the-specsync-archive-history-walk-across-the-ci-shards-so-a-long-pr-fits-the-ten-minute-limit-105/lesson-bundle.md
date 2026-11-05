# Lesson bundle: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Spread the SpecSync Archive history walk across the CI shards so a long PR fits the ten-minute limit (105)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: .github/scripts/check-specsync-archive.py, .github/scripts/test-specsync-archive.py, .github/workflows/ci.yml, docs/ci.md, CHANGELOG.md
- **Acceptance**: Each SpecSync Archive shard walks only its share of the PR revisions (positions K, K+N, ... of the base and rev-list, repeats once), refuses any change in them that was omitted instead of archived, keeps every other check, and reports walked and total revisions with authenticated and total archives. The three new tests fail first, every existing fixture passes, the long #99 candidate is measured again per phase on Linux with 4 CPUs, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `188f96b2fc6a387dfb32e9d7bd6c1f984c3face6`
- Base commit: `a688800ee100a77c50119e2ce17530c64a1e0fe8`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #105 (P2): on the final #99 head `6a9bfad6`, against trusted base `8b7f8bf6` (361 PR revisions, 86 archives), hosted job `SpecSync Archive (1/4)` was cancelled at its 10-minute limit. #103 split archive authentication across four shards, but shard 1 alone still walks every PR revision.

## Measurement before the change

- Inspector from main `a688800e`, SpecSync 6.2.0 built from source.
- Docker `chough-linux:3`, Linux aarch64, `--cpuset-cpus=0-3`, cold clone of the candidate.

| Phase | Shard 1/4 | Shard 2/4 |
|---|---|---|
| History walk: 362 historical `change list` reads | 234.8 s (about 0.65 s each) | none |
| Checkouts | 4.3 s | none |
| Archive scans | 3.6 s | none |
| Authenticating 22 archives (wall) | 22.5 s | 4.5 s |
| Audit | 0.4 s | 0.2 s |
| **Total** | **268.6 s** | **4.9 s** |

The walk is about 87% of shard 1. With SpecSync 6.2.0, authentication is no longer the bulk. The hosted runner is at least 2.2 times slower: it was cancelled at 600 s.

## Measurement after the change

Same conditions, inspector with `shard_revisions`; logs in `.chough/artifacts/archive-history-shards-105/`.

| Shard | Revisions walked | Walk | Archives authenticated | Authentication wall | Total |
|---|---|---|---|---|---|
| 1/4 | 91 | 64.2 s | 22 | 4.6 s | 72.7 s |
| 2/4 | 91 | 63.4 s | 22 | 6.1 s | 73.5 s |
| 3/4 | 90 | 71.2 s | 21 | 6.1 s | 81.9 s |
| 4/4 | 90 | 78.6 s | 21 | 5.8 s | 88.8 s |

Together: 362 of 362 revisions (361 plus the base), and 86 of 86 archives. The slowest shard is 88.8 s, against 268.6 s before. At the hosted slowdown implied by the cancellation (at least 2.2 times), that is about 3.3 minutes against a 10-minute limit. Hosted proof on this long candidate needs a hosted run on it; this PR's own hosted shards cover only its own short history.

Skipping revisions with an identical `.specsync` tree was considered. Only 103 of the 361 repeat a tree (258 are distinct), so it saves little. It would also rely on native `list` reading nothing outside that tree. It was not chosen.

Human intent: existing DRIVE-10. Leif's agreed order, via the orc, 2026-11-05: #105 after #114, then the reduction work.

Out of scope:
- SpecSync's own per-read cost;
- the 10-minute limit, which stays;
- branch protection.

## From the change's design.md

# Design

## Approach

Split the PR history walk across shards the same way authentication is split.

- `shard_revisions(revisions, (K, N))` takes the base plus `rev-list --reverse base..HEAD`, with repeats removed in first-seen order, and returns positions K, K+N, and so on. The N shards together walk every revision exactly once.
- Each shard collects the historical roster and archive IDs of its revisions as obligations. It refuses with "omitted instead of archived" if any obligation is missing from the current archives. That check runs before any native status read, as today.
- In every passing shard the obligations equal the current archives. The archive slice each shard authenticates is therefore the same partition as before, so every archive is still authenticated exactly once.
- Every shard still runs the version, protocol, policy, audit and active-roster checks, the trusted-base, merge-base and full-history checks, the symlink and inventory refusals, and the before/after fingerprint.
- A sharded run reports "Shard K/N: walked R of T PR revision(s); A of B archive(s) authenticated", so the job logs prove coverage. The default 1/1 run walks every revision and prints the same message as before.

## Out of scope

- Skipping revisions or archives.
- Trusting labels.
- Changing native checks, the shard count or the job limit.

## Risks

- A change that exists only in revisions another shard walks is found by that shard alone. That is why the hosted check needs all four shards green, as it already does for authentication, with `fail-fast: false`.

## Constraints consulted

DRIVE-10; the docs/ci.md archive section; the specs/cli/context.md lessons on the archive inspector (#80, #103).

## From the change's testing.md

# Testing

## .github/scripts/test-specsync-archive.py (`fledge run ci-archive-fixtures`)

- `test_shards_partition_the_pr_revisions_exactly_once_in_order`: for N from 1 to 5, the shards' revisions are disjoint and together cover every revision in order, with repeats walked once.
- `test_shards_split_authentication_and_the_history_walk`:
  - the archived fixture reports "walked R of T PR revision(s)" with ceil(T/2) and floor(T/2), and 1 and 0 of 1 archives;
  - a draft fails in both shards;
  - a removed archive fails in both shards;
  - 3/2 is refused.
- `test_the_shard_that_walks_a_revision_refuses_a_change_omitted_from_it`: in history base → draft introduced → draft removed, shard 2/2 and the 1/1 run refuse "omitted instead of archived", and shard 1/2 passes, having walked 2 of 3.

The first two fixture tests and the unit test fail before the change: there is no `shard_revisions`, the output has no revision counts, and shard 2/2 passes. Every existing executor and fixture test still passes; these cover drafts, omitted, corrupt and tampered archives, shallow history, bad base, malformed native replies, timeouts, output bounds and fingerprints.

## Performance

Run the profiler again on the long #99 candidate under the same conditions as before. Report each shard's setup, history walk and authentication times and its revision and archive counts.

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
