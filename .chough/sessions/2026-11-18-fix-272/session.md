---
kind: session
id: 2026-11-18-fix-272
aliases: [fix-272]
objective: journal add holds the writer lock of REQ-cli-040, so parallel posts all reach the feed (#272)
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: fix-272
pr: ""
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
cases: [TC-cli-journal-add-holds-the-writer-lock]
design: []
files: [src/journal.rs, tests/journal.rs]
observations: [2026-11-18-tc-cli-journal-add-holds-the-writer-lock]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
title: journal add holds the writer lock
summary:
  - chough journal add took no writer lock, so several adds at once left the feed missing posts (#272). It now holds the lock REQ-cli-040 already named for it. Two tests failed on main first; the verify lane passed on macOS at 3f3f95c. Linux and CI were not run locally.
---
# journal add holds the writer lock of REQ-cli-040, so parallel posts all reach the feed (#272)

## Goal

journal add holds the writer lock of REQ-cli-040, so parallel posts all reach the feed (#272)

## Where it stands

- Failing tests first: journal_add_waits_for_the_writer_lock and parallel_journal_adds_all_reach_the_feed failed on main at 8abb5bb (the storm's feed listed 2 of 12 posts)
- journal add takes the writer lock before it reads or writes; both tests pass

## Decisions and why

- No requirement text changes: REQ-cli-040 already lists journal add as a writer that holds the lock, so this is --no-spec-change. library index is not in that list and is left alone.
- Leif, 2026-11-17, in the task he gave Fable: 'implement only defects the existing contract already requires, open draft pull requests, and stop at every human gate.' The code and tests were written on that instruction before scope approval; Fable approves nothing.
- 0xLeif decided "Scope approval of fix-272": "record my scope approval of fix-272, fix-267, fix-262 and fix-260 as 0xLeif" (Claude Code session with Fable, 2026-11-17, after his 'i approve all' to Fable's report of the four scope gates); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-272 as 0xLeif
- 0xLeif decided "Implementation review of fix-272": "pass all four, merge one at a time" (Claude Code session with Fable, 2026-11-17, in reply to Fable's report of the four implementation review gates); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change review fix-272 as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test journal -- --exact journal_add_waits_for_the_writer_lock for TC-cli-journal-add-holds-the-writer-lock: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-journal-add-holds-the-writer-lock.md)

## Open threads

## Timeline

- 2026-11-18T00:00:49Z · start · intent → intent · journal add holds the writer lock of REQ-cli-040, so parallel posts all reach the feed (#272) ^t-38377f
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:00:59Z · advance · intent → design ·  ^t-f22888
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:00:59Z · skip · design → plan · CLI locking change; nothing visual or interactive changes ^t-fc732e
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:04:32Z · gate-opened · plan → plan · Scope approval of fix-272 ^t-2b5c8e
  - native_change: fix-272
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:35:18Z · gate-closed · plan → plan · Scope approval of fix-272 ^t-4642fb
  - native_change: fix-272
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - relayed_by:
    - decided_by: 0xLeif
    - words: record my scope approval of fix-272, fix-267, fix-262 and fix-260 as 0xLeif
    - where: Claude Code session with Fable, 2026-11-17, after his 'i approve all' to Fable's report of the four scope gates
    - recorded_by: fable-t4b
    - native: specsync change approve fix-272
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:36:27Z · advance · plan → build · native:approved ^t-70cfd6
  - native_change: fix-272
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:36:27Z · advance · build → test · native:approved ^t-5bc777
  - native_change: fix-272
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:36:27Z · advance · test → accept · native:approved ^t-b2910a
  - native_change: fix-272
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:36:42Z · gate-opened · accept → accept · Implementation review of fix-272 ^t-0de349
  - native_change: fix-272
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:58:29Z · gate-closed · accept → accept · Implementation review of fix-272 ^t-8f472c
  - native_change: fix-272
  - revision: fd978ee5d0eaabe65f61c1bc574e3f3e537304c8
  - relayed_by:
    - decided_by: 0xLeif
    - words: pass all four, merge one at a time
    - where: Claude Code session with Fable, 2026-11-17, in reply to Fable's report of the four implementation review gates
    - recorded_by: fable-t4b
    - native: specsync change review fix-272
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:58:31Z · advance · accept → done · native:archived ^t-48814e
  - native_change: fix-272
  - gate: native:archived
  - revision: fd978ee5d0eaabe65f61c1bc574e3f3e537304c8
  - by: fable (claude-fable-5-2, hc t4b)
