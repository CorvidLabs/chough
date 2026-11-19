---
change: fix-271
artifact: testing
---

# Testing

`tests/session_pin.rs`:
- `a_pinned_shell_writes_its_own_session_whatever_the_pointer_says`: two open sessions, with the pointer on one. `evidence add --session <other>` and `evidence add` with `CHOUGH_SESSION=<other>` each record into the other session, the pointer is unchanged, and `next --session <other>` shows that session. Fails on main.
- `a_pin_to_a_session_that_is_not_open_refuses_with_nothing_written`: `--session` and `CHOUGH_SESSION` naming an unknown or archived session refuse with every file unchanged. Fails on main.
- `init_and_inbox_add_keep_their_session_and_session_use_ignores_the_variable`.

`tests/integrations.rs` keeps checking the templates' versions in `integrations/templates.toml`. Case `TC-cli-a-pinned-shell-writes-its-own-session` binds the first test.
