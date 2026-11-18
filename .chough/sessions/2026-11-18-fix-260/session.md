---
kind: session
id: 2026-11-18-fix-260
aliases: [fix-260]
objective: "A new record whose write fails is not left behind as an empty or cut-off file (REQ-records-001, #260)"
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: fix-260
pr: ""
criteria: [DRIVE-25]
requirements: [REQ-records-001]
cases: [TC-records-a-failed-new-record-write-leaves-no-file]
design: []
files: [src/records.rs, tests/bugs.rs]
observations:
  - 2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file
  - 2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file-7d826f
  - 2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file-23bdea
  - 2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file-48e544
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
title: A new record whose write fails is not left behind
summary:
  - "A bug add whose write failed part way left a partial record at the name (#260). A new record is now written to a private temporary file and linked into place, and a failure removes only the temporary this call created. Tests failed first, including both review findings on #306; the verify lane passed on macOS after main (c78f878) was merged in. Linux and a real full disk were not run locally."
---
# A new record whose write fails is not left behind as an empty or cut-off file (REQ-records-001, #260)

## Goal

A new record whose write fails is not left behind as an empty or cut-off file (REQ-records-001, #260)

## Where it stands

- Failing test first on main at 8abb5bb: bug add under a 4,096-byte file size limit failed and left a cut-off record
- The create-new writer removes the file it created when its write fails; the test passes

## Decisions and why

- No requirement text changes (--no-spec-change): REQ-records-001 already says a record Chough writes is one Markdown file that starts with front matter. The one create-new writer removes the file it created when its write fails; bugs and journal posts both go through it. A doctor finding for a leftover partial record, and a temp-and-link write, are left out.
- Leif, 2026-11-17, in the task he gave Fable: 'implement only defects the existing contract already requires, open draft pull requests, and stop at every human gate.' The code and tests were written on that instruction before scope approval; Fable approves nothing.
- 0xLeif decided "Scope approval of fix-260": "record my scope approval of fix-272, fix-267, fix-262 and fix-260 as 0xLeif" (Claude Code session with Fable, 2026-11-17, after his 'i approve all' to Fable's report of the four scope gates); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-260 as 0xLeif
- 0xLeif decided "Implementation review of fix-260": "Pass (Recommended)" (Claude Code session with Fable, 2026-11-17, answer to the interview question 'Do you pass the implementation review of fix-260?' after the agent review passed 68110b5); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change review fix-260 as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test bugs -- --exact a_bug_add_whose_write_fails_part_way_leaves_no_partial_record for TC-records-a-failed-new-record-write-leaves-no-file: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_add_whose_write_fails_part_way_leaves_no_partial_record for TC-records-a-failed-new-record-write-leaves-no-file: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file-7d826f.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_add_whose_write_fails_part_way_leaves_no_partial_record for TC-records-a-failed-new-record-write-leaves-no-file: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file-23bdea.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_add_whose_write_fails_part_way_leaves_no_partial_record for TC-records-a-failed-new-record-write-leaves-no-file: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-failed-new-record-write-leaves-no-file-48e544.md)

## Open threads

## Timeline

- 2026-11-18T00:15:02Z · start · intent → intent · A new record whose write fails is not left behind as an empty or cut-off file (REQ-records-001, #260) ^t-9a8506
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:15:02Z · advance · intent → design ·  ^t-f730d5
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:15:02Z · skip · design → plan · Record writing change; nothing visual or interactive changes ^t-e1aab6
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:20:38Z · gate-opened · plan → plan · Scope approval of fix-260 ^t-33afcf
  - native_change: fix-260
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:36:11Z · gate-closed · plan → plan · Scope approval of fix-260 ^t-c89717
  - native_change: fix-260
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - relayed_by:
    - decided_by: 0xLeif
    - words: record my scope approval of fix-272, fix-267, fix-262 and fix-260 as 0xLeif
    - where: Claude Code session with Fable, 2026-11-17, after his 'i approve all' to Fable's report of the four scope gates
    - recorded_by: fable-t4b
    - native: specsync change approve fix-260
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:37:09Z · advance · plan → build · native:approved ^t-411897
  - native_change: fix-260
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:37:09Z · advance · build → test · native:approved ^t-2bb14e
  - native_change: fix-260
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:37:09Z · advance · test → accept · native:approved ^t-98812d
  - native_change: fix-260
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:37:12Z · gate-opened · accept → accept · Implementation review of fix-260 ^t-3b3f5c
  - native_change: fix-260
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:10:30Z · gate-closed · accept → accept · Implementation review of fix-260 ^t-fadbef
  - native_change: fix-260
  - revision: 16678f51ba64843c6538d70dd08a2d4d915b51e2
  - relayed_by:
    - decided_by: 0xLeif
    - words: Pass (Recommended)
    - where: Claude Code session with Fable, 2026-11-17, answer to the interview question 'Do you pass the implementation review of fix-260?' after the agent review passed 68110b5
    - recorded_by: fable-t4b
    - native: specsync change review fix-260
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:10:31Z · advance · accept → done · native:archived ^t-b350df
  - native_change: fix-260
  - gate: native:archived
  - revision: 16678f51ba64843c6538d70dd08a2d4d915b51e2
  - by: fable (claude-fable-5-2, hc t4b)
