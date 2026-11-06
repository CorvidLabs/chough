---
change: find-the-legacy-history-anchor-in-sessions-rolled-into-the-vault-or-shelved-141-p1
artifact: tasks
---

# Tasks

- [x] Add the failing test `sessions_still_write_after_the_anchor_holder_rolls_into_the_vault_or_is_shelved`.
- [ ] Make `adopted_anchor` also read `.chough/sessions/shelved/` and the sessions rolled into the vault at `<library-dir>/attachments/*/sessions/*.json`, with the same bound and the same exact anchor match.
- [ ] Add a CHANGELOG line, and run verify on Linux.
