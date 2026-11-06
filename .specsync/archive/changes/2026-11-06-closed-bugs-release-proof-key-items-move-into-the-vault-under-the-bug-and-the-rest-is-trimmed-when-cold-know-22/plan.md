---
change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
artifact: plan
---

# Plan

## Approach

Retention, the vault and docs, behind failing tests.

## Out of scope

KNOW-23 to KNOW-28.

## Steps

1. Leif approves the definition; then move to build.
2. Commit the failing tests in tests/reduce.rs:
   - `a_closed_bug_moves_its_key_items_into_the_vault_and_releases_the_rest`, which covers verified, duplicate and wont-fix;
   - `an_open_bug_keeps_every_linked_observation`;
   - the strengthened `a_closed_bug_releases_its_loose_fixture`.
3. Implement the key items, the release in the trim, the moves under the bug, and the bug notes.
4. Update docs/cli.md, docs/FRD.md, the CHANGELOG and a per-change case file.
5. Measure the trim on this repository (dry run), then verify on Linux.

## Risks

- A key item that is still hot or protected stays in `.chough/` until it is cold.

## Constraints consulted

KNOW-22; Leif's "Approve as proposed" (2026-11-05 16:34 MST); REQ-cli-046.
