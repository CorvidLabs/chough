---
change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
artifact: testing
---

# Testing

## tests/reduce.rs

- `a_closed_bug_moves_its_key_items_into_the_vault_and_releases_the_rest`, for verified, duplicate and wont-fix:
  - f1 (the earliest failure, with its log) moves to `knowledge/attachments/BUG-0002/`;
  - p1 (the first pass after it) moves there only for the verified bug;
  - x1 is trimmed and ledgered;
  - f2 and n1 keep their KNOW-17 protections;
  - the bug record is unchanged;
  - `knowledge/bugs/BUG-0002.md` links what moved.

  It fails first: all three runs are protected by `BUG-0002 <status>`.
- `an_open_bug_keeps_every_linked_observation`: a guard; it passes before and after.
- `a_closed_bug_releases_its_loose_fixture`: now also asserts the fixture's bytes at `knowledge/attachments/BUG-0001/artifacts/bug-repro.log`. It fails first: the fixture is deleted, not moved.

## tests/evidence.rs

- The `evidence prune` test still asserts `BUG-0001 verified` as a protection.
- The shared `knowledge_fixture` uses an open bug.

## Lanes

`fledge lanes run verify` passes on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-048 | tests/reduce.rs `a_closed_bug_moves_its_key_items_into_the_vault_and_releases_the_rest`, `a_closed_bug_releases_its_loose_fixture`, `an_open_bug_keeps_every_linked_observation`; tests/evidence.rs `every_protection_reason_is_named` (evidence prune keeps a verified bug's link) |
