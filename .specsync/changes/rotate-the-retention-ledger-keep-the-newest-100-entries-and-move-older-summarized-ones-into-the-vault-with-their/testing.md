---
change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
artifact: testing
---

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
