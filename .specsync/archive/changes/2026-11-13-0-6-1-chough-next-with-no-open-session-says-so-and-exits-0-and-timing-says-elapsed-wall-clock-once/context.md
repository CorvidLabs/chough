---
change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
artifact: context
---

# Context

Chough 0.6.0's fourth fresh-eyes round (t38n) found two output items, deferred to 0.6.1
by Leif. F10: `chough next` with no open session, the normal state after the move into
done, prints "Error: no open session" and exits 1. F5: `chough timing` still prints
"(elapsed wall-clock)" on the human-gate line, though it should say so once in its
header, and a note reads "read read-only". On 2026-11-13T02:03Z Leif chose "Yes, start
(Recommended)" for 0.6.1 as six small PRs; this is one. Bugs:
`.chough/bugs/2026-11-12-chough-next-with-no-open-session-exits-1-with-error-no-open.md`
and `.chough/bugs/2026-11-12-chough-timing-still-says-elapsed-wall-clock-on-the-human-gate.md`.
