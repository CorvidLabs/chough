# Lesson bundle: write-ledger-shards-and-the-rotated-ledger-within-the-16-mib-ledger-bound-not-the-64-kib-record-bound-127

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Write ledger shards and the rotated ledger within the 16 MiB ledger bound, not the 64 KiB record bound (127)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, tests/reduce.rs
- **Acceptance**: A rotation whose shard and kept ledger each exceed 64 KiB writes both within the 16 MiB ledger bound and reports rotated, losing no line; on this repository the rotation leaves 100 entries in .chough/pruned.jsonl. The test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `a1d9b5ff7d6c98485dbdfc9ba3b498db09cdd435`
- Base commit: `ec4bf5861d0e13b6ff32731b9e68c0a61944a3d1`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The done move of #127 (KNOW-23) ran on this repository after Leif's review and "Record and merge" (2026-11-05, 18:45 MST). Its rotation refused with "managed file exceeds 65536 bytes: knowledge/attachments/claude-host-0be606ee/pruned.jsonl". Rotation wrote shards and the ledger with `records::write_replace`, which bounds a managed record at 64 KiB. REQ-cli-049 sets 16 MiB, and the ledger alone is 1.6 MB, so every real rotation refused. The tests used only small shards.

Before refusing, one small shard (`build-the-f9b8e664`, 6 lines) had been written; it was committed as `ec4bf586`. Readers count those lines once, by the leftover rule of REQ-cli-049, so nothing was lost or double-counted. The next rotation drops them from the ledger. #127 is held unmerged until this fix is reviewed.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
