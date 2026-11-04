---
change: remove-the-history-events-a-command-published-when-its-save-is-refused-because-the-session-file-vanished-and-test-that
artifact: testing
---

# Testing

- tests/sessions.rs `session_use_waits_for_a_running_stage_advance` (REQ-history-008):
  while `stage advance` on alpha holds the lock (its native read is slowed), `session use
  beta` does not finish before it; afterwards alpha's history has no unselected event.
  This guards #77's lock and passes at 75a732d by design.
- tests/sessions.rs `a_save_refused_because_the_session_vanished_leaves_no_event`
  (REQ-history-008): a stub `specsync` moves alpha's session file away during the native
  read; `stage advance` refuses with "nothing was saved" and leaves no new event file in
  alpha's cycle. Fails at 75a732d.
- src/sessions.rs unit test `a_vanish_after_a_successful_save_keeps_the_saved_event`
  (REQ-history-008, #96 review): fails at bf06289's src, which deleted the saved event.
- `fledge lanes run verify` passes on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-008 | tests/sessions.rs `session_use_waits_for_a_running_stage_advance`, `a_save_refused_because_the_session_vanished_leaves_no_event`; src/sessions.rs `a_vanish_after_a_successful_save_keeps_the_saved_event` |
