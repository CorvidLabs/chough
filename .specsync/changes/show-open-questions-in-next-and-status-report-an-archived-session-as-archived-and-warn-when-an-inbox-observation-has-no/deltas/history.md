## MODIFIED

### REQUIREMENT REQ-history-006

Sessions per objective. `chough session new <key>` SHALL create `.chough/sessions/<key>.json`, a schema 2 checkpoint with its own history cycle, without replacing a file, and point at it; `session adopt <key>` SHALL move an existing checkpoint with a history cursor there byte for byte; `session use <key>` SHALL point at an open session; `session list` SHALL report each session's key, stage, status (`archived` for an archived session, otherwise its stage status) and whether it is active or archived. `session new` and `session use` SHALL move every other session at stage done to `.chough/sessions/archive/<key>.json` unchanged, except that a done session with unfolded inbox entries SHALL stay open and be reported as kept open. `chough inbox add --session <key>` SHALL create one entry file under `.chough/sessions/<key>.inbox/` and SHALL NOT edit any session; when an `observation` entry names an observation with no record in `.chough/evidence/`, `inbox add` SHALL still add it and SHALL warn on stderr, naming the ID, and `inbox list` SHALL mark that entry `recorded: false`; SHALL add the active session's unfolded entries to its lists without duplicates and move each into `folded/`. Inbox entries SHALL carry no approval, review, consent, gate or stage field. Two branches that record work in different sessions SHALL merge without conflict. Evidence retention SHALL treat every open and archived session as a checkpoint reference source, and SHALL refuse a preview or apply when any session cannot be read. Trace: DRIVE-3, LIFE-8, LIFE-3, LIFE-7.

Acceptance Criteria
- Two branches that each advance a different session merge in Git with no conflict and keep both sessions' history.
- Switching away from a done session moves it to the archive with its bytes unchanged.
- Folding an inbox twice adds each entry once, and adding an entry never changes a session file.
- A done session with unfolded inbox entries stays open on a switch.
- An observation referenced only by an inactive or archived session is protected from `evidence prune --apply`, and an unreadable session file refuses it with nothing removed.
- `session list` reports an archived session with status `archived`.
- `inbox add --kind observation` with an ID that has no evidence record adds the entry, warns on stderr naming the ID, and `inbox list` marks it `recorded: false`; a recorded observation gets no warning and `recorded: true`.
