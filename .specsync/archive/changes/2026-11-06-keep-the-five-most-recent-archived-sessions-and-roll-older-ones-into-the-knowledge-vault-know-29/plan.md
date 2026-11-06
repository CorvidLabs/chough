---
change: keep-the-five-most-recent-archived-sessions-and-roll-older-ones-into-the-knowledge-vault-know-29
artifact: plan
---

# Plan

## Approach

One step at done and one generated note kind, behind failing tests.

## Out of scope

KNOW-24, KNOW-25 to KNOW-28.

## Steps

1. Leif approves the scope.
2. Commit the failing tests:
   - `done_keeps_the_five_newest_archived_sessions_and_rolls_the_rest_into_the_vault`;
   - `with_the_librarian_off_archived_sessions_stay`;
   - the session-note path in the union test.
3. Implement the roll-up, the notes and the union line.
4. Update docs/cli.md, the CHANGELOG and a case file.
5. Do a dry count on this repository, then run verify on Linux.

## Risks

See design.

## Constraints consulted

KNOW-29.
