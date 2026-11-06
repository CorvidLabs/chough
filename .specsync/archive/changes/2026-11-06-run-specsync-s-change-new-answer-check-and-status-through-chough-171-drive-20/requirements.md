---
change: run-specsync-s-change-new-answer-check-and-status-through-chough-171-drive-20
artifact: requirements
---

# Requirements

ADDED REQ-cli-057 (see `deltas/cli.md`): `chough change new`, `answer`, `check` and `status` run the matching SpecSync step and pass its output, refusals and exit code through unchanged, write nothing under `.chough/`, keep the native read bound only for the status read, stop the child on SIGINT, SIGTERM or SIGHUP, and refuse with the install command when `specsync` cannot be started.

No existing requirement changes. Trace: DRIVE-20, FLOW-14, FLOW-1.
