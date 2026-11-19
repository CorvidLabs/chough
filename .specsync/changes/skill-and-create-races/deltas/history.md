# history delta

## ADDED

### REQUIREMENT REQ-history-016

Uncommitted records read. Besides the committed records REQ-history-012 reads, `chough doctor` SHALL read every record under `.chough/` that Git does not track yet and does not ignore, such as a new session, an evidence entry a killed run left empty or a bug cut off mid-write, and SHALL report one its reader refuses as `unreadable`, naming the file, relative to the project, and the reader's reason, with the recovery `chough session list` gives for a session, `chough session shelve <id>`, so a record that stops `chough next` and `chough status` is never one doctor is silent about (#408). Trace: DRIVE-17.

Acceptance Criteria
- An uncommitted session whose stage is unknown, an uncommitted 0-byte evidence entry and an uncommitted bug cut off inside its front matter each give an `unreadable` finding naming the file and the reader's reason, the session's naming `chough session shelve <id>`; no output holds an absolute path.
- A file under `.chough/local/`, which Git ignores, gives no finding, and a committed unreadable record is still reported once.

## MODIFIED

(none)

## REMOVED

(none)
