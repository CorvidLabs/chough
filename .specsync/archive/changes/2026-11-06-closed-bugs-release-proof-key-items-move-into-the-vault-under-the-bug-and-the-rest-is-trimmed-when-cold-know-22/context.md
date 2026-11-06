---
change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
artifact: context
---

# Context

## KNOW-22

"A verified, duplicate or won't-fix bug stops holding its observations: its key items (reproduction, failure-first log, the fixing run) move into the vault under the bug's note, and the rest is summarized and trimmed." Leif captured it as written on 2026-11-05.

## Leif's choice: "Approve as proposed"

2026-11-05, 16:34 MST, via the orc. For verified, duplicate and wont-fix bugs, the key items are:
- (a) the fixture under `.chough/`;
- (b) the earliest linked failed observation, with its artifacts;
- (c) for verified bugs, the first linked passed observation after it.

They move to `knowledge/attachments/<bug folder>/`, linked from a minimal bug note. Other linked observations are summarized and trimmed by the normal cold rule. Records keep their status and IDs, and open bugs keep everything.

## On this repository

There are 52 bugs: 45 verified and 7 open. They make 167 observation links, and closed bugs make about 150 of the librarian's protections.

## Found while building

`references()` also serves `chough evidence prune`, which deletes outright and moves nothing into the vault. The release therefore applies only inside the trim's protection rules. `evidence prune` still keeps every bug's links, so a key item is never deleted without being moved.

## Out of scope

- The full bug note (KNOW-27).
- The vault's folders and index (KNOW-28).
- Ledger rotation (KNOW-23).
