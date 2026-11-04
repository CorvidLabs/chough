## ADDED

### REQUIREMENT REQ-history-009

Inbox entries and their sessions. `chough inbox add` SHALL hold the project's checkpoint writer lock from before it checks that its session is open until it exits, so a concurrent `session use` or `session new` either sees the entry and keeps a done session open, or archives it first and the add refuses with nothing written; an entry SHALL never be left beside an archived session. Evidence retention SHALL treat the observation IDs named by unfolded inbox entries of open and archived sessions as references, labelled `inbox of session <key>`, and SHALL refuse when such an entry cannot be read. Trace: DRIVE-3, LIFE-8.

Acceptance Criteria
- While another process holds the lock and archives the session, `inbox add` waits, then refuses with "no open session" and writes no entry.
- An old observation named only by an unfolded inbox entry is protected as `inbox of session <key>` and survives `evidence prune --apply`.
