---
kind: session
id: 2026-11-20-260-whole-or-nothing
aliases: [260-whole-or-nothing]
objective: "A new record is written whole or not at all, so a full disk or a file-size limit never leaves a 0-byte or cut-off record at its name, and an empty record is named as such (#260, #352)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
pr: "426"
criteria: []
requirements: []
cases: []
design: []
files: []
observations:
  - 2026-11-20-tc-records-a-cut-off-write-leaves-no-record
  - 2026-11-20-tc-records-an-empty-record-is-named-as-empty
  - 2026-11-20-tc-records-a-cut-off-write-leaves-no-record-bf0e58
  - 2026-11-20-tc-records-an-empty-record-is-named-as-empty-97908b
  - 2026-11-20-tc-records-a-cut-off-write-leaves-no-record-ffa649
  - 2026-11-20-tc-records-an-empty-record-is-named-as-empty-113250
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-260-a-new-record-is-written-whole-or-not-at-all
    holds: accept
    drives: review
    opened: 2026-11-20T18:57:21Z
run_consents: []
next: Continue accept
title: A new record is written whole or not at all
summary:
  - "chough bug add and journal add write a new record to .chough/local/ first, sync it, and give it its name without replacing anything, so a full disk or a file-size limit leaves no file at the name; where the file system cannot rename that way it is created at its name and removed on any error. No hard link is used, so exFAT works (#260, #424). An empty record is named as empty, with the way out."
  - "Verified: the tester's PASS at 829d479b; corvid-agent's approval and Linux run; its four notes taken at 73145abd (the temporary file goes through the symlink guard); main de49485f merged in at 50f0fd0c, own lines byte-identical; full lane green at 34fd2a92."
---
# A new record is written whole or not at all, so a full disk or a file-size limit never leaves a 0-byte or cut-off record at its name, and an empty record is named as such (#260, #352)

## Goal

A new record is written whole or not at all, so a full disk or a file-size limit never leaves a 0-byte or cut-off record at its name, and an empty record is named as such (#260, #352)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-260-a-new-record-is-written-whole-or-not-at-all (native approval)": "Standing OK, tell me after" (Leif's standing OK for scope approvals (2026-11-20 dialog, relayed via t4c): his answer 'Standing OK, tell me after' in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 04:17Z. Confirmed by t4c for this change on fable-lead's summary (one criterion added to each changed requirement, nothing removed, no refusal). Recorded by t4c itself, at the draft b7550830); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve fix-260-a-new-record-is-written-whole-or-not-at-all as 0xLeif
- 0xLeif decided "One send-back of fix-260-a-new-record-is-written-whole-or-not-at-all to add src/checkpoint.rs to its paths": "Standing OK, tell me after" (Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: src/checkpoint.rs joins #260's paths so its approved exFAT criterion holds, and #260 also closes #424; no delta word changes, no new refusal. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of fix-260-a-new-record-is-written-whole-or-not-at-all with src/checkpoint.rs added to its paths (native approval)": "Standing OK, tell me after" (Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: src/checkpoint.rs joins #260's paths so its approved exFAT criterion holds, and #260 also closes #424; no delta word changes, no new refusal. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve fix-260-a-new-record-is-written-whole-or-not-at-all as 0xLeif

## What was verified

- [Ran cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record for TC-records-a-cut-off-write-leaves-no-record: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-cut-off-write-leaves-no-record.md)
- [Ran cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty for TC-records-an-empty-record-is-named-as-empty: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-an-empty-record-is-named-as-empty.md)
- [Ran cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record for TC-records-a-cut-off-write-leaves-no-record: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-cut-off-write-leaves-no-record-bf0e58.md)
- [Ran cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty for TC-records-an-empty-record-is-named-as-empty: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-an-empty-record-is-named-as-empty-97908b.md)
- [Ran cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record for TC-records-a-cut-off-write-leaves-no-record: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-cut-off-write-leaves-no-record-ffa649.md)
- [Ran cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty for TC-records-an-empty-record-is-named-as-empty: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-an-empty-record-is-named-as-empty-113250.md)

## Open threads

## Timeline

- 2026-11-20T11:02:21Z · start · intent → intent · A new record is written whole or not at all, so a full disk or a file-size limit never leaves a 0-byte or cut-off record at its name, and an empty record is named as such (#260, #352) ^t-e8a776
  - revision: 056cbb8029f6535ff74f866a0523893916f682db
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T14:13:59Z · gate-opened · intent → intent · Scope approval of fix-260-a-new-record-is-written-whole-or-not-at-all (native approval) ^t-1124df
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: b75508306c8b9073e75eb0f79de5b10d3d09e84d
  - holds: intent
  - drives: approve
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T14:13:59Z · gate-closed · intent → intent · Scope approval of fix-260-a-new-record-is-written-whole-or-not-at-all (native approval) ^t-24f961
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: b75508306c8b9073e75eb0f79de5b10d3d09e84d
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Leif's standing OK for scope approvals (2026-11-20 dialog, relayed via t4c): his answer 'Standing OK, tell me after' in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 04:17Z. Confirmed by t4c for this change on fable-lead's summary (one criterion added to each changed requirement, nothing removed, no refusal). Recorded by t4c itself, at the draft b7550830
    - recorded_by: t4c
    - native: specsync change approve fix-260-a-new-record-is-written-whole-or-not-at-all
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T16:22:24Z · advance · intent → design ·  ^t-bd7097
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: ca3b03608717301c055316009a047504fb271bc4
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T16:22:24Z · advance · design → plan ·  ^t-0664ef
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: ca3b03608717301c055316009a047504fb271bc4
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T16:22:25Z · advance · plan → build · native:approved ^t-3b6ab7
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - gate: native:approved
  - revision: ca3b03608717301c055316009a047504fb271bc4
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T16:39:37Z · gate-opened · build → build · One send-back of fix-260-a-new-record-is-written-whole-or-not-at-all to add src/checkpoint.rs to its paths ^t-f11898
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: 5cfd95b5c9192701d5ef8bfe2e28bb6ee7e5e0e9
  - holds: build
  - drives: none
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T16:39:37Z · gate-closed · build → build · One send-back of fix-260-a-new-record-is-written-whole-or-not-at-all to add src/checkpoint.rs to its paths ^t-d3d358
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: 5cfd95b5c9192701d5ef8bfe2e28bb6ee7e5e0e9
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: src/checkpoint.rs joins #260's paths so its approved exFAT criterion holds, and #260 also closes #424; no delta word changes, no new refusal. Recorded by t4c
    - recorded_by: t4c
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T16:39:38Z · gate-opened · build → build · Re-approval of fix-260-a-new-record-is-written-whole-or-not-at-all with src/checkpoint.rs added to its paths (native approval) ^t-e20c42
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: 5cfd95b5c9192701d5ef8bfe2e28bb6ee7e5e0e9
  - holds: build
  - drives: approve
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T16:39:38Z · direct-native-write · build → build · fix-260-a-new-record-is-written-whole-or-not-at-all's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-d03c4c
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: 5cfd95b5c9192701d5ef8bfe2e28bb6ee7e5e0e9
  - direct_write:
    - change: fix-260-a-new-record-is-written-whole-or-not-at-all
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T16:39:38Z · gate-closed · build → build · Re-approval of fix-260-a-new-record-is-written-whole-or-not-at-all with src/checkpoint.rs added to its paths (native approval) ^t-dadd6a
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: 5cfd95b5c9192701d5ef8bfe2e28bb6ee7e5e0e9
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: src/checkpoint.rs joins #260's paths so its approved exFAT criterion holds, and #260 also closes #424; no delta word changes, no new refusal. Recorded by t4c
    - recorded_by: t4c
    - native: specsync change approve fix-260-a-new-record-is-written-whole-or-not-at-all
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T18:57:20Z · advance · build → test · native:verifying ^t-fa5ae8
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - gate: native:verifying
  - revision: 34fd2a92d1ad9a81db575af01bbd7430d9acd8e7
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T18:57:20Z · advance · test → accept · native:verifying ^t-9c6f15
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - gate: native:verifying
  - revision: 34fd2a92d1ad9a81db575af01bbd7430d9acd8e7
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T18:57:20Z · gate-opened · accept → accept · Implementation review of fix-260-a-new-record-is-written-whole-or-not-at-all ^t-f2965f
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: 34fd2a92d1ad9a81db575af01bbd7430d9acd8e7
  - holds: accept
  - drives: review
  - by: t4c (claude-opus-5-5, hc t4c)
