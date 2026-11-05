---
change: move-old-proof-into-the-knowledge-vault-trim-loose-artifacts-of-cold-changes-and-move-key-items-into-knowledge
artifact: plan
---

# Plan

## Approach

A new `src/attachments.rs` holds the key-item rule, attachment paths, first-commit
times for artifacts (one `git log`), and loose-artifact grouping and protection.
`evidence.rs` adds loose groups to the summary candidates, computes each candidate's
moves, and performs the moves in the existing ledger-first apply with restore.
`library.rs` lists loose artifacts and links attachments in each archive note.

## Out of scope

`chough evidence prune` (its protections are unchanged), the hot window of 3, and moving
proof of hot or current changes.

## Steps

1. Add the failing tests.
2. Implement `src/attachments.rs` and the summary and apply hooks.
3. Write the archive note sections.
4. Update the docs and skill reference.
5. Fold in the lessons.
6. Run and measure the trim on this repository.

## Risks

- Removing artifacts: they must be committed identically at HEAD, ledgered first, and
  restored on any validity change.
- Moving files: the copy is written before the original is removed, and a restore
  deletes the copy.

## Constraints consulted

KNOW-8, KNOW-9, KNOW-10, KNOW-11 and REQ-history-001; KNOW-3 (the vault points to
sources; attachments are history, like snapshots).
