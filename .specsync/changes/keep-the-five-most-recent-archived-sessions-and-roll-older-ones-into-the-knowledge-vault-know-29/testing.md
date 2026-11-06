---
change: keep-the-five-most-recent-archived-sessions-and-roll-older-ones-into-the-knowledge-vault-know-29
artifact: testing
---

# Testing

## tests/reduce.rs

- `done_keeps_the_five_newest_archived_sessions_and_rolls_the_rest_into_the_vault`:
  - seven archived sessions;
  - s1 to s5 stay;
  - s0 (a tie, broken by key) and s6 (no change) move byte for byte into `knowledge/attachments/<key>/sessions/<key>.json`;
  - their notes name the objective and link the attachment;
  - `library index --check` and `session list` pass.
- `with_the_librarian_off_archived_sessions_stay`: no session moves, and the step reports `skipped`.

## tests/vault.rs

`only_generated_knowledge_notes_merge_by_union` checks that a session note is `union`. All three tests fail first.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-011 | tests/reduce.rs `done_keeps_the_five_newest_archived_sessions_and_rolls_the_rest_into_the_vault`, `with_the_librarian_off_archived_sessions_stay` |
| REQ-cli-051 | tests/reduce.rs `done_keeps_the_five_newest_archived_sessions_and_rolls_the_rest_into_the_vault`; tests/vault.rs `only_generated_knowledge_notes_merge_by_union` |

## Lanes

`fledge lanes run verify` passes on Linux.
