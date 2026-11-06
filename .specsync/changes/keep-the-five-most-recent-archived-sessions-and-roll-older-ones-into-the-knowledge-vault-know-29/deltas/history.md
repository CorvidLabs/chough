## ADDED

### REQUIREMENT REQ-history-010

Archived session roll-up. When `chough stage advance` moves into done and its save succeeds, with the librarian on, after the trim of REQ-cli-041 and before the knowledge refresh of REQ-cli-043, the helper SHALL keep in `.chough/sessions/archive/` only the five most recent archived sessions. Recency SHALL be the finalization timestamp of the session's native change, newest first, then the key, the later key first. A session without a native change, or whose change has no readable finalization, SHALL count as the oldest. Each other archived session SHALL move byte for byte to `<library-dir>/attachments/<folder of its key>/sessions/<key>.json`, with the folder formed as in REQ-cli-046, and SHALL never replace a file. An attachment that already holds the same bytes SHALL be kept, and the source removed. One that holds different bytes SHALL leave the session in place and be reported. It SHALL report `applied.session_rollup` with status `rolled` and the moved keys, `unchanged`, `skipped` (librarian off), or `refused` with its reason, and SHALL NOT undo the move into done. A rolled session is no longer a retention reference source: its cycle is closed, and its native archive keeps what the change touched. `session list` SHALL NOT list it. Nothing SHALL be deleted from Git history. Trace: KNOW-29, DRIVE-13, KNOW-13.

Acceptance Criteria
- With seven archived sessions, five tied to archives finalized 60 to 20 days ago, one tied with the oldest, and one without a native change, the move into done keeps the five newest in `.chough/sessions/archive/` and moves the other two byte for byte into `knowledge/attachments/<key>/sessions/<key>.json`, reported as `rolled`.
- With the librarian off, no session moves and the step reports `skipped`.
