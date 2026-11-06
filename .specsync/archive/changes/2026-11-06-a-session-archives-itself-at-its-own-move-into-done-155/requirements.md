---
change: a-session-archives-itself-at-its-own-move-into-done-155
artifact: requirements
---

# Requirements

REQ-history-006 (MODIFIED, in `deltas/history.md`) gains the done step:
- At a successful move into done, after REQ-cli-041, REQ-history-011, REQ-cli-043 and REQ-cli-049, the active session moves to `.chough/sessions/archive/<key>.json` unchanged and the active pointer is cleared. It is reported as `applied.session_archive` with status `archived`.
- A session with unfolded inbox entries stays open with its pointer (`kept_open`, with the count).
- A session for which release is authorized at the move into done stays open for the release (`kept_open`), and archives itself at the move from release into done.
- Without sessions, the step reports `skipped`.
- The step never undoes the move into done.
- Afterwards `next` and `status` refuse, naming `chough session new <key>`.

Three acceptance criteria are added. DRIVE-13 joins the trace. Every other sentence and criterion is unchanged.

REQ-cli-049 (MODIFIED, in `deltas/cli.md`): one sentence and one acceptance criterion. In sessions mode the ledger rotation runs before the active session archives itself, the last done step. No other cli text changes.
