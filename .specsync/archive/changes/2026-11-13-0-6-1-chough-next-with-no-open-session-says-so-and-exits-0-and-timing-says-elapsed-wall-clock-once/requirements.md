---
change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
artifact: requirements
---

# Requirements

REQ-cli-021 and REQ-history-006 (modified): with no open session, `chough next` says
there is no open session and names `chough session new <key>`, exiting 0; `chough status`
still refuses with that message. REQ-reporting-001 (modified): no gate-wait, segment or
estimate line repeats the elapsed wall-clock label the header gives.
