---
change: find-the-legacy-history-anchor-in-sessions-rolled-into-the-vault-or-shelved-141-p1
artifact: testing
---

# Testing

- tests/sessions.rs `sessions_still_write_after_the_anchor_holder_rolls_into_the_vault_or_is_shelved`:
  - the adopting session moves to `knowledge/attachments/legacy/sessions/legacy.json`, and in a second run to `.chough/sessions/shelved/legacy.json`;
  - another session's `stage advance` succeeds, and the legacy log is unchanged;
  - a changed legacy log is still refused.

  It fails first, with the P1 refusal.
- `a_session_started_after_legacy_adoption_writes_events_and_keeps_the_log_guarded` still passes.
- `fledge lanes run verify` passes on Linux.
