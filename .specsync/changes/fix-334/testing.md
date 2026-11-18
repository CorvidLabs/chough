---
change: fix-334
artifact: testing
---

# Testing

`tests/session_pin.rs::a_pinned_session_before_any_session_names_session_new`, red on main:
- With `CHOUGH_SESSION` set and no session open, `chough next` refuses, naming `CHOUGH_SESSION` and `chough session new <key>`, with the tree unchanged.
- With `--session`, the refusal names `--session`.

Case `TC-cli-a-pinned-session-before-any-session-names-session-new` binds it.
