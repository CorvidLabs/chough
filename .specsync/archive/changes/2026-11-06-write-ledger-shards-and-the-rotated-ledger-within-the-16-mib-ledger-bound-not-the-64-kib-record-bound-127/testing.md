---
change: write-ledger-shards-and-the-rotated-ledger-within-the-16-mib-ledger-bound-not-the-64-kib-record-bound-127
artifact: testing
---

# Testing

- tests/reduce.rs `a_rotation_larger_than_the_record_bound_still_rotates`:
  - 300 entries with 600-byte reasons;
  - rotation reports `rotated`;
  - the shard and the kept ledger are each larger than 65,536 bytes;
  - no line is lost.

  It fails first with "managed file exceeds 65536 bytes: knowledge/attachments/a1/pruned.jsonl".
- The other KNOW-23 tests still pass.
- On this repository, a rotation reports `rotated` and leaves 100 entries in `.chough/pruned.jsonl`.
- `fledge lanes run verify` passes on Linux.
