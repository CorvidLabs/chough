---
change: keep-one-session-per-objective-or-native-change-under-chough-sessions-with-append-only-transitions-an-active-pointer
artifact: requirements
---

# Requirements

ADDED REQ-cli-037 (`deltas/cli.md`): when `.chough/sessions/` exists the canonical
checkpoint is the active session file, resolved from the untracked pointer or the
only open session, and every command reads and writes that file; without it the
behaviour and bytes are unchanged.

ADDED REQ-history-006 (`deltas/history.md`): sessions per objective or native change,
each with its own history cycle; `session new`, `adopt`, `use` and `list`; automatic
archiving of done sessions on switch; and the create-only inbox with `inbox add`,
`list` and `fold`. Inbox entries carry no approval, review, consent, gate or stage
field.
