---
kind: session
id: 2026-11-19-m1-migrate-refuses-before-writing
aliases: [m1-migrate-refuses-before-writing]
objective: "Migration refuses before it writes and leaves a working tree: any entry at a target or receipt name counts as taken, parallel applies cannot both apply, a returning 0.5.x record already migrated is refused by name with the generated index rebuilt, and doctor --fix keeps a pointer that names an open session by alias (#376, #370, #337, #378; #375 and #338 await Leif's choice)"
stage: done
stage_status: archived
iteration: 1
native_change: fix-m1-migrate-refuses-before-writing
pr: 417
criteria: [DRIVE-27, LIFE-20, LIFE-21]
requirements: [REQ-records-009, REQ-cli-055, REQ-records-015]
observations:
  - 2026-11-20-tc-records-migrate-judges-returning-files-by-the-receipt
  - 2026-11-20-tc-records-migrate-names-every-returning-file-in-one-refusal
  - 2026-11-20-tc-records-migrate-removes-a-returning-file-as-already
  - 2026-11-20-tc-records-migrate-refuses-an-edited-committed-receipt
  - 2026-11-20-tc-records-migrate-an-uncommitted-receipt-judges-nothing
  - 2026-11-20-tc-records-migrate-refuses-an-entry-that-is-no-receipt
  - 2026-11-20-tc-records-migrate-a-special-target-refuses-before-writing
  - 2026-11-20-tc-records-migrate-one-of-several-applies
  - 2026-11-20-tc-records-migrate-a-new-bug-rebuilds-the-index-in-place
  - 2026-11-20-tc-records-migrate-refuses-a-record-whose-id-is-held
  - 2026-11-20-tc-records-migrate-refuses-a-returning-legacy-checkpoint
  - 2026-11-20-tc-records-migrate-a-failed-write-removes-nothing
  - 2026-11-20-tc-cli-doctor-keeps-a-migrated-alias-pointer
  - 2026-11-20-tc-records-migrate-refuses-a-separate-case-colliding-target
  - 2026-11-20-tc-records-migrate-judges-returning-files-by-the-receipt-0e6c40
  - 2026-11-20-tc-records-migrate-names-every-returning-file-in-one-refusal-f6d6c2
  - 2026-11-20-tc-records-migrate-removes-a-returning-file-as-already-392a86
  - 2026-11-20-tc-records-migrate-refuses-an-edited-committed-receipt-f27657
  - 2026-11-20-tc-records-migrate-an-uncommitted-receipt-judges-nothing-d83a22
  - 2026-11-20-tc-records-migrate-refuses-an-entry-that-is-no-receipt-87fa1c
  - 2026-11-20-tc-records-migrate-a-special-target-refuses-before-writing-0155ed
  - 2026-11-20-tc-records-migrate-one-of-several-applies-127d95
  - 2026-11-20-tc-records-migrate-a-new-bug-rebuilds-the-index-in-place-02e9b0
  - 2026-11-20-tc-records-migrate-refuses-a-record-whose-id-is-held-4a1800
  - 2026-11-20-tc-records-migrate-refuses-a-returning-legacy-checkpoint-3cc83a
  - 2026-11-20-tc-records-migrate-a-failed-write-removes-nothing-5c1455
  - 2026-11-20-tc-cli-doctor-keeps-a-migrated-alias-pointer-8e82d3
  - 2026-11-20-tc-records-migrate-judges-returning-files-by-the-receipt-f2391b
  - 2026-11-20-tc-records-migrate-names-every-returning-file-in-one-refusal-4bcd95
  - 2026-11-20-tc-records-migrate-removes-a-returning-file-as-already-0dec3d
  - 2026-11-20-tc-records-migrate-refuses-an-edited-committed-receipt-8a926f
  - 2026-11-20-tc-records-migrate-an-uncommitted-receipt-judges-nothing-f08ebb
  - 2026-11-20-tc-records-migrate-refuses-an-entry-that-is-no-receipt-985401
  - 2026-11-20-tc-records-migrate-a-special-target-refuses-before-writing-aef616
  - 2026-11-20-tc-records-migrate-one-of-several-applies-1b61eb
  - 2026-11-20-tc-records-migrate-a-new-bug-rebuilds-the-index-in-place-c8b356
  - 2026-11-20-tc-records-migrate-refuses-a-record-whose-id-is-held-d53d68
  - 2026-11-20-tc-records-migrate-refuses-a-returning-legacy-checkpoint-68d47f
  - 2026-11-20-tc-records-migrate-a-failed-write-removes-nothing-2eb471
  - 2026-11-20-tc-cli-doctor-keeps-a-migrated-alias-pointer-668349
  - 2026-11-20-tc-records-migrate-skips-macos-metadata
title: Migration refuses before it writes and leaves a working tree
summary:
  - chough migrate reads every target and receipt entry without following a link or waiting and refuses anything in the way before it writes (#337); judges returning 0.5.x files by the committed receipt that lists them (#370); takes the writer lock before it plans, so one apply of a digest applies (#378); removes no 0.5.x file before every planned file is on disk and gives knowledge/index.md its name by moving the 0.5.x INDEX.md aside, on every file system including a Docker Desktop mount (#338); removes folders left holding only macOS metadata and names them (#375); and doctor --fix keeps a pointer that names an open session by alias (#376).
  - "Verified at 75264e00: all eleven verify steps and fourteen cases on macOS; the tester's PASS on macOS (25 set rows fixed, nine real repositories); tester2's PASS on Linux and a Docker Desktop mount; corvid-agent's approval; fable-lead's read."
closed: 2026-11-20
outcome: done
---
# Migration refuses before it writes and leaves a working tree: any entry at a target or receipt name counts as taken, parallel applies cannot both apply, a returning 0.5.x record already migrated is refused by name with the generated index rebuilt, and doctor --fix keeps a pointer that names an open session by alias (#376, #370, #337, #378; #375 and #338 await Leif's choice)

## Goal

Migration refuses before it writes and leaves a working tree: any entry at a target or receipt name counts as taken, parallel applies cannot both apply, a returning 0.5.x record already migrated is refused by name with the generated index rebuilt, and doctor --fix keeps a pointer that names an open session by alias (#376, #370, #337, #378; #375 and #338 await Leif's choice)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-m1-migrate-refuses-before-writing (native approval)": "Approve with the 3 choices" (Scope: his 'Approve with the 3 choices' of the draft at a82f0e4e (Claude Code chat message relayed by hc t4c through fable-lead, hc t53, 2026-11-19). The one path tests/fixtures/migrate/ added on his 'a, all yes.. im back' (relayed the same way, about 00:38Z on 2026-11-20, item 8). Recording confirmed by his direct answer 'Yes, record it' in t4b's Claude Code session, 2026-11-20); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-m1-migrate-refuses-before-writing as 0xLeif
- 0xLeif decided "Re-approval of fix-m1-migrate-refuses-before-writing with REQ-cli-055 rebased onto main under standing OK (d) (native approval)": "Yes, record it" (His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, to recording this re-approval under standing OK (d): his 'yes to both' (relayed by hc t4c through fable-lead, hc t53, about 15:02Z on 2026-11-19) lets a delta stale only because main changed the same requirement be rebased onto main's text with exactly the same added and removed words, checked by fable-lead's script, which passed on 2026-11-20 (delta sha256 15ebb4b32eec2b93...). Scope as approved by his 'Approve with the 3 choices' (2026-11-19) with tests/fixtures/migrate/ added (item 8, 2026-11-20); answers and the 17 paths unchanged); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-m1-migrate-refuses-before-writing as 0xLeif
- 0xLeif decided "One send-back of fix-m1-migrate-refuses-before-writing to add the #263 case file to its paths, run by t4b": "A" (Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 01:24Z on 2026-11-20, choosing the option t4c put to him: 'A (recommended): keep what you approved. The file gets replaced, the preview says so, and #263's test is updated to match. This adds one test file to M1's scope, through a send-back t4b runs. The wording doesn't change.'); recorded by fable-t4b; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of fix-m1-migrate-refuses-before-writing with the #263 case file added to its paths (native approval)": "Yes, record it" (His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, to recording this re-approval after his relayed 'A' (hc t4c through fable-lead, about 01:24Z on 2026-11-20): keep what he approved, the file gets replaced, the preview says so, #263's test is updated to match, one test file added through a send-back t4b runs, no wording change. The one path added is .chough/cases/records.migrate-case-colliding-target.cases.toml; deltas unchanged (cli 15ebb4b32eec2b93, records 37c36f2822fdb97c). The scope stands on his 'Approve with the 3 choices' (2026-11-19), item 8 (2026-11-20) and standing OK (d), as recorded before); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-m1-migrate-refuses-before-writing as 0xLeif
- 0xLeif decided "One send-back of fix-m1-migrate-refuses-before-writing to add the macOS-metadata case file to its paths, run by t4b": "Yes, all of it" (His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, after his relayed 'Ok' (hc t4c through fable-lead, about 03:00Z on 2026-11-20, item 3 of t4c's six-item list): record that word, run one send-back adding .chough/cases/records.migrate-macos-metadata.cases.toml to M1's paths with no delta word changed, retitle that case to the approved REQ-records-015 criterion, and record his re-approval); recorded by fable-t4b; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of fix-m1-migrate-refuses-before-writing with the macOS-metadata case file added to its paths (native approval)": "Yes, all of it" (His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, after his relayed 'Ok' (hc t4c through fable-lead, about 03:00Z, item 3): the one path .chough/cases/records.migrate-macos-metadata.cases.toml added; deltas unchanged (cli 15ebb4b32eec2b93, records 37c36f2822fdb97c). The scope stands on his 'Approve with the 3 choices' (2026-11-19), item 8 and 'A' (2026-11-20) and standing OK (d), as recorded before); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-m1-migrate-refuses-before-writing as 0xLeif
- 0xLeif decided "Implementation review of fix-m1-migrate-refuses-before-writing": "Go" (Standing OK 6(b): Leif's 'Go' to the six rules (Claude Code chat message of 2026-11-19, relayed by his planning session hc t4c), recording his implementation review of a batch with a tester PASS that changes no approved sentence. At 75264e00: the tester's PASS on macOS, tester2's PASS on Linux and a Docker Desktop mount, corvid-agent's approval, fable-lead's read of the whole diff against main; no approved sentence changed by the build. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change review fix-m1-migrate-refuses-before-writing as 0xLeif with verdict pass

## What was verified

- 26 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test migrate_knowledge -- --exact a_separate_case_colliding_target_is_replaced_only_as_previewed for TC-records-migrate-refuses-a-separate-case-colliding-target: passed (exit 0, tests run 1). - 968094ce8c66; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-refuses-a-separate-case-colliding-target
- Ran cargo test --locked --test migrate -- --exact a_returning_edited_session_is_refused_naming_it_and_its_receipt for TC-records-migrate-judges-returning-files-by-the-receipt: passed (exit 0, tests run 1). - 39b6339377aa; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-judges-returning-files-by-the-receipt-f2391b
- Ran cargo test --locked --test migrate -- --exact returning_files_with_other_bytes_are_all_named_in_one_refusal for TC-records-migrate-names-every-returning-file-in-one-refusal: passed (exit 0, tests run 1). - 1cfaaa361bb3; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-names-every-returning-file-in-one-refusal-4bcd95
- Ran cargo test --locked --test migrate -- --exact a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated for TC-records-migrate-removes-a-returning-file-as-already-migrated: passed (exit 0, tests run 1). - a2b9fc694cbb; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-removes-a-returning-file-as-already-0dec3d
- Ran cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored for TC-records-migrate-refuses-an-edited-committed-receipt: passed (exit 0, tests run 1). - c4b3e660274d; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-refuses-an-edited-committed-receipt-8a926f
- Ran cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does for TC-records-migrate-an-uncommitted-receipt-judges-nothing: passed (exit 0, tests run 1). - db814052503e; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-an-uncommitted-receipt-judges-nothing-f08ebb
- Ran cargo test --locked --test migrate -- --exact an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting for TC-records-migrate-refuses-an-entry-that-is-no-receipt: passed (exit 0, tests run 1). - 7b614277bc01; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-refuses-an-entry-that-is-no-receipt-985401
- Ran cargo test --locked --test migrate -- --exact a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting for TC-records-migrate-a-special-target-refuses-before-writing: passed (exit 0, tests run 1). - 03b22611829a; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-a-special-target-refuses-before-writing-aef616
- Ran cargo test --locked --test migrate -- --exact of_three_applies_of_one_digest_only_one_applies for TC-records-migrate-one-of-several-applies: passed (exit 0, tests run 1). - e00661215721; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-one-of-several-applies-1b61eb
- Ran cargo test --locked --test migrate -- --exact a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place for TC-records-migrate-a-new-bug-rebuilds-the-index-in-place: passed (exit 0, tests run 1). - a399f5c2e04d; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-a-new-bug-rebuilds-the-index-in-place-c8b356
- Ran cargo test --locked --test migrate -- --exact a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused for TC-records-migrate-refuses-a-record-whose-id-is-held: passed (exit 0, tests run 1). - 9b40c331cfbb; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-refuses-a-record-whose-id-is-held-d53d68
- Ran cargo test --locked --test migrate -- --exact a_returning_legacy_checkpoint_holding_another_session_is_refused for TC-records-migrate-refuses-a-returning-legacy-checkpoint: passed (exit 0, tests run 1). - 21e4919ed47d; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-refuses-a-returning-legacy-checkpoint-68d47f
- Ran cargo test --locked --test migrate -- --exact an_apply_whose_planned_write_fails_leaves_every_0_5_x_file for TC-records-migrate-a-failed-write-removes-nothing: passed (exit 0, tests run 1). - dfc3cc50df57; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-a-failed-write-removes-nothing-2eb471
- Ran cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias for TC-cli-doctor-keeps-a-migrated-alias-pointer: passed (exit 0, tests run 1). - afbc08af1093; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-doctor-keeps-a-migrated-alias-pointer-668349
- Ran cargo test --locked --test migrate -- --exact migrate_skips_macos_metadata_beside_0_5_records for TC-records-migrate-skips-macos-metadata: passed (exit 0, tests run 1). - ca40d21a5b67; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-migrate-skips-macos-metadata

## Bugs

None.

## Open threads

## Timeline

- 2026-11-19T06:11:08Z · start · intent → intent · Migration refuses before it writes and leaves a working tree: any entry at a target or receipt name counts as taken, parallel applies cannot both apply, a returning 0.5.x record already migrated is refused by name with the generated index rebuilt, and doctor --fix keeps a pointer that names an open session by alias (#376, #370, #337, #378; #375 and #338 await Leif's choice) ^t-6a8318
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T06:11:08Z · advance · intent → design ·  ^t-77d10f
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T06:11:09Z · skip · design → plan · Bug fixes to migration refusals and one doctor check; nothing visual or interactive changes ^t-00412e
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-20T00:44:43Z · gate-opened · plan → plan · Scope approval of fix-m1-migrate-refuses-before-writing (native approval) ^t-6f005e
  - revision: a82f0e4e83c5cc8dbe3d42ea28f108df99281f0a
  - holds: plan
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:44:58Z · gate-closed · plan → plan · Scope approval of fix-m1-migrate-refuses-before-writing (native approval) ^t-b2a435
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: a82f0e4e83c5cc8dbe3d42ea28f108df99281f0a
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve with the 3 choices
    - where: Scope: his 'Approve with the 3 choices' of the draft at a82f0e4e (Claude Code chat message relayed by hc t4c through fable-lead, hc t53, 2026-11-19). The one path tests/fixtures/migrate/ added on his 'a, all yes.. im back' (relayed the same way, about 00:38Z on 2026-11-20, item 8). Recording confirmed by his direct answer 'Yes, record it' in t4b's Claude Code session, 2026-11-20
    - recorded_by: fable-t4b
    - native: specsync change approve fix-m1-migrate-refuses-before-writing
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T01:04:47Z · advance · plan → build · native:approved ^t-d0c11e
  - native_change: fix-m1-migrate-refuses-before-writing
  - gate: native:approved
  - revision: 421a36220a07700acde2002895e5b599f0918caf
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T01:16:24Z · gate-opened · build → build · Re-approval of fix-m1-migrate-refuses-before-writing with REQ-cli-055 rebased onto main under standing OK (d) (native approval) ^t-59fdbe
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: 421a36220a07700acde2002895e5b599f0918caf
  - holds: build
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T01:16:24Z · direct-native-write · build → build · fix-m1-migrate-refuses-before-writing's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-13c072
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: 421a36220a07700acde2002895e5b599f0918caf
  - direct_write:
    - change: fix-m1-migrate-refuses-before-writing
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T01:16:24Z · gate-closed · build → build · Re-approval of fix-m1-migrate-refuses-before-writing with REQ-cli-055 rebased onto main under standing OK (d) (native approval) ^t-bc8a7d
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: 421a36220a07700acde2002895e5b599f0918caf
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, record it
    - where: His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, to recording this re-approval under standing OK (d): his 'yes to both' (relayed by hc t4c through fable-lead, hc t53, about 15:02Z on 2026-11-19) lets a delta stale only because main changed the same requirement be rebased onto main's text with exactly the same added and removed words, checked by fable-lead's script, which passed on 2026-11-20 (delta sha256 15ebb4b32eec2b93...). Scope as approved by his 'Approve with the 3 choices' (2026-11-19) with tests/fixtures/migrate/ added (item 8, 2026-11-20); answers and the 17 paths unchanged
    - recorded_by: fable-t4b
    - native: specsync change approve fix-m1-migrate-refuses-before-writing
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T01:25:13Z · gate-opened · build → build · One send-back of fix-m1-migrate-refuses-before-writing to add the #263 case file to its paths, run by t4b ^t-13bd80
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: 30b71fbbd19949e66af3500a25c615aa60b41e39
  - holds: build
  - drives: none
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T01:25:14Z · gate-closed · build → build · One send-back of fix-m1-migrate-refuses-before-writing to add the #263 case file to its paths, run by t4b ^t-8bf367
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: 30b71fbbd19949e66af3500a25c615aa60b41e39
  - relayed_by:
    - decided_by: 0xLeif
    - words: A
    - where: Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 01:24Z on 2026-11-20, choosing the option t4c put to him: 'A (recommended): keep what you approved. The file gets replaced, the preview says so, and #263's test is updated to match. This adds one test file to M1's scope, through a send-back t4b runs. The wording doesn't change.'
    - recorded_by: fable-t4b
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T01:33:33Z · gate-opened · build → build · Re-approval of fix-m1-migrate-refuses-before-writing with the #263 case file added to its paths (native approval) ^t-b0efd5
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: 30b71fbbd19949e66af3500a25c615aa60b41e39
  - holds: build
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T01:33:33Z · direct-native-write · build → build · fix-m1-migrate-refuses-before-writing's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-0eb960
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: 30b71fbbd19949e66af3500a25c615aa60b41e39
  - direct_write:
    - change: fix-m1-migrate-refuses-before-writing
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T01:33:33Z · gate-closed · build → build · Re-approval of fix-m1-migrate-refuses-before-writing with the #263 case file added to its paths (native approval) ^t-9d010f
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: 30b71fbbd19949e66af3500a25c615aa60b41e39
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, record it
    - where: His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, to recording this re-approval after his relayed 'A' (hc t4c through fable-lead, about 01:24Z on 2026-11-20): keep what he approved, the file gets replaced, the preview says so, #263's test is updated to match, one test file added through a send-back t4b runs, no wording change. The one path added is .chough/cases/records.migrate-case-colliding-target.cases.toml; deltas unchanged (cli 15ebb4b32eec2b93, records 37c36f2822fdb97c). The scope stands on his 'Approve with the 3 choices' (2026-11-19), item 8 (2026-11-20) and standing OK (d), as recorded before
    - recorded_by: fable-t4b
    - native: specsync change approve fix-m1-migrate-refuses-before-writing
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T03:22:11Z · gate-opened · build → build · One send-back of fix-m1-migrate-refuses-before-writing to add the macOS-metadata case file to its paths, run by t4b ^t-7b1811
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: bacde95a18ce2a789814bb4b1b6b7179ec381125
  - holds: build
  - drives: none
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T03:22:11Z · gate-closed · build → build · One send-back of fix-m1-migrate-refuses-before-writing to add the macOS-metadata case file to its paths, run by t4b ^t-789ae8
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: bacde95a18ce2a789814bb4b1b6b7179ec381125
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, all of it
    - where: His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, after his relayed 'Ok' (hc t4c through fable-lead, about 03:00Z on 2026-11-20, item 3 of t4c's six-item list): record that word, run one send-back adding .chough/cases/records.migrate-macos-metadata.cases.toml to M1's paths with no delta word changed, retitle that case to the approved REQ-records-015 criterion, and record his re-approval
    - recorded_by: fable-t4b
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T03:22:21Z · gate-opened · build → build · Re-approval of fix-m1-migrate-refuses-before-writing with the macOS-metadata case file added to its paths (native approval) ^t-3b6dc4
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: bacde95a18ce2a789814bb4b1b6b7179ec381125
  - holds: build
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T03:22:21Z · direct-native-write · build → build · fix-m1-migrate-refuses-before-writing's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-48f3ae
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: bacde95a18ce2a789814bb4b1b6b7179ec381125
  - direct_write:
    - change: fix-m1-migrate-refuses-before-writing
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T03:22:21Z · gate-closed · build → build · Re-approval of fix-m1-migrate-refuses-before-writing with the macOS-metadata case file added to its paths (native approval) ^t-0b6fb4
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: bacde95a18ce2a789814bb4b1b6b7179ec381125
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, all of it
    - where: His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, after his relayed 'Ok' (hc t4c through fable-lead, about 03:00Z, item 3): the one path .chough/cases/records.migrate-macos-metadata.cases.toml added; deltas unchanged (cli 15ebb4b32eec2b93, records 37c36f2822fdb97c). The scope stands on his 'Approve with the 3 choices' (2026-11-19), item 8 and 'A' (2026-11-20) and standing OK (d), as recorded before
    - recorded_by: fable-t4b
    - native: specsync change approve fix-m1-migrate-refuses-before-writing
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T06:23:41Z · advance · build → test · native:verifying ^t-aeb4f7
  - native_change: fix-m1-migrate-refuses-before-writing
  - gate: native:verifying
  - revision: 75264e0039a3957affd98dca1708de2819c2af13
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T06:23:41Z · advance · test → accept · native:verifying ^t-b33754
  - native_change: fix-m1-migrate-refuses-before-writing
  - gate: native:verifying
  - revision: 75264e0039a3957affd98dca1708de2819c2af13
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T06:23:41Z · gate-opened · accept → accept · Implementation review of fix-m1-migrate-refuses-before-writing ^t-8e4c3a
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: 75264e0039a3957affd98dca1708de2819c2af13
  - holds: accept
  - drives: review
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T06:25:05Z · gate-closed · accept → accept · Implementation review of fix-m1-migrate-refuses-before-writing ^t-03b554
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: 8072cd903c1e575727c5a3824d706cccfd949817
  - relayed_by:
    - decided_by: 0xLeif
    - words: Go
    - where: Standing OK 6(b): Leif's 'Go' to the six rules (Claude Code chat message of 2026-11-19, relayed by his planning session hc t4c), recording his implementation review of a batch with a tester PASS that changes no approved sentence. At 75264e00: the tester's PASS on macOS, tester2's PASS on Linux and a Docker Desktop mount, corvid-agent's approval, fable-lead's read of the whole diff against main; no approved sentence changed by the build. Recorded by t4c
    - recorded_by: t4c
    - native: specsync change review fix-m1-migrate-refuses-before-writing
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T06:25:06Z · advance · accept → done · native:archived ^t-0d4bfe
  - native_change: fix-m1-migrate-refuses-before-writing
  - gate: native:archived
  - revision: 8072cd903c1e575727c5a3824d706cccfd949817
  - by: t4c (claude-opus-5-5, hc t4c)

Closed 2026-11-20: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-20-tc-records-migrate-judges-returning-files-by-the-receipt
    claim: "Ran cargo test --locked --test migrate -- --exact a_returning_edited_session_is_refused_naming_it_and_its_receipt for TC-records-migrate-judges-returning-files-by-the-receipt: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_returning_edited_session_is_refused_naming_it_and_its_receipt
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:637f8e12bac3bc35a9dffccd0cb3447b6a24d591cac4945b3e3292ebf0e8431f
    case: TC-records-migrate-judges-returning-files-by-the-receipt
    case_digest: 6c7cfd3ac789b843371f448c58173940d0035062ba7f60ac2f55b3018ca95734
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:23:36Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-names-every-returning-file-in-one-refusal
    claim: "Ran cargo test --locked --test migrate -- --exact returning_files_with_other_bytes_are_all_named_in_one_refusal for TC-records-migrate-names-every-returning-file-in-one-refusal: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact returning_files_with_other_bytes_are_all_named_in_one_refusal
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:ac81b237743fac27d59093dc2670315707e2ffb0549994326a2e2186c1d4ba17
    case: TC-records-migrate-names-every-returning-file-in-one-refusal
    case_digest: 608c0d0ceb5ba1f61b81faeea361b40abbe25d69d36139596a755d00c15ea572
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:23:40Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-removes-a-returning-file-as-already
    claim: "Ran cargo test --locked --test migrate -- --exact a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated for TC-records-migrate-removes-a-returning-file-as-already-migrated: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:933ad8c4199f41562eb3cc60872dad27ec37f7d7055619dc4d9709e2b6c9f067
    case: TC-records-migrate-removes-a-returning-file-as-already-migrated
    case_digest: 1dfb5e9be9cf0d8e6c2558dbdea62c5f17e77536335642d5e1ee559848fd4e39
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:23:44Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-an-edited-committed-receipt
    claim: "Ran cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored for TC-records-migrate-refuses-an-edited-committed-receipt: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:4cbb794903418a255399d67d5cdcd4f7ac968200ba9b87cfb7543bd5457c08b7
    case: TC-records-migrate-refuses-an-edited-committed-receipt
    case_digest: 365b8b4c3b4cd60cec0c86242516373eedbe64be6894863ce037fa128aa1e103
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:23:47Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-an-uncommitted-receipt-judges-nothing
    claim: "Ran cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does for TC-records-migrate-an-uncommitted-receipt-judges-nothing: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:046f7b5e06f99d5d9351acbd363d03a2cc2db854b150ca8694a269aa8135edea
    case: TC-records-migrate-an-uncommitted-receipt-judges-nothing
    case_digest: b0e42920b60fac10644ef01752dd88c198e7f0b3a4c97573934f7846643694ca
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:23:51Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-an-entry-that-is-no-receipt
    claim: "Ran cargo test --locked --test migrate -- --exact an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting for TC-records-migrate-refuses-an-entry-that-is-no-receipt: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:14c6bd855dd868d460bd0cb43571e5bb7f649b92a7f9983a4797b6490eb9c336
    case: TC-records-migrate-refuses-an-entry-that-is-no-receipt
    case_digest: 0c9383d5653d9060bd4989b5a8834a33afbce388d238caee3b16dcc1ed87b2ef
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:23:55Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-a-special-target-refuses-before-writing
    claim: "Ran cargo test --locked --test migrate -- --exact a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting for TC-records-migrate-a-special-target-refuses-before-writing: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:b4581ecd8d581f69482492a721e7cab8697bd42cc94f0d2e99c1587f1887f8fa
    case: TC-records-migrate-a-special-target-refuses-before-writing
    case_digest: f9a9b4bcd8ec15bd9a2d3f85e58c0e1b2c75880af7fac9fb1de4103e61dea597
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:24:02Z
    duration_seconds: 4
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-one-of-several-applies
    claim: "Ran cargo test --locked --test migrate -- --exact of_three_applies_of_one_digest_only_one_applies for TC-records-migrate-one-of-several-applies: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact of_three_applies_of_one_digest_only_one_applies
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:4cd51ce18eabb365fd00aaca12e814f59ba7f6c647e504ee82aba9235f22a5a6
    case: TC-records-migrate-one-of-several-applies
    case_digest: 2612e544ae7561ba82fc9ec204edec2b97fa28464c621b5a215132c5d2bc0b18
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:24:11Z
    duration_seconds: 7
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-a-new-bug-rebuilds-the-index-in-place
    claim: "Ran cargo test --locked --test migrate -- --exact a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place for TC-records-migrate-a-new-bug-rebuilds-the-index-in-place: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:b9c12bfc5c5f141838371c27bfedf40e0f51a3f6e7294d4e6692a43c20a47686
    case: TC-records-migrate-a-new-bug-rebuilds-the-index-in-place
    case_digest: 99c3ddff20c586e5ad229f40b496f086bdeed59ca2988decaa488c3b21eddd83
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:24:15Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-a-record-whose-id-is-held
    claim: "Ran cargo test --locked --test migrate -- --exact a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused for TC-records-migrate-refuses-a-record-whose-id-is-held: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:571e314dc5e9cd51698bd1a9d3dc87d0f97ff88b0ad858ecf7e35de8db0d03a0
    case: TC-records-migrate-refuses-a-record-whose-id-is-held
    case_digest: 6f451587dac58b87c23df4915220cba3f6c0ebf8f489832ca9128dbb1b79f435
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:24:19Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-a-returning-legacy-checkpoint
    claim: "Ran cargo test --locked --test migrate -- --exact a_returning_legacy_checkpoint_holding_another_session_is_refused for TC-records-migrate-refuses-a-returning-legacy-checkpoint: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_returning_legacy_checkpoint_holding_another_session_is_refused
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:5ca1f592fe54e3eac080b29c2e8bdb9a5cea913b13a62c70e66fb74d2a96a4ed
    case: TC-records-migrate-refuses-a-returning-legacy-checkpoint
    case_digest: 2718a4355329e350b2d940a2ce154c38a5a41d0ce93402442a815a48b58854d0
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:24:21Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-a-failed-write-removes-nothing
    claim: "Ran cargo test --locked --test migrate -- --exact an_apply_whose_planned_write_fails_leaves_every_0_5_x_file for TC-records-migrate-a-failed-write-removes-nothing: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_apply_whose_planned_write_fails_leaves_every_0_5_x_file
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:db63009502dbf5a78d4d3fbf92c4ecce61283057b01003085217da5f263ff5f3
    case: TC-records-migrate-a-failed-write-removes-nothing
    case_digest: 72d5932eebb5ce0f17c5d5cb6cd34a44f1ba11bd5ecf5ad8beed21026ce8b7d3
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:24:24Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-doctor-keeps-a-migrated-alias-pointer
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias for TC-cli-doctor-keeps-a-migrated-alias-pointer: not-run (exit 101, tests run 0)."
    result: not-run
    command: cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:c6c8d1a534c9744884a60e85812e9fb7cfd602f47f45b1521c347567fc2ecbe1
    case: TC-cli-doctor-keeps-a-migrated-alias-pointer
    case_digest: aaa3e31d7e306eb17a846963bba55d7497375e523063f99d8f280035a3dd741d
    criteria: DRIVE-17
    requirements: REQ-cli-055
    limitations: "cargo exited 101 before reporting any test result: a build or environment failure, not an assertion about the behavior; inherited environment variables were not recorded"
    observed: 2026-11-20T02:24:26Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-a-separate-case-colliding-target
    claim: "Ran cargo test --locked --test migrate_knowledge -- --exact a_separate_case_colliding_target_is_replaced_only_as_previewed for TC-records-migrate-refuses-a-separate-case-colliding-target: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate_knowledge -- --exact a_separate_case_colliding_target_is_replaced_only_as_previewed
    revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
    environment: local-macos-arm64
    content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
    fingerprint: sha256:968094ce8c664a26f5193163c3b729b6a21b1d0402efd36a80aa94b94c5a5ddd
    case: TC-records-migrate-refuses-a-separate-case-colliding-target
    case_digest: 88dc76561c5a6932090b7d2af1867c1b84a3f9fdf78cae929276c8560ca7e523
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:24:29Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-judges-returning-files-by-the-receipt-0e6c40
    claim: "Ran cargo test --locked --test migrate -- --exact a_returning_edited_session_is_refused_naming_it_and_its_receipt for TC-records-migrate-judges-returning-files-by-the-receipt: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_returning_edited_session_is_refused_naming_it_and_its_receipt
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:7d0900b007b0899578e9ecc222aaa8fafb8a00f6ec3d4be53fb5639e789f48ae
    case: TC-records-migrate-judges-returning-files-by-the-receipt
    case_digest: 6c7cfd3ac789b843371f448c58173940d0035062ba7f60ac2f55b3018ca95734
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:35:41Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-names-every-returning-file-in-one-refusal-f6d6c2
    claim: "Ran cargo test --locked --test migrate -- --exact returning_files_with_other_bytes_are_all_named_in_one_refusal for TC-records-migrate-names-every-returning-file-in-one-refusal: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact returning_files_with_other_bytes_are_all_named_in_one_refusal
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:bba8b912c3b6235ce7ced3581d20f13cac0f9f0e40efa8561268e52478a374da
    case: TC-records-migrate-names-every-returning-file-in-one-refusal
    case_digest: 608c0d0ceb5ba1f61b81faeea361b40abbe25d69d36139596a755d00c15ea572
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:35:44Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-removes-a-returning-file-as-already-392a86
    claim: "Ran cargo test --locked --test migrate -- --exact a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated for TC-records-migrate-removes-a-returning-file-as-already-migrated: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:8b04481c2924f97a205e1c5447ed1a7c9647b2447d8077e4f5e067e7607f0029
    case: TC-records-migrate-removes-a-returning-file-as-already-migrated
    case_digest: 1dfb5e9be9cf0d8e6c2558dbdea62c5f17e77536335642d5e1ee559848fd4e39
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:35:47Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-an-edited-committed-receipt-f27657
    claim: "Ran cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored for TC-records-migrate-refuses-an-edited-committed-receipt: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:89edc8dbe630341866e4a6a395cc01b7b138ed9de9700782c7278fe2e6ced72f
    case: TC-records-migrate-refuses-an-edited-committed-receipt
    case_digest: 365b8b4c3b4cd60cec0c86242516373eedbe64be6894863ce037fa128aa1e103
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:35:50Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-an-uncommitted-receipt-judges-nothing-d83a22
    claim: "Ran cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does for TC-records-migrate-an-uncommitted-receipt-judges-nothing: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:13de8893b991a393dac32581ec68ac753b81dc6dc6917fcc79b0afc9dda6192d
    case: TC-records-migrate-an-uncommitted-receipt-judges-nothing
    case_digest: b0e42920b60fac10644ef01752dd88c198e7f0b3a4c97573934f7846643694ca
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:35:54Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-an-entry-that-is-no-receipt-87fa1c
    claim: "Ran cargo test --locked --test migrate -- --exact an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting for TC-records-migrate-refuses-an-entry-that-is-no-receipt: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:71a0c6d20dd57c0f459da14caee5402af9e2ef2f2a7e175b5783ef4efdf4dbf6
    case: TC-records-migrate-refuses-an-entry-that-is-no-receipt
    case_digest: 0c9383d5653d9060bd4989b5a8834a33afbce388d238caee3b16dcc1ed87b2ef
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:35:57Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-a-special-target-refuses-before-writing-0155ed
    claim: "Ran cargo test --locked --test migrate -- --exact a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting for TC-records-migrate-a-special-target-refuses-before-writing: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:702b22f99192a887589adeb9f22f436b73c81eccb71f757018c36b4d7391abfc
    case: TC-records-migrate-a-special-target-refuses-before-writing
    case_digest: f9a9b4bcd8ec15bd9a2d3f85e58c0e1b2c75880af7fac9fb1de4103e61dea597
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:36:02Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-one-of-several-applies-127d95
    claim: "Ran cargo test --locked --test migrate -- --exact of_three_applies_of_one_digest_only_one_applies for TC-records-migrate-one-of-several-applies: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact of_three_applies_of_one_digest_only_one_applies
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:a4367c0389310f0e497f50f0ccd59b371d5b8e82b3e4be980424152093bbe042
    case: TC-records-migrate-one-of-several-applies
    case_digest: 2612e544ae7561ba82fc9ec204edec2b97fa28464c621b5a215132c5d2bc0b18
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:36:18Z
    duration_seconds: 14
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-a-new-bug-rebuilds-the-index-in-place-02e9b0
    claim: "Ran cargo test --locked --test migrate -- --exact a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place for TC-records-migrate-a-new-bug-rebuilds-the-index-in-place: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:470b8d4ed4b470e1ba4845800e0fcb72799e315fde80785128798caf815ae9f1
    case: TC-records-migrate-a-new-bug-rebuilds-the-index-in-place
    case_digest: 99c3ddff20c586e5ad229f40b496f086bdeed59ca2988decaa488c3b21eddd83
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:36:26Z
    duration_seconds: 5
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-a-record-whose-id-is-held-4a1800
    claim: "Ran cargo test --locked --test migrate -- --exact a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused for TC-records-migrate-refuses-a-record-whose-id-is-held: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:f59f7c6a4298ef7e9a8a55d149b184706603260d6353e76d147abbd82d5928fd
    case: TC-records-migrate-refuses-a-record-whose-id-is-held
    case_digest: 6f451587dac58b87c23df4915220cba3f6c0ebf8f489832ca9128dbb1b79f435
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:36:33Z
    duration_seconds: 5
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-a-returning-legacy-checkpoint-3cc83a
    claim: "Ran cargo test --locked --test migrate -- --exact a_returning_legacy_checkpoint_holding_another_session_is_refused for TC-records-migrate-refuses-a-returning-legacy-checkpoint: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_returning_legacy_checkpoint_holding_another_session_is_refused
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:1ddf47d9b03fcf15802ef242e9867da6915806b648c2e44b5fce60bd8403a1b3
    case: TC-records-migrate-refuses-a-returning-legacy-checkpoint
    case_digest: 2718a4355329e350b2d940a2ce154c38a5a41d0ce93402442a815a48b58854d0
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:36:38Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-a-failed-write-removes-nothing-5c1455
    claim: "Ran cargo test --locked --test migrate -- --exact an_apply_whose_planned_write_fails_leaves_every_0_5_x_file for TC-records-migrate-a-failed-write-removes-nothing: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_apply_whose_planned_write_fails_leaves_every_0_5_x_file
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:b8ab6c87826618471121b88084988d0ce3085f1d4983cef224c01bdbdd1111fa
    case: TC-records-migrate-a-failed-write-removes-nothing
    case_digest: 72d5932eebb5ce0f17c5d5cb6cd34a44f1ba11bd5ecf5ad8beed21026ce8b7d3
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:36:43Z
    duration_seconds: 3
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-doctor-keeps-a-migrated-alias-pointer-8e82d3
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias for TC-cli-doctor-keeps-a-migrated-alias-pointer: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias
    revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
    environment: local-macos-arm64
    content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
    fingerprint: sha256:335bb1e921776b632c56c46ee4a93abb7cdf6c90a6c9c59ec57944e525ec6c68
    case: TC-cli-doctor-keeps-a-migrated-alias-pointer
    case_digest: aaa3e31d7e306eb17a846963bba55d7497375e523063f99d8f280035a3dd741d
    criteria: DRIVE-17
    requirements: REQ-cli-055
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T02:36:51Z
    duration_seconds: 5
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-judges-returning-files-by-the-receipt-f2391b
    claim: "Ran cargo test --locked --test migrate -- --exact a_returning_edited_session_is_refused_naming_it_and_its_receipt for TC-records-migrate-judges-returning-files-by-the-receipt: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_returning_edited_session_is_refused_naming_it_and_its_receipt
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:39b6339377aadf988e593ec0e2eb5c1480df988ab54c43968cc158a51b156215
    case: TC-records-migrate-judges-returning-files-by-the-receipt
    case_digest: 6c7cfd3ac789b843371f448c58173940d0035062ba7f60ac2f55b3018ca95734
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:58:32Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-names-every-returning-file-in-one-refusal-4bcd95
    claim: "Ran cargo test --locked --test migrate -- --exact returning_files_with_other_bytes_are_all_named_in_one_refusal for TC-records-migrate-names-every-returning-file-in-one-refusal: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact returning_files_with_other_bytes_are_all_named_in_one_refusal
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:1cfaaa361bb3e98dd26bbc4052fac79520eb0f6a14f16851f11750ab08ee70ce
    case: TC-records-migrate-names-every-returning-file-in-one-refusal
    case_digest: 608c0d0ceb5ba1f61b81faeea361b40abbe25d69d36139596a755d00c15ea572
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:58:35Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-removes-a-returning-file-as-already-0dec3d
    claim: "Ran cargo test --locked --test migrate -- --exact a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated for TC-records-migrate-removes-a-returning-file-as-already-migrated: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:a2b9fc694cbbfc83846ca07a28e7b66c223494bee50b0972717529df68f1589f
    case: TC-records-migrate-removes-a-returning-file-as-already-migrated
    case_digest: 1dfb5e9be9cf0d8e6c2558dbdea62c5f17e77536335642d5e1ee559848fd4e39
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:58:38Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-an-edited-committed-receipt-8a926f
    claim: "Ran cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored for TC-records-migrate-refuses-an-edited-committed-receipt: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_edited_committed_receipt_is_refused_until_committed_or_restored
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:c4b3e660274d2f29350b300ce510e0c6996a4fb21acaf32d3f08f58ab88170d5
    case: TC-records-migrate-refuses-an-edited-committed-receipt
    case_digest: 365b8b4c3b4cd60cec0c86242516373eedbe64be6894863ce037fa128aa1e103
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:58:41Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-an-uncommitted-receipt-judges-nothing-f08ebb
    claim: "Ran cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does for TC-records-migrate-an-uncommitted-receipt-judges-nothing: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_uncommitted_receipt_judges_nothing_and_a_committed_one_does
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:db814052503edca0603aba01405b3d64ca6fe1ceb8eab8c0c69ba76fdd672dac
    case: TC-records-migrate-an-uncommitted-receipt-judges-nothing
    case_digest: b0e42920b60fac10644ef01752dd88c198e7f0b3a4c97573934f7846643694ca
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:58:50Z
    duration_seconds: 7
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-an-entry-that-is-no-receipt-985401
    claim: "Ran cargo test --locked --test migrate -- --exact an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting for TC-records-migrate-refuses-an-entry-that-is-no-receipt: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:7b614277bc01f92e03e4c1b4a1aac1ffa1ef475beeafad216fe3932924469938
    case: TC-records-migrate-refuses-an-entry-that-is-no-receipt
    case_digest: 0c9383d5653d9060bd4989b5a8834a33afbce388d238caee3b16dcc1ed87b2ef
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:59:00Z
    duration_seconds: 7
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-a-special-target-refuses-before-writing-aef616
    claim: "Ran cargo test --locked --test migrate -- --exact a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting for TC-records-migrate-a-special-target-refuses-before-writing: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:03b22611829a5a73703ad17193fab24969774ce3ce1365e7ff46ae24cb9c0a3b
    case: TC-records-migrate-a-special-target-refuses-before-writing
    case_digest: f9a9b4bcd8ec15bd9a2d3f85e58c0e1b2c75880af7fac9fb1de4103e61dea597
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:59:15Z
    duration_seconds: 11
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-one-of-several-applies-1b61eb
    claim: "Ran cargo test --locked --test migrate -- --exact of_three_applies_of_one_digest_only_one_applies for TC-records-migrate-one-of-several-applies: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact of_three_applies_of_one_digest_only_one_applies
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:e006612157217d0f7db6226e2aa83b064f1a1786b2ee18b4f564407a21e0729c
    case: TC-records-migrate-one-of-several-applies
    case_digest: 2612e544ae7561ba82fc9ec204edec2b97fa28464c621b5a215132c5d2bc0b18
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:59:36Z
    duration_seconds: 18
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-a-new-bug-rebuilds-the-index-in-place-c8b356
    claim: "Ran cargo test --locked --test migrate -- --exact a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place for TC-records-migrate-a-new-bug-rebuilds-the-index-in-place: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:a399f5c2e04d4a8c793d242c3372d0c1d696c8473a4946b0a96c5bb0702d77d4
    case: TC-records-migrate-a-new-bug-rebuilds-the-index-in-place
    case_digest: 99c3ddff20c586e5ad229f40b496f086bdeed59ca2988decaa488c3b21eddd83
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:59:43Z
    duration_seconds: 5
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-a-record-whose-id-is-held-d53d68
    claim: "Ran cargo test --locked --test migrate -- --exact a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused for TC-records-migrate-refuses-a-record-whose-id-is-held: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:9b40c331cfbbc19658dabd7e0756860e37977e7881116f6f5fef4c404c3b6995
    case: TC-records-migrate-refuses-a-record-whose-id-is-held
    case_digest: 6f451587dac58b87c23df4915220cba3f6c0ebf8f489832ca9128dbb1b79f435
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:59:50Z
    duration_seconds: 4
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-refuses-a-returning-legacy-checkpoint-68d47f
    claim: "Ran cargo test --locked --test migrate -- --exact a_returning_legacy_checkpoint_holding_another_session_is_refused for TC-records-migrate-refuses-a-returning-legacy-checkpoint: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_returning_legacy_checkpoint_holding_another_session_is_refused
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:21e4919ed47d1581e7c29850b334e1fe3f1c0da722151033ae0bf8b3baf27919
    case: TC-records-migrate-refuses-a-returning-legacy-checkpoint
    case_digest: 2718a4355329e350b2d940a2ce154c38a5a41d0ce93402442a815a48b58854d0
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:59:54Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-a-failed-write-removes-nothing-2eb471
    claim: "Ran cargo test --locked --test migrate -- --exact an_apply_whose_planned_write_fails_leaves_every_0_5_x_file for TC-records-migrate-a-failed-write-removes-nothing: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact an_apply_whose_planned_write_fails_leaves_every_0_5_x_file
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:dfc3cc50df5709593f7cab5fe4d5bbc2d585ead8f7afbaace14aa8d8016a7fd5
    case: TC-records-migrate-a-failed-write-removes-nothing
    case_digest: 72d5932eebb5ce0f17c5d5cb6cd34a44f1ba11bd5ecf5ad8beed21026ce8b7d3
    criteria: LIFE-20
    requirements: REQ-records-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:00:00Z
    duration_seconds: 4
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-doctor-keeps-a-migrated-alias-pointer-668349
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias for TC-cli-doctor-keeps-a-migrated-alias-pointer: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_fix_keeps_a_pointer_that_names_an_open_session_by_alias
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:afbc08af109308d5e353efaa7cc3fa76393c100d39509ce254d1932f9ba3e5eb
    case: TC-cli-doctor-keeps-a-migrated-alias-pointer
    case_digest: aaa3e31d7e306eb17a846963bba55d7497375e523063f99d8f280035a3dd741d
    criteria: DRIVE-17
    requirements: REQ-cli-055
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:00:09Z
    duration_seconds: 5
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-records-migrate-skips-macos-metadata
    claim: "Ran cargo test --locked --test migrate -- --exact migrate_skips_macos_metadata_beside_0_5_records for TC-records-migrate-skips-macos-metadata: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact migrate_skips_macos_metadata_beside_0_5_records
    revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
    environment: local-macos-arm64
    content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
    fingerprint: sha256:ca40d21a5b678947fcf491679493cf614be8389ae23eaafa74916cd8ff28209c
    case: TC-records-migrate-skips-macos-metadata
    case_digest: da5102d13f5a8ae9701fd4461c1d13f985754f77525b3bdb8205abccbf621e5d
    criteria: DRIVE-14
    requirements: REQ-records-015
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:00:16Z
    duration_seconds: 4
    actor: fable (claude-opus-5-5, hc t4b)
```
