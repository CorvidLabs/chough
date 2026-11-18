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
observations:
  - 2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason
  - 2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason-c672c9
  - 2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason-28a08b
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
- 0xLeif decided "Scope approval of fix-250, rewritten onto main (native approval)": "Yes: send back once + re-approve" (Relayed by Leif's planning session (hc t4c) on 2026-11-18, which asked him whether fable-t4b may run the send-back on fix-250 this once and whether he re-approves the same scope: the reason on each withdrawn gate's gate-closed entry, echoed in doctor's JSON; only REQ-history-012, REQ-cli-030 and REQ-records-004 change; the REQ-history-002 list edit removed); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-250 as 0xLeif

## What was verified

- [Ran cargo test --locked --test doctor -- --exact doctor_decision_fixes_need_a_reason_and_record_no_human_decision for TC-history-a-withdrawn-stale-gate-keeps-doctors-reason: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason.md)
- [Ran cargo test --locked --test doctor -- --exact doctor_decision_fixes_need_a_reason_and_record_no_human_decision for TC-history-a-withdrawn-stale-gate-keeps-doctors-reason: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason-c672c9.md)
- [Ran cargo test --locked --test doctor -- --exact doctor_decision_fixes_need_a_reason_and_record_no_human_decision for TC-history-a-withdrawn-stale-gate-keeps-doctors-reason: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-history-a-withdrawn-stale-gate-keeps-doctors-reason-28a08b.md)

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
- 2026-11-18T20:40:59Z · gate-opened · accept → accept · Scope approval of fix-250, rewritten onto main (native approval) ^t-69fd07
  - native_change: fix-250
  - revision: 1e941d6827b1352ddbfa2bfcbad6956864d08a27
  - holds: accept
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T20:40:59Z · direct-native-write · accept → accept · fix-250's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-377f13
  - native_change: fix-250
  - revision: 1e941d6827b1352ddbfa2bfcbad6956864d08a27
  - direct_write:
    - change: fix-250
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T20:40:59Z · gate-closed · accept → accept · Scope approval of fix-250, rewritten onto main (native approval) ^t-fd8f75
  - native_change: fix-250
  - revision: 1e941d6827b1352ddbfa2bfcbad6956864d08a27
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes: send back once + re-approve
    - where: Relayed by Leif's planning session (hc t4c) on 2026-11-18, which asked him whether fable-t4b may run the send-back on fix-250 this once and whether he re-approves the same scope: the reason on each withdrawn gate's gate-closed entry, echoed in doctor's JSON; only REQ-history-012, REQ-cli-030 and REQ-records-004 change; the REQ-history-002 list edit removed
    - recorded_by: fable-t4b
    - native: specsync change approve fix-250
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
