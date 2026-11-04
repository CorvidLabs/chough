---
change: serialize-checkpoint-writers-with-an-inter-process-lock-and-remove-session-temps-left-by-killed-writers-issues-62-and
artifact: context
---

# Context

Kimi's second lab review (2026-11-03, Chough 7bd1b62, round 2 items 1 and 3):

- Three `chough checkpoint` lanes run at once in one clone all exited 0 with success,
  but only the last lane's decision and native change survived. Reproduced on main
  4daf861 with six writers: one decision of six survived. Filed as
  https://github.com/CorvidLabs/chough/issues/62.
- A `chough checkpoint` killed with SIGKILL partway through left
  `.chough/.session.json.<pid>.tmp` (valid JSON). The session itself stayed valid, but
  nothing ever removed the temp. Reproduced on 4daf861 at a 250 microsecond kill delay;
  the temp survived later `status` and `checkpoint` calls. Filed as
  https://github.com/CorvidLabs/chough/issues/64.

Cause: `write_replace_bounded` writes a create-new temp, syncs it and renames it, so
each write is atomic, but every command reads the checkpoint, changes it in memory and
replaces it with nothing serializing the read-modify-write, so the last rename wins.
The temp is removed on an error return but not when the process is killed.

The two fixes are one change because the cleanup is only safe under the lock: every
session writer holds the lock through its rename, so a session temp found while holding
the lock belongs to a writer that is gone. Related but separate: #30 (record identity
across parallel agents) and #63 (unknown --native-change), handled on its own branch.
