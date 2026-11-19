# history delta

## ADDED

### REQUIREMENT REQ-history-016

Uncommitted records read. Besides the committed records REQ-history-012 reads, `chough doctor` SHALL read every entry that Git does not track and does not ignore in the folders under `.chough/` that Chough reads records from, such as a session's folder, its `evidence/` and inbox, `bugs/` and the archived stories, for example a new session, an evidence entry a killed run left empty or a bug cut off mid-write, and SHALL report one its reader refuses as `unreadable`, the finding REQ-history-012 defines for a committed record, naming the file, relative to the project, and the reader's reason, with the recovery `chough session list` gives for a session, `chough session shelve <id>`, so a record that stops `chough next` and `chough status` is never one doctor is silent about (#408). It SHALL read such an entry only when it is a regular file, opened so that nothing can make the read wait, and SHALL report a FIFO, a socket or a folder there as an unreadable record by its project-relative path, without opening it. Trace: DRIVE-17.

Acceptance Criteria
- An uncommitted session whose stage is unknown, an uncommitted 0-byte evidence entry and an uncommitted bug cut off inside its front matter each give an `unreadable` finding naming the file and the reader's reason, the session's naming `chough session shelve <id>`; no output holds an absolute path.
- A FIFO, a socket and a folder at an evidence entry's name, none tracked, each give an `unreadable` finding naming it relative to the project, and `chough doctor` finishes within 10 seconds without opening the FIFO.
- A file under `.chough/local/`, which Git ignores, gives no finding, and a committed unreadable record is still reported once.

## MODIFIED

(none)

## REMOVED

(none)
