---
change: keep-the-checkpoint-writer-lock-in-the-project-so-every-temp-directory-shares-it-name-the-file-being-written-when-busy
artifact: requirements
---

# Requirements

ADDED REQ-cli-040 (see `deltas/cli.md`): serialized checkpoint writers with one lock per
project in `.chough/local/checkpoint.lock`, a busy refusal naming the file being
written, and the legacy writer's refusal after adoption. Trace: DRIVE-11, DRIVE-3.

ADDED REQ-history-007 (see `deltas/history.md`): `.chough/local/` holds only Git-ignored
local state, the active-session pointer and the checkpoint lock; a project without
sessions gains no tracked or session file. Trace: DRIVE-3, DRIVE-11.
