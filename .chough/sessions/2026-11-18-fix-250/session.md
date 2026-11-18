---
kind: session
id: 2026-11-18-fix-250
aliases: [fix-250]
objective: "When doctor --fix --reason withdraws stale gates, each withdrawn gate's Timeline entry and the archived story carry the reason, and doctor's JSON echoes it (REQ-history-012, #250)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-250
pr: ""
criteria: [DRIVE-16]
requirements: [REQ-history-012]
cases: [TC-history-a-withdrawn-stale-gate-keeps-doctors-reason]
design: []
files: [src/doctor.rs, src/checkpoint.rs, src/stage.rs, src/session_md.rs, tests/doctor.rs]
observations: [2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason]
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-250
    holds: accept
    drives: review
    opened: 2026-11-18T20:15:19Z
run_consents: []
next: Wait for the tester's check and Leif's implementation review of fix-250
---
# When doctor --fix --reason withdraws stale gates, each withdrawn gate's Timeline entry and the archived story carry the reason, and doctor's JSON echoes it (REQ-history-012, #250)

## Goal

When doctor --fix --reason withdraws stale gates, each withdrawn gate's Timeline entry and the archived story carry the reason, and doctor's JSON echoes it (REQ-history-012, #250)

## Where it stands

- A withdrawn stale gate's entry keeps doctor's reason and the JSON echoes it; the test fails on main and passes with the fix

## Decisions and why

- 0xLeif decided "Scope approval of fix-250": "record it on each gate. When doctor --fix --reason withdraws stale gates, each withdrawn gate's Timeline entry and the archived story carry the reason in the person's words, and doctor's JSON echoes it back. REQ-history-012 gets that sentence." (Leif's decision comment on GitHub issue #250, 2026-11-18, from his planning interview; his planning session (hc t4c) relayed it to fable-t4b as his scope approval); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-250 as 0xLeif

## What was verified

- [Ran cargo test --locked --test doctor -- --exact doctor_decision_fixes_need_a_reason_and_record_no_human_decision for TC-history-a-withdrawn-stale-gate-keeps-doctors-reason: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason.md)

## Open threads

## Timeline

- 2026-11-18T20:07:02Z · start · intent → intent · When doctor --fix --reason withdraws stale gates, each withdrawn gate's Timeline entry and the archived story carry the reason, and doctor's JSON echoes it (REQ-history-012, #250) ^t-b2c716
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T20:07:14Z · advance · intent → design ·  ^t-c9d542
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T20:07:14Z · skip · design → plan · Record content change; nothing visual or interactive changes ^t-cd78e4
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T20:09:24Z · gate-opened · plan → plan · Scope approval of fix-250 ^t-5c7124
  - native_change: fix-250
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T20:09:24Z · gate-closed · plan → plan · Scope approval of fix-250 ^t-68c2e3
  - native_change: fix-250
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - relayed_by:
    - decided_by: 0xLeif
    - words: record it on each gate. When doctor --fix --reason withdraws stale gates, each withdrawn gate's Timeline entry and the archived story carry the reason in the person's words, and doctor's JSON echoes it back. REQ-history-012 gets that sentence.
    - where: Leif's decision comment on GitHub issue #250, 2026-11-18, from his planning interview; his planning session (hc t4c) relayed it to fable-t4b as his scope approval
    - recorded_by: fable-t4b
    - native: specsync change approve fix-250
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T20:09:25Z · advance · plan → build · native:approved ^t-9cba31
  - native_change: fix-250
  - gate: native:approved
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T20:09:25Z · advance · build → test · native:approved ^t-ab7839
  - native_change: fix-250
  - gate: native:approved
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T20:15:19Z · advance · test → accept · native:verifying ^t-6bdf3d
  - native_change: fix-250
  - gate: native:verifying
  - revision: a1d1118d283038341d73ac2cdcafdae736eaeaae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T20:15:19Z · gate-opened · accept → accept · Implementation review of fix-250 ^t-0f918d
  - native_change: fix-250
  - revision: a1d1118d283038341d73ac2cdcafdae736eaeaae
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
