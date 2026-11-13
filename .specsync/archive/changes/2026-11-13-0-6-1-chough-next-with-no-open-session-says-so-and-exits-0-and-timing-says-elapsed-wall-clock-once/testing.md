---
change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
artifact: testing
---

# Testing

`tests/next.rs` or `tests/sessions.rs`: after the move into done, `chough next` prints the
no-session message naming `chough session new <key>` and exits 0, while `chough status`
refuses. `tests/timing.rs`: the text says elapsed wall-clock once, in its header, and no
note says "read read-only". Red at the test commit, green at the fix.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-021 | `tests/next.rs`: with no open session, next exits 0 naming `chough session new <key>` |
| REQ-history-006 | `tests/sessions.rs`: after the move into done, next exits 0 and status refuses |
| REQ-reporting-001 | `tests/timing.rs`: elapsed wall-clock is said once, in the header |
