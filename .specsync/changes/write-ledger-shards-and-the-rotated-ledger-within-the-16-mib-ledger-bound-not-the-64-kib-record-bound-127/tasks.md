---
change: write-ledger-shards-and-the-rotated-ledger-within-the-16-mib-ledger-bound-not-the-64-kib-record-bound-127
artifact: tasks
---

# Tasks

- [x] Add the failing test `a_rotation_larger_than_the_record_bound_still_rotates`: 300 padded entries, so both the shard and the kept ledger exceed 64 KiB.
- [ ] Write shards and the rotated ledger with `write_replace_bounded` at the 16 MiB ledger bound.
- [ ] Rerun the done-move rotation on this repository, then run verify on Linux.
