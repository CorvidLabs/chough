---
change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
artifact: tasks
---

# Tasks

- [x] Add failing tests:
  - a unit test that the revision split partitions in order and walks repeats once;
  - a fixture where only shard 2/2 walks the revision holding an omitted draft;
  - revision counts in the shard output.
- [ ] Add `shard_revisions` and walk only the shard's revisions; report the walked and total revision counts.
- [ ] Update the ci.yml comment, the docs/ci.md archive section and the CHANGELOG.
- [ ] Measure the long #99 candidate again on Linux with 4 CPUs; run the fixtures and the verify lane.
