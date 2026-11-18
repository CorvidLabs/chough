# cli delta

## ADDED

(none)

## MODIFIED

### REQUIREMENT REQ-cli-037

Active session checkpoint. The canonical checkpoint SHALL be the active session's `.chough/sessions/<id>/session.md`, named by the untracked pointer `.chough/local/active-session` or, with no pointer, the only open session or else the only open session that is not finished (REQ-history-006), and every command SHALL read and write that file. A command SHALL instead use the session that its `--session <id>` option names, or, without the option, the session the `CHOUGH_SESSION` environment variable names, each resolved by ID, alias or bare slug as `chough session use` resolves them, so each agent shell writes to its own session whatever the shared pointer says. A named session that is not open SHALL refuse the command, naming it and `chough session list`, with nothing written, and the option and the variable SHALL never change the pointer. `chough init --session <key>` keeps its meaning, the first session's key, and `chough inbox add --session` already names its session; `chough session new` and `chough session use`, which set the pointer, ignore the variable. A finished session, and a session folder whose story exists (REQ-history-006), SHALL NOT count toward several open sessions, and telling which sessions are finished SHALL need no native read (DRIVE-16). Chough SHALL refuse with nothing written when it cannot tell which session is active, and a command that needs a session when none is open SHALL refuse naming `chough session new <key>`. `chough init --objective <text>` SHALL create the first session as `chough session new` would, keyed by `--session <key>` or else by the objective's slug, and `chough init` without `--objective` SHALL create none. Only the active session is written, so this is not a second writable store (REQ-cli-007). Each command SHALL resolve the active session once and save only to that session, so a switch by another process partway through a command does not redirect its save; if that session is archived or removed meanwhile, the save SHALL refuse with nothing written. Trace: DRIVE-3, DRIVE-16, DRIVE-24.

Acceptance Criteria
- A stage move changes only the active session's `session.md`.
- No session, several open sessions without a pointer, and a pointer to a missing or archived session each refuse with nothing written, and the first names `chough session new <key>`.
- A `session use` by another process partway through a command leaves the newly active session's file unchanged, and the command's save lands in the session it resolved.
- With no pointer, an open session whose native change is archived on HEAD, and a session folder whose story exists, do not count toward several open sessions: the only other open session is active, with no native read.
- `chough init --objective <text>` creates one session named from the objective's slug, and `chough init` alone creates none.
- With two open sessions and the pointer on one, `chough evidence add --session <other>`, and the same command with `CHOUGH_SESSION=<other>`, record into the other session and leave the pointer unchanged, and `chough next --session <other>` shows that session.
- A `--session` or `CHOUGH_SESSION` naming an archived or unknown session refuses with every file unchanged.

## REMOVED

(none)
