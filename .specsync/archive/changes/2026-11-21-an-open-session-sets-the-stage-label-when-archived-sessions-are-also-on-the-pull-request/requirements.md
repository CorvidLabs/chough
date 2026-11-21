---
change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
artifact: requirements
---

# Requirements

REQ-cli-070 is modified in `deltas/cli.md`. The rest of that requirement is copied there in full.

The new sentences: when any introduced file is still an open session, the script chooses the stage from those open sessions and ignores archived stories. When no open session remains, it chooses from the archived stories. The one-distinct-main-stage rule then applies to the chosen set. Trace adds LIFE-32. One new acceptance criterion covers an archived `done` plus an open `intent`, and an archived-only `done`. The existing mixed-stage criterion now says two open sessions.
