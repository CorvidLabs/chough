---
change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
artifact: design
---

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
