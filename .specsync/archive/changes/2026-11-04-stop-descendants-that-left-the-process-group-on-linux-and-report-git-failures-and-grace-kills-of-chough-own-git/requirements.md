---
change: stop-descendants-that-left-the-process-group-on-linux-and-report-git-failures-and-grace-kills-of-chough-own-git
artifact: requirements
---

# Requirements

MODIFIED REQ-cli-011 (see `deltas/cli.md`): on Linux a stop also stops descendants that
left the process group, orphans included; a grace kill after one of Chough's own Git
commands and a failed `git rev-parse HEAD` (other than no repository or no commit) are
reported on stderr. Trace adds LIFE-7.
