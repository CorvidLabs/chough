---
change: a-session-whose-native-change-is-archived-is-finished-and-abandoned-work-closes-with-a-reason-163-drive-16
artifact: testing
---

# Testing

New tests in tests/sessions.rs. A `specsync` stub on PATH records any call and fails, so each test also asserts there was no native read.
- `a_session_whose_native_change_is_archived_is_finished_without_a_native_read`:
  - alpha's native change is archived and beta's is open, with no pointer;
  - `checkpoint --decision` lands in beta, and alpha is unchanged;
  - `session new gamma` archives alpha byte for byte and leaves beta open.
- `session_use_archives_a_finalized_session_unless_its_inbox_has_entries`: `session use beta` archives the finalized alpha and keeps the finalized gamma, which has an unfolded entry, open under `kept_open`.
- `session_close_archives_abandoned_work_with_its_reason`:
  - a blank reason refuses with nothing changed;
  - closing alpha archives it with the reason in its decisions and leaves beta active;
  - closing it again refuses as archived, and a missing key refuses;
  - closing the active beta clears the pointer, and `session list` shows both archived.
- `session_close_refuses_a_session_with_unfolded_inbox_entries`: it names `chough session use alpha` and `chough inbox fold`, and nothing moves.

All four fail first: an unknown `close` subcommand, nothing archived, and "several open sessions (alpha, beta)". The existing session tests stay unchanged. A case binds the tests. `fledge lanes run verify` passes on Linux.
