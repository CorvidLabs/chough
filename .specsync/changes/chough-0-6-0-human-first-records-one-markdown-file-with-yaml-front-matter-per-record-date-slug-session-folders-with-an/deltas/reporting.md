# Chough 0.6.0: timing reads session timelines and archived stories

## MODIFIED

### REQUIREMENT REQ-reporting-001

Timing report. The helper SHALL provide a read-only timing report derived from existing records: stage segments from transitions, including iterations, back, reset, skip, pause and resume, and side-stage enter and return; case observations and repeated-run summaries; bug creation, reproduction and verification; and the native lifecycle times native SpecSync exposes. It SHALL append a timestamped transition when an awaiting-human gate opens and one when it is cleared, and SHALL report active time separately from paused time and from time waiting on a human, without recording who decided or what. Every duration SHALL be labelled elapsed wall-clock time, not effort; history summaries SHALL give ranges with their sample count, labelled estimates rather than targets; a missing, unparseable or out-of-order timestamp SHALL be reported unknown, never zero. Transitions SHALL be read from each session's timeline, in open sessions and archived stories alike (REQ-history-002), and case observations from evidence entries and the receipts of archived stories and bugs (REQ-cli-017), within the bounds of REQ-history-003; a session or story that cannot be read SHALL be named and the report marked partial. Inputs SHALL be bounded, and the report SHALL write no record. Trace: LIFE-10, DRIVE-24, LIFE-3, LIFE-6, LIFE-7.

Acceptance Criteria
- Fixture transitions with fixed timestamps for every transition kind yield the expected segments, gate records and `direct-native-write` events (REQ-history-013) start or end no segment, a `direct-native-write` event is not counted as an unrecognised kind, and paused and waiting-on-human time are excluded from active time.
- Opening a gate appends one gate-opened transition and clearing it one gate-closed transition; the report shows its waiting time, and a gate-opened record whose gate is no longer awaiting a human, with no gate-closed record, is reported closed at an unknown time.
- A missing timestamp or an end before its start is reported unknown, not zero, and fewer than three samples are reported too few to estimate.
- Running the report leaves every Chough and SpecSync record byte-identical, and input beyond its bound is reported partial.
- With one open session and three archived stories, the report gives each session's stage segments and gate waits from its timeline and each case's runs from entries and receipts; an unreadable story is named and the report is marked partial.
- No file under `.chough/history/` or `.chough/transitions.jsonl` is read.
