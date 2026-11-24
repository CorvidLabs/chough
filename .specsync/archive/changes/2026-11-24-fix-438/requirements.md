---
change: fix-438
artifact: requirements
---

# Requirements

Covers REQ-cli-061: `chough gate record` closes all the named gates or
none. Observable outcome: when any named gate's close would be refused
after the existing pre-checks — the reachable case is a later save over
the session record bound — the whole record is refused with nothing
written, and no earlier gate is left closed. The native step still runs
at most once, before any gate is closed.