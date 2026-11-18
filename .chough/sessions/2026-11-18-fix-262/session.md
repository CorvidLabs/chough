---
kind: session
id: 2026-11-18-fix-262
aliases: [fix-262]
objective: "A FIFO or other non-regular file in a record's place blocks no reader, and a session whose record is one is listed with the reason and the shelve recovery (REQ-history-006, #262)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-262
pr: ""
criteria: [DRIVE-14]
requirements: [REQ-history-006]
cases:
  - TC-history-a-fifo-in-the-bugs-folder-blocks-no-reader
  - TC-history-a-session-whose-record-is-a-fifo-is-listed
design: []
files: [src/records.rs, src/evidence.rs, src/sessions.rs, tests/special_files.rs]
observations:
  - 2026-11-18-tc-history-a-fifo-in-the-bugs-folder-blocks-no-reader
  - 2026-11-18-tc-history-a-session-whose-record-is-a-fifo-is-listed
  - 2026-11-18-tc-history-a-fifo-in-the-bugs-folder-blocks-no-reader-dd7eb7
  - 2026-11-18-tc-history-a-session-whose-record-is-a-fifo-is-listed-1d0a81
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-262
    holds: accept
    drives: review
    opened: 2026-11-18T00:37:06Z
run_consents: []
next: "Wait for Leif's implementation review of fix-262 on PR #305; do not ship"
---
# A FIFO or other non-regular file in a record's place blocks no reader, and a session whose record is one is listed with the reason and the shelve recovery (REQ-history-006, #262)

## Goal

A FIFO or other non-regular file in a record's place blocks no reader, and a session whose record is one is listed with the reason and the shelve recovery (REQ-history-006, #262)

## Where it stands

- Failing tests first on main at 8abb5bb: status blocked past the 20 second deadline, and the session with a FIFO record was not listed
- The bounded reader refuses a non-regular file before opening it; the evidence-side readers skip non-regular entries; a session whose record is not a regular file is listed as not readable and can be shelved; both tests pass

## Decisions and why

- No requirement text changes (--no-spec-change). The session part rests on REQ-history-006. For the bugs folder no requirement names non-regular files; status and next now skip them as bug list already does. A doctor finding for such a file is new behaviour and is left out.
- Leif, 2026-11-17, in the task he gave Fable: 'implement only defects the existing contract already requires, open draft pull requests, and stop at every human gate.' The code and tests were written on that instruction before scope approval; Fable approves nothing.
- 0xLeif decided "Scope approval of fix-262": "record my scope approval of fix-272, fix-267, fix-262 and fix-260 as 0xLeif" (Claude Code session with Fable, 2026-11-17, after his 'i approve all' to Fable's report of the four scope gates); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-262 as 0xLeif

## What was verified

- [Ran cargo test --locked --test special_files -- --exact a_fifo_in_the_bugs_folder_blocks_no_reader for TC-history-a-fifo-in-the-bugs-folder-blocks-no-reader: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-history-a-fifo-in-the-bugs-folder-blocks-no-reader.md)
- [Ran cargo test --locked --test special_files -- --exact a_session_whose_record_is_a_fifo_is_listed_with_the_reason_and_recovery for TC-history-a-session-whose-record-is-a-fifo-is-listed: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-history-a-session-whose-record-is-a-fifo-is-listed.md)
- [Ran cargo test --locked --test special_files -- --exact a_fifo_in_the_bugs_folder_blocks_no_reader for TC-history-a-fifo-in-the-bugs-folder-blocks-no-reader: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-history-a-fifo-in-the-bugs-folder-blocks-no-reader-dd7eb7.md)
- [Ran cargo test --locked --test special_files -- --exact a_session_whose_record_is_a_fifo_is_listed_with_the_reason_and_recovery for TC-history-a-session-whose-record-is-a-fifo-is-listed: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-history-a-session-whose-record-is-a-fifo-is-listed-1d0a81.md)

## Open threads

- For a non-regular file in .chough/bugs/: is skipping it, as bug list does, right for status and next, or should they refuse and name the path?

## Timeline

- 2026-11-18T00:13:14Z · start · intent → intent · A FIFO or other non-regular file in a record's place blocks no reader, and a session whose record is one is listed with the reason and the shelve recovery (REQ-history-006, #262) ^t-fb007c
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:13:14Z · advance · intent → design ·  ^t-901666
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:13:14Z · skip · design → plan · Record reading change; nothing visual or interactive changes ^t-75a3ca
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:20:38Z · gate-opened · plan → plan · Scope approval of fix-262 ^t-f149cc
  - native_change: fix-262
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:36:10Z · gate-closed · plan → plan · Scope approval of fix-262 ^t-8a8dc9
  - native_change: fix-262
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - relayed_by:
    - decided_by: 0xLeif
    - words: record my scope approval of fix-272, fix-267, fix-262 and fix-260 as 0xLeif
    - where: Claude Code session with Fable, 2026-11-17, after his 'i approve all' to Fable's report of the four scope gates
    - recorded_by: fable-t4b
    - native: specsync change approve fix-262
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:37:02Z · advance · plan → build · native:approved ^t-1c05a3
  - native_change: fix-262
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:37:02Z · advance · build → test · native:approved ^t-0a586a
  - native_change: fix-262
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:37:02Z · advance · test → accept · native:approved ^t-86845d
  - native_change: fix-262
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:37:06Z · gate-opened · accept → accept · Implementation review of fix-262 ^t-d1c631
  - native_change: fix-262
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
