## ADDED

### REQUIREMENT REQ-history-006

Sessions per objective. `chough session new <key>` SHALL create `.chough/sessions/<key>.json`, a schema 2 checkpoint with its own history cycle, without replacing a file, and point at it; `session adopt <key>` SHALL move an existing checkpoint with a history cursor there byte for byte; `session use <key>` SHALL point at an open session; `session list` SHALL report each session's key, stage, status and whether it is active or archived. `session new` and `session use` SHALL move every other session at stage done to `.chough/sessions/archive/<key>.json` unchanged. `chough inbox add --session <key>` SHALL create one entry file under `.chough/sessions/<key>.inbox/` and SHALL NOT edit any session; `inbox fold` SHALL add the active session's unfolded entries to its lists without duplicates and move each into `folded/`. Inbox entries SHALL carry no approval, review, consent, gate or stage field. Two branches that record work in different sessions SHALL merge without conflict. Trace: DRIVE-3.

Acceptance Criteria
- Two branches that each advance a different session merge in Git with no conflict and keep both sessions' history.
- Switching away from a done session moves it to the archive with its bytes unchanged.
- Folding an inbox twice adds each entry once, and adding an entry never changes a session file.
