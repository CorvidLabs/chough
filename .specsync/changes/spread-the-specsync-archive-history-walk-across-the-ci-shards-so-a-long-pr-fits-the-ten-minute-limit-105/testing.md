---
change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
artifact: testing
---

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
