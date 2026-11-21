---
change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
artifact: requirements
---

# Requirements

No requirement text changes. REQ-cli-070 already says that one main stage applies `chough-stage:<stage>` and removes every other `chough-stage:*` label. This change makes the script do that when the matching label is already present.
