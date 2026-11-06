---
change: a-session-whose-native-change-is-archived-is-finished-and-abandoned-work-closes-with-a-reason-163-drive-16
artifact: context
---

# Context

Issue #163, filed at the orc's request on 2026-11-06: `chough next` refuses on several open sessions when their native changes are already archived. A move into done that timed out after SpecSync finalized its change leaves the session open at accept. Every other worktree without a pointer then refuses with "several open sessions". The session can't be archived, because `archive_done` takes only done sessions.

Leif chose "A1 + B" and captured DRIVE-16 ("Capture both"):
- A1: a session whose native change is archived on HEAD is finished, is archived by `session new`/`use`, and does not count toward several open. This reads `finalization.json`, with no native read.
- B: `chough session close <key> --reason` closes abandoned work, with no native read.
