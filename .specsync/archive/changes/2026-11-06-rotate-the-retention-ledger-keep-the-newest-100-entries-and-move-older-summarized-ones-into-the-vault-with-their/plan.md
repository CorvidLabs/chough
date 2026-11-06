---
change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
artifact: plan
---

# Plan

## Approach

One reader and one rotation step, behind failing tests.

## Out of scope

- KNOW-24 to KNOW-28.
- The `knowledge/bugs/*.md` union line, a separate fix.

## Steps

1. Leif approves the definition, including N=100.
2. Commit the failing tests:
   - `done_keeps_the_newest_ledger_entries_and_moves_the_rest_into_the_vault`;
   - `moved_ledger_entries_are_still_read`;
   - the shard path in `only_generated_knowledge_notes_merge_by_union`.
3. Implement the union reader, the rotation, the attachments filter and `.gitattributes`.
4. Update docs/cli.md, docs/FRD.md, the CHANGELOG and a case file.
5. Do a dry count on this repository, then run verify on Linux.

## Risks

See design.

## Constraints consulted

KNOW-23; Leif's "Keep newest N, not 14 days" (2026-11-05 18:24 MST).
