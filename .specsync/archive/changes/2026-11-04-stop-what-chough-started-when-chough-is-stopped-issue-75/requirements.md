---
change: stop-what-chough-started-when-chough-is-stopped-issue-75
artifact: requirements
---

# Requirements

MODIFIED REQ-cli-011 (see `deltas/cli.md`). On Unix, when Chough is stopped with SIGINT,
SIGTERM or SIGHUP while a case binding or native read it started is running, it stops
that command and its descendants and ends by the signal without recording an
observation, artifact or checkpoint change; an inherited ignored signal stays ignored.
Trace adds FLOW-14.
