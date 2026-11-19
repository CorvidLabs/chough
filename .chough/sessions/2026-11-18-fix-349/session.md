---
kind: session
id: 2026-11-18-fix-349
aliases: [fix-349]
objective: "An archived story's What was verified line shows each entry's limitations beside its claim (REQ-history-006, #349 part 1)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-349
pr: ""
criteria: [KNOW-35]
requirements: [REQ-history-006]
cases: [TC-history-a-storys-verified-line-shows-the-entrys-limitations]
design: []
files: [src/sessions.rs, tests/evidence.rs]
observations:
  - 2026-11-18-tc-history-a-storys-verified-line-shows-the-entrys-limitations
  - 2026-11-18-tc-history-a-storys-verified-line-shows-the-entrys-limitations-903080
  - 2026-11-19-tc-history-a-storys-verified-line-shows-the-entrys-limitations
  - 2026-11-19-tc-history-a-storys-verified-line-shows-the-entrys-limitations-9142df
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-349
    holds: accept
    drives: review
    opened: 2026-11-18T22:52:24Z
run_consents: []
next: Wait for the tester's check and Leif's implementation review of fix-349
title: A story's verified line shows limitations, bounded
summary:
  - An archived story's What was verified line now shows an entry's limitations beside its claim, always on one line (#349). A person may give an entry at most 8 limitations of 512 bytes each; evidence add and cases record refuse more with nothing written, since a story writes each limitation twice. Tests failed first for the line, for multi-line text and at the bound; the verify lane passed on macOS after main was merged in, and tester2 passed the rechecks. Linux was not run locally. Limitations as a list in receipts, and the per-session overflow, are left for later.
---
# An archived story's What was verified line shows each entry's limitations beside its claim (REQ-history-006, #349 part 1)

## Goal

An archived story's What was verified line shows each entry's limitations beside its claim (REQ-history-006, #349 part 1)

## Where it stands

- The What was verified line shows limitations; the test fails on main and passes with the change

## Decisions and why

- 0xLeif decided "Scope approval of fix-349": "Part 1 now; 2 and 3 later" (Relayed by Leif's planning session (hc t4c) on 2026-11-18, in answer to fable-t4b's scope question on #349: one small PR now for the story's What was verified line showing limitations; his decision 'yes' for that line is in issue #349); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-349 as 0xLeif
- 0xLeif decided "Scope approval of fix-349, with the limitation bound (native approval)": "512 bytes × 8 per entry" (Relayed by Leif's planning session (hc t4c) on 2026-11-18 as his scope for #349 part 3, to be added in #358 before it merges: each limitation up to 512 bytes, up to 8 per evidence entry, more refused with nothing written. The REQ-history-006 delta is the same added clause rewritten onto main's newer text, re-approved under his standing rule for today ('Yes, for today')); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-349 as 0xLeif

## What was verified

- [Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-storys-verified-line-shows-the-entrys-limitations: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-history-a-storys-verified-line-shows-the-entrys-limitations.md)
- [Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-storys-verified-line-shows-the-entrys-limitations: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-history-a-storys-verified-line-shows-the-entrys-limitations-903080.md)
- [Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-storys-verified-line-shows-the-entrys-limitations: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-a-storys-verified-line-shows-the-entrys-limitations.md)
- [Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-storys-verified-line-shows-the-entrys-limitations: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-a-storys-verified-line-shows-the-entrys-limitations-9142df.md)

## Open threads

## Timeline

- 2026-11-18T22:43:37Z · start · intent → intent · An archived story's What was verified line shows each entry's limitations beside its claim (REQ-history-006, #349 part 1) ^t-0f11ee
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:43:37Z · advance · intent → design ·  ^t-94808b
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:43:37Z · skip · design → plan · Record content change; nothing visual or interactive changes ^t-6cc349
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:47:56Z · gate-opened · plan → plan · Scope approval of fix-349 ^t-5ddecd
  - native_change: fix-349
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:47:56Z · gate-closed · plan → plan · Scope approval of fix-349 ^t-c16611
  - native_change: fix-349
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - relayed_by:
    - decided_by: 0xLeif
    - words: Part 1 now; 2 and 3 later
    - where: Relayed by Leif's planning session (hc t4c) on 2026-11-18, in answer to fable-t4b's scope question on #349: one small PR now for the story's What was verified line showing limitations; his decision 'yes' for that line is in issue #349
    - recorded_by: fable-t4b
    - native: specsync change approve fix-349
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:47:57Z · advance · plan → build · native:approved ^t-1f7119
  - native_change: fix-349
  - gate: native:approved
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:47:57Z · advance · build → test · native:approved ^t-2a964a
  - native_change: fix-349
  - gate: native:approved
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:52:24Z · advance · test → accept · native:verifying ^t-9f3d0e
  - native_change: fix-349
  - gate: native:verifying
  - revision: ce50f931a83d31f512e0ed8f07b05a3430478265
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:52:24Z · gate-opened · accept → accept · Implementation review of fix-349 ^t-6a3636
  - native_change: fix-349
  - revision: ce50f931a83d31f512e0ed8f07b05a3430478265
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T00:19:01Z · gate-opened · accept → accept · Scope approval of fix-349, with the limitation bound (native approval) ^t-0f43c1
  - native_change: fix-349
  - revision: 50bf07f9759c8d45d2281c3ccdbf61e2b951a6fc
  - holds: accept
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T00:19:01Z · direct-native-write · accept → accept · fix-349's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-dfd74c
  - native_change: fix-349
  - revision: 50bf07f9759c8d45d2281c3ccdbf61e2b951a6fc
  - direct_write:
    - change: fix-349
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T00:19:01Z · gate-closed · accept → accept · Scope approval of fix-349, with the limitation bound (native approval) ^t-041ce7
  - native_change: fix-349
  - revision: 50bf07f9759c8d45d2281c3ccdbf61e2b951a6fc
  - relayed_by:
    - decided_by: 0xLeif
    - words: 512 bytes × 8 per entry
    - where: Relayed by Leif's planning session (hc t4c) on 2026-11-18 as his scope for #349 part 3, to be added in #358 before it merges: each limitation up to 512 bytes, up to 8 per evidence entry, more refused with nothing written. The REQ-history-006 delta is the same added clause rewritten onto main's newer text, re-approved under his standing rule for today ('Yes, for today')
    - recorded_by: fable-t4b
    - native: specsync change approve fix-349
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
