---
change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
artifact: context
---

# Context

## KNOW-23

"The retention ledger stays small: entries for proof an archive note already summarizes move into the vault with that archive, and the ledger keeps only recent entries." Leif captured it as written on 2026-11-05.

## Leif's choice

2026-11-05, 18:24 MST, via the orc: "Keep newest N, not 14 days". The rest is as proposed:
- shards under attachments;
- readers take the union, ignoring duplicates;
- a 16 MiB bound on each file;
- union merge;
- writes go to the shards first.

Leif approves N in this scope.

## The ledger on main (`4c68cca7`, after #121)

789 entries, 1,633,015 bytes: 740 entries and 49 `restored` events.

How much stays in `.chough/pruned.jsonl` for each N (newest entries kept):

| N | Bytes kept |
|---|---|
| 50 | 141,712 |
| **100 (proposed)** | **281,631** |
| 200 | 542,432 |
| 300 | 841,601 |
| 400 | 1,005,706 |

The ten trims recorded so far added these entries, newest first: 8, 101, 38, 1, 7, 84, 42, 367, 7 and 85. So N=100 keeps at least the latest done's entries, except after a trim larger than 100. Entries span 20 archives. The largest shard would hold 517 entries (1,253,652 bytes), well under 16 MiB.

## Found while scoping

- The vault's archive notes are generated from the ledger, and labels, bug views and `evidence prune` read it too. So every reader must read the shards.
- KNOW-22's `knowledge/bugs/*.md` notes have no `merge=union` line. That is a separate fix and is not in this change.

## Out of scope

KNOW-24 to KNOW-28, which wait on Leif's priority change of 18:22 MST.
