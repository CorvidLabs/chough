---
kind: session
id: 2026-11-23-fix-437
aliases: [fix-437]
objective: "Fix chough#437: a direct-write entry writes before_state and after_state as none when the state is absent, in the Timeline and its text"
intent_scope: ""
modules: []
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: fix-437
pr: ""
criteria: [FLOW-2]
requirements: []
cases: []
design: []
files: []
observations: [2026-11-23-tc-records-inbox-add-without-hard-links-8d250e]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue plan
---
# Fix chough#437: a direct-write entry writes before_state and after_state as none when the state is absent, in the Timeline and its text

## Goal

Fix chough#437: a direct-write entry writes before_state and after_state as none when the state is absent, in the Timeline and its text

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of fix-437": "I approve" (pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change approve fix-437 as 0xLeif

## What was verified

- [Ran cargo test --locked --test no_hard_links -- --ignored --exact new_records_are_written_on_a_volume_without_hard_links for TC-records-inbox-add-without-hard-links: passed (exit 0).](evidence/2026-11-23-tc-records-inbox-add-without-hard-links-8d250e.md)

## Open threads

## Timeline

- 2026-11-23T14:44:33Z · start · intent → intent · fix-437 ^t-87f09a
  - revision: 35828a1df3d308677a38b138e7aebe0e7b6770ef
  - by: unknown agent (hc t14z)
- 2026-11-23T14:55:50Z · gate-opened · intent → intent · Approve the scope of fix-437 ^t-b5dbe6
  - native_change: fix-437
  - revision: a067aa560974bb904dd07c18e56ded5c038fd77d
  - holds: intent
  - drives: approve
  - by: unknown agent (hc t14z)
- 2026-11-23T14:56:25Z · direct-native-write · intent → intent · fix-437's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-cdc034
  - native_change: fix-437
  - revision: f64a87d77ce3e6075051119797060f9885db93b6
  - direct_write:
    - change: fix-437
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: unknown agent (hc t14z)
- 2026-11-23T14:56:25Z · gate-closed · intent → intent · Approve the scope of fix-437 ^t-f7a84f
  - native_change: fix-437
  - revision: f64a87d77ce3e6075051119797060f9885db93b6
  - relayed_by:
    - decided_by: 0xLeif
    - words: I approve
    - where: pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch
    - recorded_by: pi-t14z
    - summary_digest: c2c7464289420feb18e087b292f366f2a861900ac9fe922206bd001529848a12
    - list: 30451888bc48
    - native: specsync change approve fix-437
    - actor: 0xLeif
  - by: unknown agent (hc t14z)
- 2026-11-23T14:56:27Z · advance · intent → design ·  ^t-9c87e6
  - native_change: fix-437
  - revision: f64a87d77ce3e6075051119797060f9885db93b6
  - by: unknown agent (hc t14z)
- 2026-11-23T14:56:27Z · advance · design → plan ·  ^t-ae5e7b
  - native_change: fix-437
  - revision: f64a87d77ce3e6075051119797060f9885db93b6
  - by: unknown agent (hc t14z)
