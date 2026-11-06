## MODIFIED

### REQUIREMENT REQ-cli-037

Active session checkpoint. When `.chough/sessions/` exists, the canonical checkpoint SHALL be the active session file `.chough/sessions/<key>.json`, named by the untracked pointer `.chough/local/active-session` or, with no pointer, the only open session or else the only open session that is not finished (REQ-history-006), and every command SHALL read and write that file instead of `.chough/session.json`. A finished session SHALL NOT count toward several open sessions, and telling which sessions are finished SHALL need no native read (DRIVE-16). Chough SHALL refuse with nothing written when it cannot tell which session is active, and `init` SHALL refuse in sessions mode. Only the active session is written, so this is not a second writable store (REQ-cli-007). Each command SHALL resolve the active session once and save only to that session, so a switch by another process partway through a command does not redirect its save; if that session is archived or removed meanwhile, the save SHALL refuse with nothing written. Without `.chough/sessions/` behaviour and bytes SHALL be unchanged. Trace: DRIVE-3, DRIVE-16.

Acceptance Criteria
- A stage move in sessions mode changes only the active session file and its history cycle.
- No session, several open sessions without a pointer, or a pointer to a missing or archived session refuses with nothing written.
- A `session use` by another process partway through a command leaves the newly active session's file unchanged, and the command's save lands in the session it resolved.
- With no pointer, an open session whose native change is archived on HEAD does not count toward several open sessions: the only other open session is active, with no native read.
