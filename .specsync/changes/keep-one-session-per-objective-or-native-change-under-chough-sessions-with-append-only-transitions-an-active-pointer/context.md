---
change: keep-one-session-per-objective-or-native-change-under-chough-sessions-with-append-only-transitions-an-active-pointer
artifact: context
---

# Context

On Raven on 2026-11-03 several objectives shared the one `.chough/session.json`, and
subagents had to edit that same file to report back, so branches with different work
conflicted on it. Leif's brief t811u item 3 (DRIVE-3) asks for: "sessions keyed by
objective or native change id (for example `.chough/sessions/<change-id>.json` with
append-only per-session transitions), a pointer to the active one, automatic
archiving of a finished session, and a way for subagents to append observations to
the parent session without editing the same file (an append-only inbox). Merging two
branches with different sessions must not conflict. Keep `checkpoint --merge`
working, or replace it." The parent relayed at 6:37 PM MST to follow that stored
preference with the hi-first lifecycle.

The hard rule holds: Chough records no human approval or review, and native SpecSync
state decides stage gates. Specs have no size cap (Leif, relayed 6:43 PM MST); the cli
change here is one short requirement and the sessions behaviour goes in the history
spec, so any later cli split is guidance only.
