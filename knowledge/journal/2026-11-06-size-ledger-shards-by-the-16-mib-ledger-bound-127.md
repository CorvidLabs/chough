---
id: 2026-11-06-size-ledger-shards-by-the-16-mib-ledger-bound-127
title: Size ledger shards by the 16 MiB ledger bound (#127)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T02:05:10Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-write-ledger-shards-and-the-rotated-ledger-within-the-16-mib.md
  - .specsync/archive/changes/2026-11-06-write-ledger-shards-and-the-rotated-ledger-within-the-16-mib-ledger-bound-not-the-64-kib-record-bound-127/
record: .chough/archive/sessions/2026-11-06-write-ledger-shards-and-the-rotated-ledger-within-the-16-mib.md
actor:
  - agent: claude-code
    terminal: t382
---
# Size ledger shards by the 16 MiB ledger bound (#127)

Ledger shards and the rotated ledger are now written within the 16 MiB ledger bound rather than the 64 KiB record bound, closing out #127 under criterion KNOW-23.

The native change passed specsync change check. The test that a rotation larger than the record bound still rotates passed on Linux after failing first, and the full suite stood at 454 passed and 0 failed.

The maintainer approved the size-cap fix and recorded a passing review. The change was archived and the story closed done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-write-ledger-shards-and-the-rotated-ledger-within-the-16-mib.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-write-ledger-shards-and-the-rotated-ledger-within-the-16-mib-ledger-bound-not-the-64-kib-record-bound-127/)
