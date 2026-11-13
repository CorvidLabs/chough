---
change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
artifact: design
---

# Design

`chough next` with no open session prints the no-session message naming `chough session
new <key>` and exits 0; `chough status` keeps refusing, since a full report needs a
session. `chough timing`'s text drops the label from its gate-wait line, keeping it once in
the header, and the native-record note reads read-only once.
