---
change: fix-334
artifact: design
---

# Design

- `main` passes `pin_session` the source of the name: `--session` or `CHOUGH_SESSION`.
- When `sessions::open_ids` is empty, the refusal names that source and the value, says no session is open yet, and gives `chough session new <key>`.
- Otherwise it is the existing refusal, naming the source and `chough session list`.
- Nothing is written either way.
