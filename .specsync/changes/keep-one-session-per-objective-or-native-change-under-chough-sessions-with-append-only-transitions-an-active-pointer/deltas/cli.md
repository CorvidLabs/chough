## ADDED

### REQUIREMENT REQ-cli-037

Active session checkpoint. When `.chough/sessions/` exists, the canonical checkpoint SHALL be the active session file `.chough/sessions/<key>.json`, named by the untracked pointer `.chough/local/active-session` or, with no pointer, the only open session, and every command SHALL read and write that file instead of `.chough/session.json`. Chough SHALL refuse with nothing written when it cannot tell which session is active, and `init` SHALL refuse in sessions mode. Only the active session is written, so this is not a second writable store (REQ-cli-007). Without `.chough/sessions/` behaviour and bytes SHALL be unchanged. Trace: DRIVE-3.

Acceptance Criteria
- A stage move in sessions mode changes only the active session file and its history cycle.
- No session, several open sessions without a pointer, or a pointer to a missing or archived session refuses with nothing written.
