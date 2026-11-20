---
kind: session
id: 2026-11-20-260-whole-or-nothing
aliases: [260-whole-or-nothing]
objective: "A new record is written whole or not at all, so a full disk or a file-size limit never leaves a 0-byte or cut-off record at its name, and an empty record is named as such (#260, #352)"
stage: done
stage_status: archived
iteration: 1
native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
pr: "426"
observations:
  - 2026-11-20-tc-records-a-cut-off-write-leaves-no-record
  - 2026-11-20-tc-records-an-empty-record-is-named-as-empty
  - 2026-11-20-tc-records-a-cut-off-write-leaves-no-record-bf0e58
  - 2026-11-20-tc-records-an-empty-record-is-named-as-empty-97908b
  - 2026-11-20-tc-records-a-cut-off-write-leaves-no-record-ffa649
  - 2026-11-20-tc-records-an-empty-record-is-named-as-empty-113250
title: A new record is written whole or not at all
summary:
  - "chough bug add and journal add write a new record to .chough/local/ first, sync it, and give it its name without replacing anything, so a full disk or a file-size limit leaves no file at the name; where the file system cannot rename that way it is created at its name and removed on any error. No hard link is used, so exFAT works (#260, #424). An empty record is named as empty, with the way out."
  - "Verified: the tester's PASS at 829d479b; corvid-agent's approval and Linux run; its four notes taken at 73145abd (the temporary file goes through the symlink guard); main de49485f merged in at 50f0fd0c, own lines byte-identical; full lane green at 34fd2a92."
closed: 2026-11-20
outcome: done
---
# A new record is written whole or not at all, so a full disk or a file-size limit never leaves a 0-byte or cut-off record at its name, and an empty record is named as such (#260, #352)

## Goal

A new record is written whole or not at all, so a full disk or a file-size limit never leaves a 0-byte or cut-off record at its name, and an empty record is named as such (#260, #352)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-260-a-new-record-is-written-whole-or-not-at-all (native approval)": "Standing OK, tell me after" (Leif's standing OK for scope approvals (2026-11-20 dialog, relayed via t4c): his answer 'Standing OK, tell me after' in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 04:17Z. Confirmed by t4c for this change on fable-lead's summary (one criterion added to each changed requirement, nothing removed, no refusal). Recorded by t4c itself, at the draft b7550830); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve fix-260-a-new-record-is-written-whole-or-not-at-all as 0xLeif
- 0xLeif decided "One send-back of fix-260-a-new-record-is-written-whole-or-not-at-all to add src/checkpoint.rs to its paths": "Standing OK, tell me after" (Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: src/checkpoint.rs joins #260's paths so its approved exFAT criterion holds, and #260 also closes #424; no delta word changes, no new refusal. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of fix-260-a-new-record-is-written-whole-or-not-at-all with src/checkpoint.rs added to its paths (native approval)": "Standing OK, tell me after" (Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: src/checkpoint.rs joins #260's paths so its approved exFAT criterion holds, and #260 also closes #424; no delta word changes, no new refusal. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve fix-260-a-new-record-is-written-whole-or-not-at-all as 0xLeif
- 0xLeif decided "Implementation review of fix-260-a-new-record-is-written-whole-or-not-at-all": "Go" (Standing OK 6(b): Leif's 'Go' to the six rules (Claude Code chat message of 2026-11-19, relayed by his planning session hc t4c), recording his implementation review of a batch with a tester PASS that changes no approved sentence. The tester's PASS at 829d479b (0 cut-off records against main's 130 of 544; real repositories byte-identical; the full set 3 fixed, 0 new); corvid-agent's approval at 829d479b with its Linux run. Then corvid-agent's four notes at 73145abd: the temporary file goes through the symlink guard (a unit test red on 829d479b), a comment on the signal guard, a docs sentence on the unsynced folder, and the history row's order; no approved sentence changed. Main de49485f (#422) merged in at 50f0fd0c: t4b's script shows its own changed lines byte-identical before and after, the cli spec only renumbered to 130, and the full lane green at 34fd2a92. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change review fix-260-a-new-record-is-written-whole-or-not-at-all as 0xLeif with verdict pass

## What was verified

- 4 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record for TC-records-a-cut-off-write-leaves-no-record: passed (exit 0, tests run 1). - e4ebc789c36a; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-a-cut-off-write-leaves-no-record-ffa649
- Ran cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty for TC-records-an-empty-record-is-named-as-empty: passed (exit 0, tests run 1). - dc7e2ea6aadd; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-an-empty-record-is-named-as-empty-113250

## Bugs

None.

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
- 2026-11-20T18:58:06Z · gate-closed · accept → accept · Implementation review of fix-260-a-new-record-is-written-whole-or-not-at-all ^t-f6b88a
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: af015ccb56f710f6946cf99878980cbf74b41360
  - relayed_by:
    - decided_by: 0xLeif
    - words: Go
    - where: Standing OK 6(b): Leif's 'Go' to the six rules (Claude Code chat message of 2026-11-19, relayed by his planning session hc t4c), recording his implementation review of a batch with a tester PASS that changes no approved sentence. The tester's PASS at 829d479b (0 cut-off records against main's 130 of 544; real repositories byte-identical; the full set 3 fixed, 0 new); corvid-agent's approval at 829d479b with its Linux run. Then corvid-agent's four notes at 73145abd: the temporary file goes through the symlink guard (a unit test red on 829d479b), a comment on the signal guard, a docs sentence on the unsynced folder, and the history row's order; no approved sentence changed. Main de49485f (#422) merged in at 50f0fd0c: t4b's script shows its own changed lines byte-identical before and after, the cli spec only renumbered to 130, and the full lane green at 34fd2a92. Recorded by t4c
    - recorded_by: t4c
    - native: specsync change review fix-260-a-new-record-is-written-whole-or-not-at-all
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T18:58:07Z · advance · accept → done · native:archived ^t-919eab
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - gate: native:archived
  - revision: af015ccb56f710f6946cf99878980cbf74b41360
  - by: t4c (claude-opus-5-5, hc t4c)

Closed 2026-11-20: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-20-tc-records-a-cut-off-write-leaves-no-record
    claim: "Ran cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record for TC-records-a-cut-off-write-leaves-no-record: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record
    revision: fcba5f1c433bff099b257add834061ef20d128d1
    environment: local-macos-arm64
    content: 8b4498351d7422a99f5e3262da5e5b6b8289dda8cfd3d68393d7cf783bdbfaa1
    fingerprint: sha256:09bf657c1cd96f6b737843a4621f1d2c58f01ba4be89e660fe6eb58195372c98
    case: TC-records-a-cut-off-write-leaves-no-record
    case_digest: 8958da7751a5e35a7379844b4271e8c9a6de8620daa8ced583ccfde854d50317
    criteria: DRIVE-25
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T16:58:25Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-records-an-empty-record-is-named-as-empty
    claim: "Ran cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty for TC-records-an-empty-record-is-named-as-empty: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty
    revision: fcba5f1c433bff099b257add834061ef20d128d1
    environment: local-macos-arm64
    content: 8b4498351d7422a99f5e3262da5e5b6b8289dda8cfd3d68393d7cf783bdbfaa1
    fingerprint: sha256:fde8ca5b0d33791db406daa875f0e6b51c06eec2d02712ed2c9e8fbd42d0d491
    case: TC-records-an-empty-record-is-named-as-empty
    case_digest: b43130f1d46b0f8006089aed181ac9c3bd56c7612983b2c90e6061be05fc61d3
    criteria: DRIVE-25
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T16:58:28Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-records-a-cut-off-write-leaves-no-record-bf0e58
    claim: "Ran cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record for TC-records-a-cut-off-write-leaves-no-record: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record
    revision: 8418dff2f5fcfaff322fcee860159ad82e6a593d
    environment: local-macos-arm64
    content: 5ec53b8bda35b385cc4e52ed962166b271b604f4149eee7e0e597aa2f51a987c
    fingerprint: sha256:f745f157aa1215c169ef9eba177c822901e1437159cb87455d5abff126f8efea
    case: TC-records-a-cut-off-write-leaves-no-record
    case_digest: 8958da7751a5e35a7379844b4271e8c9a6de8620daa8ced583ccfde854d50317
    criteria: DRIVE-25
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T17:56:44Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-records-an-empty-record-is-named-as-empty-97908b
    claim: "Ran cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty for TC-records-an-empty-record-is-named-as-empty: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty
    revision: 8418dff2f5fcfaff322fcee860159ad82e6a593d
    environment: local-macos-arm64
    content: 5ec53b8bda35b385cc4e52ed962166b271b604f4149eee7e0e597aa2f51a987c
    fingerprint: sha256:d447e14763614a2a034a87b6e117ccf7f0482f69c8f67e7b207dbef993f28e57
    case: TC-records-an-empty-record-is-named-as-empty
    case_digest: b43130f1d46b0f8006089aed181ac9c3bd56c7612983b2c90e6061be05fc61d3
    criteria: DRIVE-25
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T17:56:46Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-records-a-cut-off-write-leaves-no-record-ffa649
    claim: "Ran cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record for TC-records-a-cut-off-write-leaves-no-record: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact a_write_cut_off_by_a_size_limit_leaves_no_record
    revision: b5e461265cb8c859ece2aefd1f75db8441b213cd
    environment: local-macos-arm64
    content: 47f48f674af2b26873198e18d643cde1bc9388765919f10320e0d91364a769a5
    fingerprint: sha256:e4ebc789c36a292fe92d12379b34c91a203291eaa0eee1c60e807002e68a80a8
    case: TC-records-a-cut-off-write-leaves-no-record
    case_digest: 8958da7751a5e35a7379844b4271e8c9a6de8620daa8ced583ccfde854d50317
    criteria: DRIVE-25
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T18:49:14Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-records-an-empty-record-is-named-as-empty-113250
    claim: "Ran cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty for TC-records-an-empty-record-is-named-as-empty: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty
    revision: b5e461265cb8c859ece2aefd1f75db8441b213cd
    environment: local-macos-arm64
    content: 47f48f674af2b26873198e18d643cde1bc9388765919f10320e0d91364a769a5
    fingerprint: sha256:dc7e2ea6aaddb38c01baef7efa5f8a7eaf67373bf8425d90dd3aae560ae1da67
    case: TC-records-an-empty-record-is-named-as-empty
    case_digest: b43130f1d46b0f8006089aed181ac9c3bd56c7612983b2c90e6061be05fc61d3
    criteria: DRIVE-25
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T18:49:16Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
```
