# Append-only corrections for recorded events (issue 167)

## Added

### REQUIREMENT REQ-history-015

Append-only corrections. `chough correct <event> --reason <text>` SHALL append a correction event that names the `EVT-` event it corrects and carries the corrected statement, in the active session's current cycle, and SHALL refuse with nothing written an unknown event id, a blank statement, a statement over 4096 bytes, and a target that is itself a correction. The corrected event and every other record SHALL keep their bytes; a correction never rewrites history. `chough next` and `chough status` SHALL show each correction recorded in the active session's current cycle with its target, beside the state the target produced. Chough builds before this requirement refuse a history holding the event ("unknown transition kind"), so the change log SHALL name the first release that contains it as the oldest Chough that reads it. Trace: LIFE-2, FLOW-13.

Acceptance Criteria
- Correcting a recorded transition appends one event with kind `correction`, a `correction` payload of target and statement, and the target's bytes are unchanged; next and status show the statement and the target.
- An unknown event id, a blank or oversized statement, and a correction of a correction are refused with nothing written.
