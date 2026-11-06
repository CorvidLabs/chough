---
change: write-ledger-shards-and-the-rotated-ledger-within-the-16-mib-ledger-bound-not-the-64-kib-record-bound-127
artifact: context
---

# Context

The done move of #127 (KNOW-23) ran on this repository after Leif's review and "Record and merge" (2026-11-05, 18:45 MST). Its rotation refused with "managed file exceeds 65536 bytes: knowledge/attachments/claude-host-0be606ee/pruned.jsonl". Rotation wrote shards and the ledger with `records::write_replace`, which bounds a managed record at 64 KiB. REQ-cli-049 sets 16 MiB, and the ledger alone is 1.6 MB, so every real rotation refused. The tests used only small shards.

Before refusing, one small shard (`build-the-f9b8e664`, 6 lines) had been written; it was committed as `ec4bf586`. Readers count those lines once, by the leftover rule of REQ-cli-049, so nothing was lost or double-counted. The next rotation drops them from the ledger. #127 is held unmerged until this fix is reviewed.
