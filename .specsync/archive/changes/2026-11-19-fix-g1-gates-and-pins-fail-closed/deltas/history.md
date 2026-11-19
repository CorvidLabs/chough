## MODIFIED

### REQUIREMENT REQ-history-009

Inbox entries in the session's agent folder. `chough inbox add --session <id>` SHALL create one entry as a Markdown record (REQ-records-001) under `.chough/sessions/<id>/agent/inbox/`, giving `kind`, `value`, `from` and `recorded` when it names an observation, and SHALL NOT edit any session file. It SHALL hold the project's writer lock from before it checks that the session is open until it exits, so a concurrent `session use` or `session new` either sees the entry and keeps a done session open, or archives it first and the add refuses with nothing written; an entry SHALL never be left beside an archived session. It SHALL publish the entry whole, by linking a complete temporary file into place, and the next save SHALL remove a temporary file an interrupted add left. When an `observation` entry names no evidence entry, `inbox add` SHALL still add it, warn on standard error naming the ID, and `inbox list` SHALL mark it `recorded: false`. `inbox fold` SHALL add the active session's unfolded entries to its sections without duplicates and move each into `agent/inbox/folded/`. Inbox entries SHALL carry no approval, review, consent, gate or stage field. Trace: DRIVE-3, DRIVE-24, LIFE-8.

Acceptance Criteria
- Folding an inbox twice adds each entry once, and adding an entry never changes a session file.
- While another process holds the lock and archives the session, `inbox add` waits, then refuses with "no open session" and writes no entry.
- An `inbox add` killed while writing leaves no partial entry, and the next save removes its temporary file.
- `inbox add --kind observation` with an unrecorded ID adds the entry, warns naming the ID, and `inbox list` marks it `recorded: false`.
- With the pointer on one session and `--session` or `CHOUGH_SESSION` naming another, `chough inbox list` lists the named session's entries and names that session.
