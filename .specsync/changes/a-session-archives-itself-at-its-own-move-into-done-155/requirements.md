---
change: a-session-archives-itself-at-its-own-move-into-done-155
artifact: requirements
---

# Requirements

REQ-history-006 (MODIFIED, in `deltas/history.md`) gains the done step:
- At a successful move into done, after REQ-cli-041, REQ-history-011, REQ-cli-043 and REQ-cli-049, the active session moves to `.chough/sessions/archive/<key>.json` unchanged and the active pointer is cleared. It is reported as `applied.session_archive` with status `archived`.
- A session with unfolded inbox entries stays open with its pointer (`kept_open`, with the count).
- Without sessions, the step reports `skipped`.
- The step never undoes the move into done.
- Afterwards `next` and `status` refuse, naming `chough session new <key>`.

Two acceptance criteria are added. DRIVE-13 joins the trace. Every other sentence and criterion is unchanged.
