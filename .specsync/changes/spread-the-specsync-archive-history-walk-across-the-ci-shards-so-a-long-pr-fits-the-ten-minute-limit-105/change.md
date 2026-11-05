---
id: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
state: implementing
type: bug_fix
base_commit: a688800ee100a77c50119e2ce17530c64a1e0fe8
---

# Spread the SpecSync Archive history walk across the CI shards so a long PR fits the ten-minute limit (105)

## Intent

Spread the SpecSync Archive history walk across the CI shards so a long PR fits the ten-minute limit (105)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Each SpecSync Archive shard walks only its share of the PR revisions (positions K, K+N, ... of the base and rev-list, repeats once), refuses any change in them that was omitted instead of archived, keeps every other check, and reports walked and total revisions with authenticated and total archives. The three new tests fail first, every existing fixture passes, the long #99 candidate is measured again per phase on Linux with 4 CPUs, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within DRIVE-10: the archive inspector keeps every check and the unchanged job limit; no requirement text changes.
