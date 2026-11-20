# history delta

## ADDED

(none)

## MODIFIED

### REQUIREMENT REQ-history-015

Append-only corrections. `chough correct <entry> --reason <text>` SHALL append a `correction` entry to the timeline that holds `<entry>`, giving `corrects: ^<entry id>` and the statement, and SHALL leave every existing byte of that file unchanged, so history is never rewritten. `<entry>` SHALL be `^<entry id>` for the active session, or `<session id>#^<entry id>` for any open session or archived story, the session ID or an alias (REQ-records-003). Correcting an entry of an archived story SHALL append to that story's Timeline and change nothing else in it (REQ-records-012). It SHALL refuse, writing nothing, an unknown session or entry, a blank statement or one over 4,096 bytes, and a correction of a correction, naming a given `<entry>` only as REQ-records-006 allows, so one holding a pattern kind or a control character is named by what it holds, never echoed (#367). `chough next` and `chough status` SHALL show the corrected text of a corrected decision or next action, marked as corrected with the count of corrections, for the active session and for archived stories they show; `chough next` SHALL list at most 10 corrections, newest first by recorded time and then entry ID, each cut to 512 bytes, with their total. `chough timing` SHALL treat a correction as a known kind that starts and ends no stage segment. Trace: LIFE-18, LIFE-7, FLOW-13.

Acceptance Criteria
- Correcting a stage move's reason appends one `correction` entry linked to it, and the session file is otherwise byte-identical; `next` and `status` show the corrected text marked as corrected.
- Correcting an entry of an archived story appends to its Timeline only, and every earlier byte of the story is unchanged.
- An unknown entry, a blank or 4,097-byte statement and a correction of a correction are refused with nothing written.
- With 12 corrections, `next` lists 10 newest first and says 12 in total; `timing --json` reports no unrecognised kind.
- An `<entry>` holding a token or an escape sequence is refused with nothing written, naming what it holds, and the output holds neither.

## REMOVED

(none)
