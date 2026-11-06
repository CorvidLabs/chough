# Lesson bundle: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Rotate the retention ledger: keep the newest 100 entries and move older summarized ones into the vault with their archive (KNOW-23)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/evidence.rs, src/library.rs, src/stage.rs, src/attachments.rs, tests/reduce.rs, tests/vault.rs, .gitattributes, docs/cli.md, CHANGELOG.md
- **Acceptance**: At done the retention ledger keeps its newest 100 entries plus entries without an archive or note; older entries move with their restores into knowledge/attachments/<first archive folder>/pruned.jsonl, shards first, nothing lost or duplicated, reported as applied.ledger_rotation; every ledger reader reads shards then the hot ledger ignoring duplicate lines, so labels, bug views and notes are unchanged; shards union-merge and are not listed as attachments. The new tests fail first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `5837c9b4954b82341447380f605289193978b13a`
- Base commit: `4c68cca7275b3ab8f49af94cc925bf547842757a`
- Verified by: `specsync check --spec cli`

## From the change's context.md

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

## From the change's design.md

# Design

## Approach

- **Readers:** a single `ledger_lines(root)` in src/evidence.rs yields each shard's lines, the shards in path order, then the hot ledger's lines. Each shard copy of a line cancels one identical copy in the hot ledger: that is the leftover of an interrupted rotation, and the next rotation drops it. Identical lines within one file are all kept, because a prune, restore and prune again in one second write two identical entries. Each file is bounded at 16 MiB. `read_ledger`, `ledger_entries` and `ledger_records` use it, so labels, bug views, `evidence prune`, the summary and the notes all see moved entries.
- **Rotation:** `rotate_ledger` runs in src/stage.rs after `refresh_at_done`. It pairs each `restored` event with the entry it follows. It keeps the newest 100 entries, plus entries with no archive or no note. It appends the moved lines to their shards, then replaces the hot file. It reports `applied.ledger_rotation`.
- **Attachments:** `attachments_of` in src/attachments.rs skips a top-level `pruned.jsonl`, so notes don't list it as an attachment.
- **Merging:** `.gitattributes` gains `knowledge/attachments/*/pruned.jsonl merge=union`.

## Risks

- A trim larger than 100 entries rotates part of itself straight away. Its entries stay readable in the shard.
- A crash between the two writes leaves lines held in both a shard and the ledger. Readers count each of them once, and the next rotation drops them from the ledger.
- Amended after approval: the approved "ignore any identical line" rule dropped a legitimate re-prune in the same second. `archive_summary_apply_is_authorized_ledgered_and_restores_on_validity_change` showed it, so the rule now cancels only cross-file leftovers.

## Constraints consulted

- REQ-cli-041, REQ-cli-043, REQ-cli-046, REQ-cli-047.
- KNOW-13, KNOW-23, DRIVE-12.
- The 16 MiB ledger bound.

## From the change's testing.md

# Testing

## tests/reduce.rs

- `done_keeps_the_newest_ledger_entries_and_moves_the_rest_into_the_vault`:
  - 105 old a1 entries, a restore of the oldest, and an entry without a note;
  - the hot ledger keeps 101 entries;
  - the shard holds the rest and the restore;
  - no line is lost or duplicated;
  - the `moved` count matches.

  It fails first: there is no `ledger_rotation`.
- `moved_ledger_entries_are_still_read`:
  - after rotation, an open bug's link to a moved ID shows `pruned`;
  - a1's note lists it;
  - `library index --check` passes.

  It fails first at the rotation status.

- `an_interrupted_rotation_leftover_is_read_once`: a leftover copy in the hot ledger of a shard entry, placed after that entry's restore, must not make the restored ID pruned again.

## tests/vault.rs

`only_generated_knowledge_notes_merge_by_union` checks the shard path is `union`. It fails first: the attribute is `unspecified`.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-049 | tests/reduce.rs `done_keeps_the_newest_ledger_entries_and_moves_the_rest_into_the_vault`, `moved_ledger_entries_are_still_read`, `an_interrupted_rotation_leftover_is_read_once`; tests/evidence.rs `archive_summary_apply_is_authorized_ledgered_and_restores_on_validity_change` (a same-second re-prune stays pruned); tests/vault.rs `only_generated_knowledge_notes_merge_by_union` |

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
