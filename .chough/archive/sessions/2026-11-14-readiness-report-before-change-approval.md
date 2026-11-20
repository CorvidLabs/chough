---
kind: session
id: 2026-11-14-readiness-report-before-change-approval
aliases: [readiness-report-before-change-approval]
objective: One read-only command lists everything still missing before a change is approved, and names the command that fixes each piece (DRIVE-29).
intent_scope: DRIVE
modules: [cli]
stage: done
stage_status: archived
iteration: 1
native_change: readiness-report-before-change-approval
pr: 311
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
observations:
  - 2026-11-18-tc-cli-change-ready-draft-fixes
  - 2026-11-18-tc-cli-change-ready-stale-delta
  - 2026-11-18-tc-cli-change-ready-carried-delta
  - 2026-11-18-tc-cli-change-ready-undocumented-export
  - 2026-11-18-tc-cli-change-ready-cross-module-case
  - 2026-11-18-tc-cli-change-ready-outside-path
  - 2026-11-18-tc-cli-change-ready-no-spec-change
  - 2026-11-18-tc-cli-change-ready-unknown-change
  - 2026-11-18-tc-cli-change-ready-missing-specsync
  - 2026-11-18-tc-cli-change-ready-fast-path
  - 2026-11-19-tc-cli-change-ready-carried-delta
  - 2026-11-19-tc-cli-change-ready-cross-module-case
  - 2026-11-19-tc-cli-change-ready-draft-fixes
  - 2026-11-19-tc-cli-change-ready-fast-path
  - 2026-11-19-tc-cli-change-ready-missing-specsync
  - 2026-11-19-tc-cli-change-ready-no-spec-change
  - 2026-11-19-tc-cli-change-ready-outside-path
  - 2026-11-19-tc-cli-change-ready-stale-delta
  - 2026-11-19-tc-cli-change-ready-undocumented-export
  - 2026-11-19-tc-cli-change-ready-unknown-change
  - 2026-11-19-tc-cli-change-ready-carried-delta-c74f5b
  - 2026-11-19-tc-cli-change-ready-cross-module-case-b06d7d
  - 2026-11-19-tc-cli-change-ready-draft-fixes-d3f705
  - 2026-11-19-tc-cli-change-ready-fast-path-dd926a
  - 2026-11-19-tc-cli-change-ready-missing-specsync-fcea02
  - 2026-11-19-tc-cli-change-ready-no-spec-change-473e45
  - 2026-11-19-tc-cli-change-ready-outside-path-36e6bd
  - 2026-11-19-tc-cli-change-ready-stale-delta-dde404
  - 2026-11-19-tc-cli-change-ready-undocumented-export-b7af39
  - 2026-11-19-tc-cli-change-ready-unknown-change-542c9b
  - 2026-11-19-tc-cli-change-ready-carried-delta-5b3cd3
  - 2026-11-19-tc-cli-change-ready-cross-module-case-8b4128
  - 2026-11-19-tc-cli-change-ready-draft-fixes-b7fa59
  - 2026-11-19-tc-cli-change-ready-fast-path-f3cec5
  - 2026-11-19-tc-cli-change-ready-missing-specsync-ec0126
  - 2026-11-19-tc-cli-change-ready-no-spec-change-e0665b
  - 2026-11-19-tc-cli-change-ready-outside-path-56bda7
  - 2026-11-19-tc-cli-change-ready-stale-delta-7b9dee
  - 2026-11-19-tc-cli-change-ready-undocumented-export-97768a
  - 2026-11-19-tc-cli-change-ready-unknown-change-37efe5
  - 2026-11-19-tc-cli-change-ready-carried-delta-a60817
  - 2026-11-19-tc-cli-change-ready-cross-module-case-d7283c
  - 2026-11-19-tc-cli-change-ready-draft-fixes-6263f6
  - 2026-11-19-tc-cli-change-ready-fast-path-72c3c8
  - 2026-11-19-tc-cli-change-ready-missing-specsync-e40b48
  - 2026-11-19-tc-cli-change-ready-no-spec-change-eb2f12
  - 2026-11-19-tc-cli-change-ready-outside-path-ef4054
  - 2026-11-19-tc-cli-change-ready-stale-delta-a0fc54
  - 2026-11-19-tc-cli-change-ready-undocumented-export-93bb69
  - 2026-11-19-tc-cli-change-ready-unknown-change-4d4838
  - 2026-11-19-tc-cli-change-ready-carried-delta-83ce80
  - 2026-11-19-tc-cli-change-ready-cross-module-case-c11600
  - 2026-11-19-tc-cli-change-ready-draft-fixes-3ab90d
  - 2026-11-19-tc-cli-change-ready-fast-path-ccc4ee
  - 2026-11-19-tc-cli-change-ready-missing-specsync-ca3384
  - 2026-11-19-tc-cli-change-ready-no-spec-change-7ab82b
  - 2026-11-19-tc-cli-change-ready-outside-path-b9eed7
  - 2026-11-19-tc-cli-change-ready-stale-delta-ca2614
  - 2026-11-19-tc-cli-change-ready-undocumented-export-9895a9
  - 2026-11-19-tc-cli-change-ready-unknown-change-3789bf
  - 2026-11-19-tc-cli-change-ready-carried-delta-154864
  - 2026-11-19-tc-cli-change-ready-cross-module-case-3370d8
  - 2026-11-19-tc-cli-change-ready-draft-fixes-6dee3c
  - 2026-11-19-tc-cli-change-ready-fast-path-cee26f
  - 2026-11-19-tc-cli-change-ready-missing-specsync-c2e3ec
  - 2026-11-19-tc-cli-change-ready-no-spec-change-d0ee4c
  - 2026-11-19-tc-cli-change-ready-outside-path-801e59
  - 2026-11-19-tc-cli-change-ready-stale-delta-7008b9
  - 2026-11-19-tc-cli-change-ready-undocumented-export-8c7986
  - 2026-11-19-tc-cli-change-ready-unknown-change-bee9af
  - 2026-11-19-tc-cli-change-ready-carried-delta-96a108
  - 2026-11-19-tc-cli-change-ready-cross-module-case-f36b02
  - 2026-11-19-tc-cli-change-ready-draft-fixes-ec6b03
  - 2026-11-19-tc-cli-change-ready-fast-path-70c5f7
  - 2026-11-19-tc-cli-change-ready-missing-specsync-499063
  - 2026-11-19-tc-cli-change-ready-no-spec-change-415d6d
  - 2026-11-19-tc-cli-change-ready-outside-path-9a5439
  - 2026-11-19-tc-cli-change-ready-stale-delta-532d38
  - 2026-11-19-tc-cli-change-ready-undocumented-export-9d25ee
  - 2026-11-19-tc-cli-change-ready-unknown-change-d1b88c
  - 2026-11-19-tc-cli-change-ready-carried-delta-011f6d
  - 2026-11-19-tc-cli-change-ready-cross-module-case-6b1c3b
  - 2026-11-19-tc-cli-change-ready-draft-fixes-cb8167
  - 2026-11-19-tc-cli-change-ready-fast-path-3b7129
  - 2026-11-19-tc-cli-change-ready-missing-specsync-563f28
  - 2026-11-19-tc-cli-change-ready-no-spec-change-2e6a19
  - 2026-11-19-tc-cli-change-ready-outside-path-83c21c
  - 2026-11-19-tc-cli-change-ready-stale-delta-e7d7e7
  - 2026-11-19-tc-cli-change-ready-undocumented-export-b98e9d
  - 2026-11-19-tc-cli-change-ready-unknown-change-f3c04f
  - 2026-11-19-tc-cli-change-ready-carried-delta-4a3b65
  - 2026-11-19-tc-cli-change-ready-cross-module-case-8bc5bf
  - 2026-11-19-tc-cli-change-ready-draft-fixes-2ad9d0
  - 2026-11-19-tc-cli-change-ready-fast-path-3260d3
  - 2026-11-19-tc-cli-change-ready-missing-specsync-e73185
  - 2026-11-19-tc-cli-change-ready-no-spec-change-086754
  - 2026-11-19-tc-cli-change-ready-outside-path-23a0bc
  - 2026-11-19-tc-cli-change-ready-stale-delta-f19345
  - 2026-11-19-tc-cli-change-ready-undocumented-export-d15a7c
  - 2026-11-19-tc-cli-change-ready-unknown-change-3ac49e
title: "chough change ready: a read-only report before approval"
summary:
  - "Adds chough change ready <id> (REQ-cli-067, DRIVE-29): one read-only report of what is still missing before a change is approved, each finding naming the command that fixes it; it exits 1 when anything is missing and writes nothing."
  - "Verified: all eleven verify steps on macOS and Linux and its ten cases on macOS at 75b6bf6e; 890 tests in corvid-agent's run; tester2's harness against real SpecSync 6.3.0, identical on d3ab8590 and 75b6bf6e; the hash cache left byte-identical or absent, a replacement by another process left alone."
  - "Known gaps go to #406: SpecSync's dry run of the approval (spec-sync #1091) will replace this report's own copy of the owner rule; #307 is folded there."
closed: 2026-11-19
outcome: done
---
# One read-only command lists everything still missing before a change is approved, and names the command that fixes each piece (DRIVE-29).

## Goal

One read-only command lists everything still missing before a change is approved, and names the command that fixes each piece (DRIVE-29).

## Where it stands

- chough change ready lists everything still missing before approval and names the command that fixes each piece. fledge fmt, test, lint and the verify lane passed with Fledge 1.8.0, and ready on this change exits 0.

## Decisions and why

- Leif chose Capture as written for DRIVE-29 in chat on 2026-11-14. The sentence is the readiness command before approval.
- 0xLeif decided "Scope approval of readiness-report-before-change-approval": "Approve this scope (Recommended)" (Grok chat, 2026-11-14, choosing the option Approve this scope for this change's scope question. Relayed by grok (hc t1w).); recorded by grok-t1w; a relayed claim, not authenticated; ran specsync change approve readiness-report-before-change-approval as 0xLeif
- fable-t4b took this session over on Leif's choice 'You take it over' (Claude Code session, 2026-11-18): merged main, moved the changelog entry to changelog.d, renumbered the cli change log row to 101. The planned local-agent proof was not run; the command was used for real on fix-263 and fix-307 instead
- 0xLeif decided "Scope approval of readiness-report-before-change-approval with src/skill.rs added (native approval)": "Yes" (Relayed by Leif's planning session (hc t4c), 2026-11-18, for PR #311: send its change back once to add src/skill.rs for the two 0.6.1 digests, with his relayed re-approval, same scope otherwise); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve readiness-report-before-change-approval as 0xLeif
- 0xLeif decided "Scope approval of readiness-report-before-change-approval with the own-work wording (native approval)": "Approve" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53), 2026-11-18, for PR #311: the REQ-cli-067 wording 'Each file that differs from the change's base_commit and is the change's own work', its definition sentence and the new acceptance criterion, with one send-back; nothing else in the delta changes); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve readiness-report-before-change-approval as 0xLeif
- 0xLeif decided "Scope approval of readiness-report-before-change-approval with its changelog fragment added (native approval)": "Yes, add it" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53), 2026-11-18, for PR #311: one more send-back to add changelog.d/2026-11-14-readiness-report-before-change-approval.md to the declared paths, paths only with the delta untouched); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve readiness-report-before-change-approval as 0xLeif
- 0xLeif decided "Implementation review of readiness-report-before-change-approval": "Merge once its findings are fixed / yes corvid-agent counts, merge 311" (Two answers relayed by Leif's planning session (hc t4c): 'Merge once its findings are fixed' (2026-11-19, early UTC) and 'yes corvid-agent counts, merge 311' (Claude Code chat message relayed through fable-lead hc t53, about 15:04Z). Before the ship: tester2's rerun PASS, fable-lead's read by patch id, and corvid-agent's approval of 75b6bf6e at 15:22Z); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change review readiness-report-before-change-approval as 0xLeif with verdict pass

## What was verified

- 90 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1). - 7175794a4519; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-change-ready-carried-delta-4a3b65
- Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1). - 9e6a2f3adc01; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-change-ready-cross-module-case-8bc5bf
- Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1). - 275bc981f724; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-change-ready-draft-fixes-2ad9d0
- Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1). - 0841d35ca782; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-change-ready-fast-path-3260d3
- Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1). - f96f14ef5b90; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-change-ready-missing-specsync-e73185
- Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1). - 33b581f9bed7; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-change-ready-no-spec-change-086754
- Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1). - f03a3082c6fe; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-change-ready-outside-path-23a0bc
- Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1). - 79b09dad4527; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-change-ready-stale-delta-f19345
- Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1). - b6d0ae4fdf83; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-change-ready-undocumented-export-d15a7c
- Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1). - 06bc2c6af513; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-change-ready-unknown-change-3ac49e

## Bugs

None.

## Open threads

## Timeline

- 2026-11-14T20:55:49Z · start · intent → intent · One read-only command lists everything still missing before a change is approved, and names the command that fixes each piece (DRIVE-29). ^t-3affaa
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: unknown agent (hc t1w)
- 2026-11-14T20:55:49Z · advance · intent → design ·  ^t-4d51ca
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: unknown agent (hc t1w)
- 2026-11-14T20:55:49Z · skip · design → plan · A CLI report only: no visuals or interactions change. ^t-32b9ae
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: unknown agent (hc t1w)
- 2026-11-14T21:03:18Z · gate-opened · plan → plan · Scope approval of readiness-report-before-change-approval ^t-d0a53e
  - native_change: readiness-report-before-change-approval
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - holds: plan
  - drives: approve
  - by: unknown agent (hc t1w)
- 2026-11-14T21:05:51Z · gate-closed · plan → plan · Scope approval of readiness-report-before-change-approval ^t-871527
  - native_change: readiness-report-before-change-approval
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve this scope (Recommended)
    - where: Grok chat, 2026-11-14, choosing the option Approve this scope for this change's scope question. Relayed by grok (hc t1w).
    - recorded_by: grok-t1w
    - native: specsync change approve readiness-report-before-change-approval
    - actor: 0xLeif
  - by: unknown agent (hc t1w)
- 2026-11-14T21:06:00Z · advance · plan → build · native:approved ^t-bb35c7
  - native_change: readiness-report-before-change-approval
  - gate: native:approved
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: unknown agent (hc t1w)
- 2026-11-18T14:59:55Z · advance · build → test · native:verifying ^t-25f1a1
  - native_change: readiness-report-before-change-approval
  - gate: native:verifying
  - revision: f659d73d58d8bd642a2e99de2741c0ba05d109a4
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:59:55Z · advance · test → accept · native:verifying ^t-b35093
  - native_change: readiness-report-before-change-approval
  - gate: native:verifying
  - revision: f659d73d58d8bd642a2e99de2741c0ba05d109a4
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T15:00:11Z · gate-opened · accept → accept · Implementation review of readiness-report-before-change-approval ^t-426955
  - native_change: readiness-report-before-change-approval
  - revision: f659d73d58d8bd642a2e99de2741c0ba05d109a4
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T02:08:34Z · gate-opened · accept → accept · Scope approval of readiness-report-before-change-approval with src/skill.rs added (native approval) ^t-d443d4
  - native_change: readiness-report-before-change-approval
  - revision: 0dd6a5fd773f3d7e9ac64da032a803a0afed9a37
  - holds: accept
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T02:08:35Z · direct-native-write · accept → accept · readiness-report-before-change-approval's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-bb1789
  - native_change: readiness-report-before-change-approval
  - revision: 0dd6a5fd773f3d7e9ac64da032a803a0afed9a37
  - direct_write:
    - change: readiness-report-before-change-approval
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T02:08:35Z · gate-closed · accept → accept · Scope approval of readiness-report-before-change-approval with src/skill.rs added (native approval) ^t-8adba3
  - native_change: readiness-report-before-change-approval
  - revision: 0dd6a5fd773f3d7e9ac64da032a803a0afed9a37
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes
    - where: Relayed by Leif's planning session (hc t4c), 2026-11-18, for PR #311: send its change back once to add src/skill.rs for the two 0.6.1 digests, with his relayed re-approval, same scope otherwise
    - recorded_by: fable-t4b
    - native: specsync change approve readiness-report-before-change-approval
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T03:57:21Z · gate-opened · accept → accept · Scope approval of readiness-report-before-change-approval with the own-work wording (native approval) ^t-b4329c
  - native_change: readiness-report-before-change-approval
  - revision: f27fabc978595e274d59c1f6af7ae03b4c95966f
  - holds: accept
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T03:57:44Z · direct-native-write · accept → accept · readiness-report-before-change-approval's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-7b4d12
  - native_change: readiness-report-before-change-approval
  - revision: c17066e4d5d1bbe2b64f2db286b3aca282e21798
  - direct_write:
    - change: readiness-report-before-change-approval
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T03:57:44Z · gate-closed · accept → accept · Scope approval of readiness-report-before-change-approval with the own-work wording (native approval) ^t-cff492
  - native_change: readiness-report-before-change-approval
  - revision: c17066e4d5d1bbe2b64f2db286b3aca282e21798
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53), 2026-11-18, for PR #311: the REQ-cli-067 wording 'Each file that differs from the change's base_commit and is the change's own work', its definition sentence and the new acceptance criterion, with one send-back; nothing else in the delta changes
    - recorded_by: fable-t4b
    - native: specsync change approve readiness-report-before-change-approval
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T04:18:07Z · gate-opened · accept → accept · Scope approval of readiness-report-before-change-approval with its changelog fragment added (native approval) ^t-2f6629
  - native_change: readiness-report-before-change-approval
  - revision: c018cddd399af9ccbed1e978e01e7177ab48e351
  - holds: accept
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T04:18:08Z · direct-native-write · accept → accept · readiness-report-before-change-approval's lifecycle records changed outside Chough: state.json changed, approvals.json changed, verification.json changed, verification-attempts.json changed; state verifying → draft ^t-7062b5
  - native_change: readiness-report-before-change-approval
  - revision: c018cddd399af9ccbed1e978e01e7177ab48e351
  - direct_write:
    - change: readiness-report-before-change-approval
    - files:
      - state.json: changed
      - approvals.json: changed
      - verification.json: changed
      - verification-attempts.json: changed
    - before_state: verifying
    - after_state: draft
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T04:18:08Z · gate-closed · accept → accept · Scope approval of readiness-report-before-change-approval with its changelog fragment added (native approval) ^t-bd0cff
  - native_change: readiness-report-before-change-approval
  - revision: c018cddd399af9ccbed1e978e01e7177ab48e351
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, add it
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53), 2026-11-18, for PR #311: one more send-back to add changelog.d/2026-11-14-readiness-report-before-change-approval.md to the declared paths, paths only with the delta untouched
    - recorded_by: fable-t4b
    - native: specsync change approve readiness-report-before-change-approval
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T15:39:05Z · gate-closed · accept → accept · Implementation review of readiness-report-before-change-approval ^t-7aade0
  - native_change: readiness-report-before-change-approval
  - revision: c824abe00244a15111d5daed1e62e38090d6f16c
  - relayed_by:
    - decided_by: 0xLeif
    - words: Merge once its findings are fixed / yes corvid-agent counts, merge 311
    - where: Two answers relayed by Leif's planning session (hc t4c): 'Merge once its findings are fixed' (2026-11-19, early UTC) and 'yes corvid-agent counts, merge 311' (Claude Code chat message relayed through fable-lead hc t53, about 15:04Z). Before the ship: tester2's rerun PASS, fable-lead's read by patch id, and corvid-agent's approval of 75b6bf6e at 15:22Z
    - recorded_by: fable-t4b
    - native: specsync change review readiness-report-before-change-approval
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T15:39:07Z · advance · accept → done · native:archived ^t-a8b5fc
  - native_change: readiness-report-before-change-approval
  - gate: native:archived
  - revision: c824abe00244a15111d5daed1e62e38090d6f16c
  - by: fable (claude-opus-5-5, hc t4b)

Closed 2026-11-19: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-18-tc-cli-change-ready-draft-fixes
    claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
    revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
    environment: local-macos-arm64
    content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
    fingerprint: sha256:cfbd5c635c9b55be6dedad73ef07ab8c37ebe60b83b9941b0fc22f24899a0448
    case: TC-cli-change-ready-draft-fixes
    case_digest: cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-18T14:58:58Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-18-tc-cli-change-ready-stale-delta
    claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten
    revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
    environment: local-macos-arm64
    content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
    fingerprint: sha256:f27ef4f5e585a09da954468baac96f781bbf503790a3192f718e4e11415dd2e2
    case: TC-cli-change-ready-stale-delta
    case_digest: 3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-18T14:59:00Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-18-tc-cli-change-ready-carried-delta
    claim: "Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named
    revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
    environment: local-macos-arm64
    content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
    fingerprint: sha256:ea61034797f19a2e19988f482711cc447afdfca5ba96f664c002c2c914196670
    case: TC-cli-change-ready-carried-delta
    case_digest: 06fd1cbe67c7f4336b5dbca85000d41baa9312cade6bbb57a4d42a22724f8493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-18T14:59:02Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-18-tc-cli-change-ready-undocumented-export
    claim: "Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named
    revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
    environment: local-macos-arm64
    content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
    fingerprint: sha256:11c87c7c87b9df1cef57c962125b82968ab0b358f744fa96d383932ca22068de
    case: TC-cli-change-ready-undocumented-export
    case_digest: 0cb0b17d7d5e65710413d8d3c217bfb814383aaf357ecae7b605b5a430e486fa
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-18T14:59:04Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-18-tc-cli-change-ready-cross-module-case
    claim: "Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named
    revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
    environment: local-macos-arm64
    content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
    fingerprint: sha256:a35a1fc4e1582e146158c4fae21f76b12390c45bc2b0a18d39740dfaedbf1359
    case: TC-cli-change-ready-cross-module-case
    case_digest: 9f35cd45d6d08282e4d49df0bf8f98767346d5c83ae6e91e0f2c5be79e097b75
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-18T14:59:06Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-18-tc-cli-change-ready-outside-path
    claim: "Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named
    revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
    environment: local-macos-arm64
    content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
    fingerprint: sha256:cf0aee7a879d09776baeb67e618113ac01602a27d2c576dc1df98ee279433a63
    case: TC-cli-change-ready-outside-path
    case_digest: 08fb3fc5bc3f9e56789e531095851ca120f520c96e45325f5fa7d31eaf028f6f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-18T14:59:07Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-18-tc-cli-change-ready-no-spec-change
    claim: "Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal
    revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
    environment: local-macos-arm64
    content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
    fingerprint: sha256:4ee5a249c20f1ac01c48863ea5be1bd34ca08951629e8b61979188322b9bc8cd
    case: TC-cli-change-ready-no-spec-change
    case_digest: 9fa20b1437572a7de57f9ee3c22d6ec8e9012213e9a5e9f8d7df5337dbcae50c
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-18T14:59:09Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-18-tc-cli-change-ready-unknown-change
    claim: "Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_unknown_change_names_status
    revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
    environment: local-macos-arm64
    content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
    fingerprint: sha256:f74e926196e049e33512edb1ee9ed0066859359e1deddbb26d2daee6ba0b5156
    case: TC-cli-change-ready-unknown-change
    case_digest: d583ee473a6bd96ea48e47ba8b11833dda832f5234e2f63fbbbcafbf10ff9493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-18T14:59:11Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-18-tc-cli-change-ready-missing-specsync
    claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command
    revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
    environment: local-macos-arm64
    content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
    fingerprint: sha256:e129e03d8e0f59d6933a2d7c38f1a1dca04e82085e958dfc662af2b8e3d082b1
    case: TC-cli-change-ready-missing-specsync
    case_digest: 83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-18T14:59:12Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-18-tc-cli-change-ready-fast-path
    claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
    revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
    environment: local-macos-arm64
    content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
    fingerprint: sha256:83148c6f5fa455ec9f39a3ddef7d790c8334225122ec032963806c70afb33222
    case: TC-cli-change-ready-fast-path
    case_digest: 82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-18T14:59:14Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-carried-delta
    claim: "Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named
    revision: 24323c5c04434521dfff65da788125d02010f545
    environment: local-macos-arm64
    content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
    fingerprint: sha256:358e67e49cce89f2c08dbe66af5e7eb62fc3116f20a19a63bcd53145ecc8699c
    case: TC-cli-change-ready-carried-delta
    case_digest: 06fd1cbe67c7f4336b5dbca85000d41baa9312cade6bbb57a4d42a22724f8493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:16:42Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-cross-module-case
    claim: "Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named
    revision: 24323c5c04434521dfff65da788125d02010f545
    environment: local-macos-arm64
    content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
    fingerprint: sha256:979d5fe346af584bd9fd1257c158c35070d6ff5be6f8e167fecce62ea80361e7
    case: TC-cli-change-ready-cross-module-case
    case_digest: 9f35cd45d6d08282e4d49df0bf8f98767346d5c83ae6e91e0f2c5be79e097b75
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:16:47Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-draft-fixes
    claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
    revision: 24323c5c04434521dfff65da788125d02010f545
    environment: local-macos-arm64
    content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
    fingerprint: sha256:6d0e963ac291e99c37d00d112be21d65e66ae4c9f0fef4e6a345de54f2ac75ce
    case: TC-cli-change-ready-draft-fixes
    case_digest: cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:16:51Z
    duration_seconds: 2
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-fast-path
    claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
    revision: 24323c5c04434521dfff65da788125d02010f545
    environment: local-macos-arm64
    content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
    fingerprint: sha256:167e3699e7409073fb6bef49349d5f21ee73f4df877ce19c0d5046423d96ff65
    case: TC-cli-change-ready-fast-path
    case_digest: 82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:16:53Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-missing-specsync
    claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command
    revision: 24323c5c04434521dfff65da788125d02010f545
    environment: local-macos-arm64
    content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
    fingerprint: sha256:767b8df73166a282a2d2b131f66bfea649acd406529952b307822046d3d9bd43
    case: TC-cli-change-ready-missing-specsync
    case_digest: 83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:16:57Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-no-spec-change
    claim: "Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal
    revision: 24323c5c04434521dfff65da788125d02010f545
    environment: local-macos-arm64
    content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
    fingerprint: sha256:fe334a0200666b460cccb2629a4025314eccbe41f9bba7b799a8165410e7e138
    case: TC-cli-change-ready-no-spec-change
    case_digest: 9fa20b1437572a7de57f9ee3c22d6ec8e9012213e9a5e9f8d7df5337dbcae50c
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:17:00Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-outside-path
    claim: "Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named
    revision: 24323c5c04434521dfff65da788125d02010f545
    environment: local-macos-arm64
    content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
    fingerprint: sha256:0dd2f536313e34d88f8e418b032060d1150336cffdb869f83060dea1d7bc7fcc
    case: TC-cli-change-ready-outside-path
    case_digest: 08fb3fc5bc3f9e56789e531095851ca120f520c96e45325f5fa7d31eaf028f6f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:17:04Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-stale-delta
    claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten
    revision: 24323c5c04434521dfff65da788125d02010f545
    environment: local-macos-arm64
    content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
    fingerprint: sha256:10d783679d42f3f90ddedd25b2835e68f96a2f9e1633a531b51fcd1ec0740e8a
    case: TC-cli-change-ready-stale-delta
    case_digest: 3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:17:10Z
    duration_seconds: 3
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-undocumented-export
    claim: "Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named
    revision: 24323c5c04434521dfff65da788125d02010f545
    environment: local-macos-arm64
    content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
    fingerprint: sha256:88e1ed61fa31029eb86f56cda93f2ec09eed3136fadc81933d196c32aed48957
    case: TC-cli-change-ready-undocumented-export
    case_digest: 0cb0b17d7d5e65710413d8d3c217bfb814383aaf357ecae7b605b5a430e486fa
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:17:22Z
    duration_seconds: 8
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-unknown-change
    claim: "Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_unknown_change_names_status
    revision: 24323c5c04434521dfff65da788125d02010f545
    environment: local-macos-arm64
    content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
    fingerprint: sha256:72bcc9475bcdbd25339bc1a158083c2207b9275da0e5c03a02ae45be32499fbc
    case: TC-cli-change-ready-unknown-change
    case_digest: d583ee473a6bd96ea48e47ba8b11833dda832f5234e2f63fbbbcafbf10ff9493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:17:25Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-carried-delta-c74f5b
    claim: "Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named
    revision: 240d7dafabeea57a669447149f515f63f6d3e15a
    environment: local-macos-arm64
    content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
    fingerprint: sha256:82de58d3fc0a18cf46adecb55112d58418bde3862052fc10de3b3b1e0e8fd1ff
    case: TC-cli-change-ready-carried-delta
    case_digest: 06fd1cbe67c7f4336b5dbca85000d41baa9312cade6bbb57a4d42a22724f8493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:13:22Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-cross-module-case-b06d7d
    claim: "Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named
    revision: 240d7dafabeea57a669447149f515f63f6d3e15a
    environment: local-macos-arm64
    content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
    fingerprint: sha256:29ec2589b6df021f6ded849b199106bbb7e89bcb14543b249c4137ebba52eb12
    case: TC-cli-change-ready-cross-module-case
    case_digest: 9f35cd45d6d08282e4d49df0bf8f98767346d5c83ae6e91e0f2c5be79e097b75
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:13:26Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-draft-fixes-d3f705
    claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
    revision: 240d7dafabeea57a669447149f515f63f6d3e15a
    environment: local-macos-arm64
    content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
    fingerprint: sha256:4e208348cdc8571c38d3533c2b1eb07a92cf9aa737faf84e606d17ca622c2471
    case: TC-cli-change-ready-draft-fixes
    case_digest: cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:13:31Z
    duration_seconds: 2
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-fast-path-dd926a
    claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
    revision: 240d7dafabeea57a669447149f515f63f6d3e15a
    environment: local-macos-arm64
    content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
    fingerprint: sha256:fdb3cc458a0f3e3cabca72ddffb6a67a22a69e1d41c97320a60982494f8678c8
    case: TC-cli-change-ready-fast-path
    case_digest: 82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:13:34Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-missing-specsync-fcea02
    claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command
    revision: 240d7dafabeea57a669447149f515f63f6d3e15a
    environment: local-macos-arm64
    content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
    fingerprint: sha256:1c5d292b83f35916b11290f7362c16980155f877b3301653c3e56f950bd3339f
    case: TC-cli-change-ready-missing-specsync
    case_digest: 83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:13:38Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-no-spec-change-473e45
    claim: "Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal
    revision: 240d7dafabeea57a669447149f515f63f6d3e15a
    environment: local-macos-arm64
    content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
    fingerprint: sha256:2a05c0dbe98cda9df7562ce5c080d23d7d8c85aa9eaf833efcd44f1bc2fd5fe1
    case: TC-cli-change-ready-no-spec-change
    case_digest: 9fa20b1437572a7de57f9ee3c22d6ec8e9012213e9a5e9f8d7df5337dbcae50c
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:13:42Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-outside-path-36e6bd
    claim: "Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named
    revision: 240d7dafabeea57a669447149f515f63f6d3e15a
    environment: local-macos-arm64
    content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
    fingerprint: sha256:53538f3bfb20ebbb0bf025d749b0259460c1d0b7b8bf66293a1a9ba04b3ce103
    case: TC-cli-change-ready-outside-path
    case_digest: 08fb3fc5bc3f9e56789e531095851ca120f520c96e45325f5fa7d31eaf028f6f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:13:46Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-stale-delta-dde404
    claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten
    revision: 240d7dafabeea57a669447149f515f63f6d3e15a
    environment: local-macos-arm64
    content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
    fingerprint: sha256:ce66109725d42bc8cbe7d6018c00d0aecb9e157518f09be2f90a90f54ce1a239
    case: TC-cli-change-ready-stale-delta
    case_digest: 3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:13:50Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-undocumented-export-b7af39
    claim: "Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named
    revision: 240d7dafabeea57a669447149f515f63f6d3e15a
    environment: local-macos-arm64
    content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
    fingerprint: sha256:420761d6b3ef12908aa18faf0bfb23531827a1cc4eaeca33c532fc123845cccc
    case: TC-cli-change-ready-undocumented-export
    case_digest: 0cb0b17d7d5e65710413d8d3c217bfb814383aaf357ecae7b605b5a430e486fa
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:13:54Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-unknown-change-542c9b
    claim: "Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_unknown_change_names_status
    revision: 240d7dafabeea57a669447149f515f63f6d3e15a
    environment: local-macos-arm64
    content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
    fingerprint: sha256:93b112f9b316288dbda965d94fccf9cb1b8d7482f5819fee39a2086c6d46ef5c
    case: TC-cli-change-ready-unknown-change
    case_digest: d583ee473a6bd96ea48e47ba8b11833dda832f5234e2f63fbbbcafbf10ff9493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:13:56Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-carried-delta-5b3cd3
    claim: "Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named
    revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
    environment: local-macos-arm64
    content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
    fingerprint: sha256:8d2294fb9450a9efb2a032c65977b328fba84cb7a126decb676f0fbaf0b36688
    case: TC-cli-change-ready-carried-delta
    case_digest: 06fd1cbe67c7f4336b5dbca85000d41baa9312cade6bbb57a4d42a22724f8493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:49:20Z
    duration_seconds: 3
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-cross-module-case-8b4128
    claim: "Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named
    revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
    environment: local-macos-arm64
    content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
    fingerprint: sha256:a329bb3fcbf64ba56dc9cbcddc6b3914c3282e77018cd31e75a73538bf016358
    case: TC-cli-change-ready-cross-module-case
    case_digest: 9f35cd45d6d08282e4d49df0bf8f98767346d5c83ae6e91e0f2c5be79e097b75
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:49:26Z
    duration_seconds: 3
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-draft-fixes-b7fa59
    claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
    revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
    environment: local-macos-arm64
    content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
    fingerprint: sha256:fd34beb67ace23ba1d1ee27dafe6b668ee9de198e51fecd4780fe29c123b1f5d
    case: TC-cli-change-ready-draft-fixes
    case_digest: cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:49:33Z
    duration_seconds: 4
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-fast-path-f3cec5
    claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
    revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
    environment: local-macos-arm64
    content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
    fingerprint: sha256:325a356fb51aca5a23a4e7deb133e54f3290f08024af8d74e356df9326885915
    case: TC-cli-change-ready-fast-path
    case_digest: 82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:49:35Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-missing-specsync-ec0126
    claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command
    revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
    environment: local-macos-arm64
    content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
    fingerprint: sha256:b5e7a0bbeb4cc683260bb30ca2b56190b25e25e21082645384444daca8a7f18a
    case: TC-cli-change-ready-missing-specsync
    case_digest: 83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:49:40Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-no-spec-change-e0665b
    claim: "Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal
    revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
    environment: local-macos-arm64
    content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
    fingerprint: sha256:36c7e7ec2ee70b4fef56b2ee7e9b2618e477fffb5b542ab050ebd767f7316364
    case: TC-cli-change-ready-no-spec-change
    case_digest: 9fa20b1437572a7de57f9ee3c22d6ec8e9012213e9a5e9f8d7df5337dbcae50c
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:49:45Z
    duration_seconds: 2
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-outside-path-56bda7
    claim: "Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named
    revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
    environment: local-macos-arm64
    content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
    fingerprint: sha256:2483f496b5f4bd08061509c0042a8698d5f58730fb962bda701ab27c794c1ef0
    case: TC-cli-change-ready-outside-path
    case_digest: 08fb3fc5bc3f9e56789e531095851ca120f520c96e45325f5fa7d31eaf028f6f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:49:53Z
    duration_seconds: 4
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-stale-delta-7b9dee
    claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten
    revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
    environment: local-macos-arm64
    content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
    fingerprint: sha256:6b3dbe60f2da9a66d02b92fcd1ff8b599d750c9ecbc3d0544c907f069dd9da22
    case: TC-cli-change-ready-stale-delta
    case_digest: 3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:50:00Z
    duration_seconds: 4
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-undocumented-export-97768a
    claim: "Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named
    revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
    environment: local-macos-arm64
    content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
    fingerprint: sha256:333636b0c2c7caa6a602a85e35c7f049d3c87892baa483412d07cf18a69ac9eb
    case: TC-cli-change-ready-undocumented-export
    case_digest: 0cb0b17d7d5e65710413d8d3c217bfb814383aaf357ecae7b605b5a430e486fa
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:50:05Z
    duration_seconds: 2
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-unknown-change-37efe5
    claim: "Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_unknown_change_names_status
    revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
    environment: local-macos-arm64
    content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
    fingerprint: sha256:2acd1254abce3b5048936396d03c3f5bb39f8f43bf2bf3e19ed975a535871f71
    case: TC-cli-change-ready-unknown-change
    case_digest: d583ee473a6bd96ea48e47ba8b11833dda832f5234e2f63fbbbcafbf10ff9493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:50:09Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-carried-delta-a60817
    claim: "Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named
    revision: e42c96fec77c56feff65256be9e1d5deb1f34742
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:84903709d578657e93e021c448b2ba4d39069c085d10b02d262d82aa2f96cfb1
    case: TC-cli-change-ready-carried-delta
    case_digest: 06fd1cbe67c7f4336b5dbca85000d41baa9312cade6bbb57a4d42a22724f8493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:12:22Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-cross-module-case-d7283c
    claim: "Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named
    revision: e42c96fec77c56feff65256be9e1d5deb1f34742
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:00094fcddd37bc4eaf8e02326ff05ac21e1e8d8aa73a899e79f0a940622e1270
    case: TC-cli-change-ready-cross-module-case
    case_digest: 9f35cd45d6d08282e4d49df0bf8f98767346d5c83ae6e91e0f2c5be79e097b75
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:12:25Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-draft-fixes-6263f6
    claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
    revision: e42c96fec77c56feff65256be9e1d5deb1f34742
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:aaea7c91cabefdd2ca4ffa9e756a8cd97e1262e3100bea838b1cfe34c758be75
    case: TC-cli-change-ready-draft-fixes
    case_digest: cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:12:28Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-fast-path-72c3c8
    claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
    revision: e42c96fec77c56feff65256be9e1d5deb1f34742
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:325a356fb51aca5a23a4e7deb133e54f3290f08024af8d74e356df9326885915
    case: TC-cli-change-ready-fast-path
    case_digest: 82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:12:30Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-missing-specsync-e40b48
    claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command
    revision: e42c96fec77c56feff65256be9e1d5deb1f34742
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:de460a381b328f484a2895ac89b771048fed43b3a85dd430a37ebbe53bd4a35c
    case: TC-cli-change-ready-missing-specsync
    case_digest: 83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:12:33Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-no-spec-change-eb2f12
    claim: "Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal
    revision: e42c96fec77c56feff65256be9e1d5deb1f34742
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:6188977154e3be0391d9dc0ea2b877bd3c934194d7dfbb2235ed4b8145de2f5b
    case: TC-cli-change-ready-no-spec-change
    case_digest: 9fa20b1437572a7de57f9ee3c22d6ec8e9012213e9a5e9f8d7df5337dbcae50c
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:12:35Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-outside-path-ef4054
    claim: "Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named
    revision: e42c96fec77c56feff65256be9e1d5deb1f34742
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:6492c4d9a3bfbcb8b0a80b7ac3cab82908b9906bc3ebdfca44fe68985b17b29a
    case: TC-cli-change-ready-outside-path
    case_digest: 08fb3fc5bc3f9e56789e531095851ca120f520c96e45325f5fa7d31eaf028f6f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:12:39Z
    duration_seconds: 2
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-stale-delta-a0fc54
    claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten
    revision: e42c96fec77c56feff65256be9e1d5deb1f34742
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:d47f36b11f021ce26bb956aa67cfb18207314edfa3720c90b1103ec86f420de8
    case: TC-cli-change-ready-stale-delta
    case_digest: 3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:12:42Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-undocumented-export-93bb69
    claim: "Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named
    revision: e42c96fec77c56feff65256be9e1d5deb1f34742
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:951fdbc2db87b7c28490e45b72f639ad700a91c4b2e02b1fa938a04ec1a35c0e
    case: TC-cli-change-ready-undocumented-export
    case_digest: 0cb0b17d7d5e65710413d8d3c217bfb814383aaf357ecae7b605b5a430e486fa
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:12:45Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-unknown-change-4d4838
    claim: "Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_unknown_change_names_status
    revision: e42c96fec77c56feff65256be9e1d5deb1f34742
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:bb95bf0bd07f2819ce7d2692b490901a1d9dc90e57d6817a53d86f472eca044f
    case: TC-cli-change-ready-unknown-change
    case_digest: d583ee473a6bd96ea48e47ba8b11833dda832f5234e2f63fbbbcafbf10ff9493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:12:47Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-carried-delta-83ce80
    claim: "Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named
    revision: 745c1109cdd230e410c04e2698da940ae00d2c3e
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:b106679ed522650a49c5a824ed73e67ea264f6f3a4ca889d5459c1a209d3ca8c
    case: TC-cli-change-ready-carried-delta
    case_digest: 06fd1cbe67c7f4336b5dbca85000d41baa9312cade6bbb57a4d42a22724f8493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:22:24Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-cross-module-case-c11600
    claim: "Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named
    revision: 745c1109cdd230e410c04e2698da940ae00d2c3e
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:a2d0980bb56dbcb09e7334c9a38fc228d5aea4c0fe153e9f67026e14c60a6d99
    case: TC-cli-change-ready-cross-module-case
    case_digest: 9f35cd45d6d08282e4d49df0bf8f98767346d5c83ae6e91e0f2c5be79e097b75
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:22:26Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-draft-fixes-3ab90d
    claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
    revision: 745c1109cdd230e410c04e2698da940ae00d2c3e
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:8817f4460efff1d5ccb841ab97f63c08748304acb35620a1eb1d26380d026d17
    case: TC-cli-change-ready-draft-fixes
    case_digest: cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:22:28Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-fast-path-ccc4ee
    claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
    revision: 745c1109cdd230e410c04e2698da940ae00d2c3e
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:ebe9dbb40e65f0a4b31cb4d656e31998ca2cd55b26486734aa5a4f7e2fa883a9
    case: TC-cli-change-ready-fast-path
    case_digest: 82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:22:30Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-missing-specsync-ca3384
    claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command
    revision: 745c1109cdd230e410c04e2698da940ae00d2c3e
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:d977187f9ed882414ba9b54c6030916f4f52447a046a692c662b7218b717f9f7
    case: TC-cli-change-ready-missing-specsync
    case_digest: 83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:22:32Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-no-spec-change-7ab82b
    claim: "Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal
    revision: 745c1109cdd230e410c04e2698da940ae00d2c3e
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:1d798565fb85ddbaccb1117b7dd9311073ec44a76da9f7ded2dc1737bc55b4d3
    case: TC-cli-change-ready-no-spec-change
    case_digest: 9fa20b1437572a7de57f9ee3c22d6ec8e9012213e9a5e9f8d7df5337dbcae50c
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:22:34Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-outside-path-b9eed7
    claim: "Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named
    revision: 745c1109cdd230e410c04e2698da940ae00d2c3e
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:c97008319e036d812cc9725f4758d14b00f50aed1aeded9376a1ce9a3080f5a3
    case: TC-cli-change-ready-outside-path
    case_digest: 08fb3fc5bc3f9e56789e531095851ca120f520c96e45325f5fa7d31eaf028f6f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:22:37Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-stale-delta-ca2614
    claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten
    revision: 745c1109cdd230e410c04e2698da940ae00d2c3e
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:f717e8ed22312cfbff23a985f3e5b7975ce5999415b8020d6f2410d5bbf57e52
    case: TC-cli-change-ready-stale-delta
    case_digest: 3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:22:39Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-undocumented-export-9895a9
    claim: "Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named
    revision: 745c1109cdd230e410c04e2698da940ae00d2c3e
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:9e7dadb47b8a6fd675d3f1c7d628776411f378c3e809fe8bd0ee241db4affd82
    case: TC-cli-change-ready-undocumented-export
    case_digest: 0cb0b17d7d5e65710413d8d3c217bfb814383aaf357ecae7b605b5a430e486fa
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:22:41Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-unknown-change-3789bf
    claim: "Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_unknown_change_names_status
    revision: 745c1109cdd230e410c04e2698da940ae00d2c3e
    environment: local-macos-arm64
    content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
    fingerprint: sha256:96297ea775874108aad831bc71ead6e6da0b7fa498616593b3d79aac472206aa
    case: TC-cli-change-ready-unknown-change
    case_digest: d583ee473a6bd96ea48e47ba8b11833dda832f5234e2f63fbbbcafbf10ff9493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:22:43Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-carried-delta-154864
    claim: "Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named
    revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
    environment: local-macos-arm64
    content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
    fingerprint: sha256:dab6d55ab9f795f0a01ab0a9b1a15e141751d1d97efbd921f948f7d0a2cc9920
    case: TC-cli-change-ready-carried-delta
    case_digest: 06fd1cbe67c7f4336b5dbca85000d41baa9312cade6bbb57a4d42a22724f8493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:38:05Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-cross-module-case-3370d8
    claim: "Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named
    revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
    environment: local-macos-arm64
    content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
    fingerprint: sha256:5255c10248a20c95afbd08c864352d04f80ae9cb09d9de095c2dafb123859c82
    case: TC-cli-change-ready-cross-module-case
    case_digest: 9f35cd45d6d08282e4d49df0bf8f98767346d5c83ae6e91e0f2c5be79e097b75
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:38:08Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-draft-fixes-6dee3c
    claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
    revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
    environment: local-macos-arm64
    content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
    fingerprint: sha256:2b61a21b91f2daa0a277fc24f5f199f1fd69edba11d1a848603cdc5592953c56
    case: TC-cli-change-ready-draft-fixes
    case_digest: cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:38:12Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-fast-path-cee26f
    claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
    revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
    environment: local-macos-arm64
    content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
    fingerprint: sha256:325a356fb51aca5a23a4e7deb133e54f3290f08024af8d74e356df9326885915
    case: TC-cli-change-ready-fast-path
    case_digest: 82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:38:15Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-missing-specsync-c2e3ec
    claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command
    revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
    environment: local-macos-arm64
    content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
    fingerprint: sha256:0a374f5c406ea3fb84c1f9ff41bab466009bc666a289dcd6525dea1d2494e7e2
    case: TC-cli-change-ready-missing-specsync
    case_digest: 83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:38:18Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-no-spec-change-d0ee4c
    claim: "Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal
    revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
    environment: local-macos-arm64
    content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
    fingerprint: sha256:2f06734e3bfc4ac555445d0d0367d179fc7b1cbb41af63fa4569b1991008bbd4
    case: TC-cli-change-ready-no-spec-change
    case_digest: 9fa20b1437572a7de57f9ee3c22d6ec8e9012213e9a5e9f8d7df5337dbcae50c
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:38:22Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-outside-path-801e59
    claim: "Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named
    revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
    environment: local-macos-arm64
    content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
    fingerprint: sha256:0ee95669148ce9c534a573da6d46c4322d54dd3073b008779c77ff7e74fbba0a
    case: TC-cli-change-ready-outside-path
    case_digest: 08fb3fc5bc3f9e56789e531095851ca120f520c96e45325f5fa7d31eaf028f6f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:38:27Z
    duration_seconds: 2
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-stale-delta-7008b9
    claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten
    revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
    environment: local-macos-arm64
    content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
    fingerprint: sha256:95eb432182ae247a95504579cd3f53e5370f51703757f8b62619a41f581f54c2
    case: TC-cli-change-ready-stale-delta
    case_digest: 3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:38:31Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-undocumented-export-8c7986
    claim: "Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named
    revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
    environment: local-macos-arm64
    content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
    fingerprint: sha256:23ce637d4ab82acaa7aac8bf8c0dcb4e1d06a4b6190e909d6b665963e4800a40
    case: TC-cli-change-ready-undocumented-export
    case_digest: 0cb0b17d7d5e65710413d8d3c217bfb814383aaf357ecae7b605b5a430e486fa
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:38:35Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-unknown-change-bee9af
    claim: "Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_unknown_change_names_status
    revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
    environment: local-macos-arm64
    content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
    fingerprint: sha256:98278124cfe2abf24914547eeffb7779bb89f25d3ae3aa2a87d9354978fde830
    case: TC-cli-change-ready-unknown-change
    case_digest: d583ee473a6bd96ea48e47ba8b11833dda832f5234e2f63fbbbcafbf10ff9493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:38:38Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-carried-delta-96a108
    claim: "Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named
    revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
    environment: local-macos-arm64
    content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
    fingerprint: sha256:e602a9afc04bb08ead7a63df01edf5d2319f1d4d00fd8c2f026bb23c1942f081
    case: TC-cli-change-ready-carried-delta
    case_digest: 06fd1cbe67c7f4336b5dbca85000d41baa9312cade6bbb57a4d42a22724f8493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:31Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-cross-module-case-f36b02
    claim: "Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named
    revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
    environment: local-macos-arm64
    content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
    fingerprint: sha256:55c7123591b1fd9ade82ae3c4d55d556f4f8197843fc4cdbb4fbb50b20e08bb2
    case: TC-cli-change-ready-cross-module-case
    case_digest: 9f35cd45d6d08282e4d49df0bf8f98767346d5c83ae6e91e0f2c5be79e097b75
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:35Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-draft-fixes-ec6b03
    claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
    revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
    environment: local-macos-arm64
    content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
    fingerprint: sha256:01eec7441b41f287a6662cd6cf6716abd6e74c80605d60d57c23c11e8c0b2dc6
    case: TC-cli-change-ready-draft-fixes
    case_digest: cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:39Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-fast-path-70c5f7
    claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
    revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
    environment: local-macos-arm64
    content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
    fingerprint: sha256:325a356fb51aca5a23a4e7deb133e54f3290f08024af8d74e356df9326885915
    case: TC-cli-change-ready-fast-path
    case_digest: 82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:42Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-missing-specsync-499063
    claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command
    revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
    environment: local-macos-arm64
    content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
    fingerprint: sha256:c21ebbfd5f508d3f499f6433b699ace50aa67cb159c9b44e8b3211ddfa8d73cf
    case: TC-cli-change-ready-missing-specsync
    case_digest: 83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:46Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-no-spec-change-415d6d
    claim: "Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal
    revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
    environment: local-macos-arm64
    content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
    fingerprint: sha256:182a18342fe6eedd5572d1ce3480f09dd00671a22380ea27a1acee04af39d926
    case: TC-cli-change-ready-no-spec-change
    case_digest: 9fa20b1437572a7de57f9ee3c22d6ec8e9012213e9a5e9f8d7df5337dbcae50c
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:50Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-outside-path-9a5439
    claim: "Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named
    revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
    environment: local-macos-arm64
    content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
    fingerprint: sha256:a8bf17ce4ab252bf0272f85c55b1198432e0a5fbf2a2afb12a6ce2b1bff470c0
    case: TC-cli-change-ready-outside-path
    case_digest: 08fb3fc5bc3f9e56789e531095851ca120f520c96e45325f5fa7d31eaf028f6f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:55Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-stale-delta-532d38
    claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten
    revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
    environment: local-macos-arm64
    content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
    fingerprint: sha256:4197e0a27f1a928e64170113b3677a1542c5c696aa90653c0ff6d972a8fa0163
    case: TC-cli-change-ready-stale-delta
    case_digest: 3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:59Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-undocumented-export-9d25ee
    claim: "Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named
    revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
    environment: local-macos-arm64
    content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
    fingerprint: sha256:84d7aa45efc8c40a64216dbdc902d308c502fbb7807a84151b74ebac6e8628d5
    case: TC-cli-change-ready-undocumented-export
    case_digest: 0cb0b17d7d5e65710413d8d3c217bfb814383aaf357ecae7b605b5a430e486fa
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:54:03Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-unknown-change-d1b88c
    claim: "Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_unknown_change_names_status
    revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
    environment: local-macos-arm64
    content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
    fingerprint: sha256:94545703cf04411759eb2f8af20b63553bedd0a17a8fc5cc4b8eaf142516d3f2
    case: TC-cli-change-ready-unknown-change
    case_digest: d583ee473a6bd96ea48e47ba8b11833dda832f5234e2f63fbbbcafbf10ff9493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:54:08Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-carried-delta-011f6d
    claim: "Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named
    revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
    environment: local-macos-arm64
    content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
    fingerprint: sha256:14a228e79f165791675d86af3f61dac94ed0e54dbb05f5432ec29bf80bbda1a0
    case: TC-cli-change-ready-carried-delta
    case_digest: 06fd1cbe67c7f4336b5dbca85000d41baa9312cade6bbb57a4d42a22724f8493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:10:37Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-cross-module-case-6b1c3b
    claim: "Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named
    revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
    environment: local-macos-arm64
    content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
    fingerprint: sha256:4d77dc955cdfc159b01f9cc3c716ae1e51207f644f793d6bf8187f26bda34e14
    case: TC-cli-change-ready-cross-module-case
    case_digest: 9f35cd45d6d08282e4d49df0bf8f98767346d5c83ae6e91e0f2c5be79e097b75
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:10:40Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-draft-fixes-cb8167
    claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
    revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
    environment: local-macos-arm64
    content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
    fingerprint: sha256:663cd13911ec3c49f6124ce267d451280292124013f8c61b019cbcefb1fba3a4
    case: TC-cli-change-ready-draft-fixes
    case_digest: cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:10:42Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-fast-path-3b7129
    claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
    revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
    environment: local-macos-arm64
    content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
    fingerprint: sha256:981cb8796248ee2400ec5a5592a88f981114c36edfb5b8b6d1c3eb0379ae65b1
    case: TC-cli-change-ready-fast-path
    case_digest: 82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:10:45Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-missing-specsync-563f28
    claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command
    revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
    environment: local-macos-arm64
    content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
    fingerprint: sha256:06aeab90d3b832445340b1c6e85907dd1655210ffeeb79b417fdbdc309c0353a
    case: TC-cli-change-ready-missing-specsync
    case_digest: 83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:10:47Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-no-spec-change-2e6a19
    claim: "Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal
    revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
    environment: local-macos-arm64
    content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
    fingerprint: sha256:9cc023e6f1fb7a3724c876f7acc6fd19fceb16e8f1a8c1489a9fc8881ebd9f7f
    case: TC-cli-change-ready-no-spec-change
    case_digest: 9fa20b1437572a7de57f9ee3c22d6ec8e9012213e9a5e9f8d7df5337dbcae50c
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:10:50Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-outside-path-83c21c
    claim: "Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named
    revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
    environment: local-macos-arm64
    content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
    fingerprint: sha256:72fea08ced4a8838f5802f6af74ad91a9f1517230119e6f970d7429092ef4f82
    case: TC-cli-change-ready-outside-path
    case_digest: 08fb3fc5bc3f9e56789e531095851ca120f520c96e45325f5fa7d31eaf028f6f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:10:55Z
    duration_seconds: 2
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-stale-delta-e7d7e7
    claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten
    revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
    environment: local-macos-arm64
    content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
    fingerprint: sha256:1ba681e3e592324335637c5a32cd3fa2a053e1f372693575e3188df7f834032f
    case: TC-cli-change-ready-stale-delta
    case_digest: 3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:00Z
    duration_seconds: 2
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-undocumented-export-b98e9d
    claim: "Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named
    revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
    environment: local-macos-arm64
    content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
    fingerprint: sha256:371b83a920363fa8232ecf2af3baff693dabb94f6f4ea632975ee59a1fa6470b
    case: TC-cli-change-ready-undocumented-export
    case_digest: 0cb0b17d7d5e65710413d8d3c217bfb814383aaf357ecae7b605b5a430e486fa
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:05Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-unknown-change-f3c04f
    claim: "Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_unknown_change_names_status
    revision: 4f0412cee9a82aaf6a7c014deb37bdfc304581df
    environment: local-macos-arm64
    content: a239f86ddc774166764dd151e6fc265bef44903adf9325361761f723382bbb37
    fingerprint: sha256:e2d71ef63bb0400ad6fce0fb61f9bcaaeb37e9054bee55a805a57d6187f9a7e4
    case: TC-cli-change-ready-unknown-change
    case_digest: d583ee473a6bd96ea48e47ba8b11833dda832f5234e2f63fbbbcafbf10ff9493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:09Z
    duration_seconds: 0
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-carried-delta-4a3b65
    claim: "Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named
    revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
    environment: local-macos-arm64
    content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
    fingerprint: sha256:7175794a4519fc4ea25a20cb78fcd28aee28156aa762197bb08b6934af78c812
    case: TC-cli-change-ready-carried-delta
    case_digest: 06fd1cbe67c7f4336b5dbca85000d41baa9312cade6bbb57a4d42a22724f8493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:22Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-cross-module-case-8bc5bf
    claim: "Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named
    revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
    environment: local-macos-arm64
    content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
    fingerprint: sha256:9e6a2f3adc01aa0dbd2a9bde1f3afebc9e6fb9e94f69f9bcff3f9c2aa91cecde
    case: TC-cli-change-ready-cross-module-case
    case_digest: 9f35cd45d6d08282e4d49df0bf8f98767346d5c83ae6e91e0f2c5be79e097b75
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:26Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-draft-fixes-2ad9d0
    claim: "Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed
    revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
    environment: local-macos-arm64
    content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
    fingerprint: sha256:275bc981f724b5ba0c1e75fcd9201b4b4400d2b3e6eefb359d37d75eb4c769da
    case: TC-cli-change-ready-draft-fixes
    case_digest: cf12447d02e586a61ea56c157cd53f30ea07de607e7f84a72763e557440e6836
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:32Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-fast-path-3260d3
    claim: "Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change
    revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
    environment: local-macos-arm64
    content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
    fingerprint: sha256:0841d35ca782bb9b9729b720497617ea1d00741b27b7c478862aadf9ab5330a5
    case: TC-cli-change-ready-fast-path
    case_digest: 82b1aa3c277db9be41403babc4fd805575bc75dbe9a79f2e653e3ea189b064b0
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:35Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-missing-specsync-e73185
    claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command
    revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
    environment: local-macos-arm64
    content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
    fingerprint: sha256:f96f14ef5b901fcf1c2d0c7d60fb1ae7fb770e902e60b3d60ea7977e6743d0dc
    case: TC-cli-change-ready-missing-specsync
    case_digest: 83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:40Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-no-spec-change-086754
    claim: "Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal
    revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
    environment: local-macos-arm64
    content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
    fingerprint: sha256:33b581f9bed70fab899832e01a365a34fe3dd4c1666e4cfe5539c6a65f448856
    case: TC-cli-change-ready-no-spec-change
    case_digest: 9fa20b1437572a7de57f9ee3c22d6ec8e9012213e9a5e9f8d7df5337dbcae50c
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:45Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-outside-path-23a0bc
    claim: "Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named
    revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
    environment: local-macos-arm64
    content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
    fingerprint: sha256:f03a3082c6fe15f2327e0b9cc5cffcde1a5b308921157edf2b1b6574ae8e45ff
    case: TC-cli-change-ready-outside-path
    case_digest: 08fb3fc5bc3f9e56789e531095851ca120f520c96e45325f5fa7d31eaf028f6f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:51Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-stale-delta-f19345
    claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten
    revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
    environment: local-macos-arm64
    content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
    fingerprint: sha256:79b09dad45278d24d326a185cfe6f289f1eff0d0cd16bb691c89d454f8c101d0
    case: TC-cli-change-ready-stale-delta
    case_digest: 3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:56Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-undocumented-export-d15a7c
    claim: "Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named
    revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
    environment: local-macos-arm64
    content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
    fingerprint: sha256:b6d0ae4fdf8339e6a1a3f53d7588a00344995a649f3625a5519cc38b13160481
    case: TC-cli-change-ready-undocumented-export
    case_digest: 0cb0b17d7d5e65710413d8d3c217bfb814383aaf357ecae7b605b5a430e486fa
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:15:01Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-ready-unknown-change-3ac49e
    claim: "Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact an_unknown_change_names_status
    revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
    environment: local-macos-arm64
    content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
    fingerprint: sha256:06bc2c6af51369c89e3909ab582df0869236e127d31a19c2d34503c7ac7e25be
    case: TC-cli-change-ready-unknown-change
    case_digest: d583ee473a6bd96ea48e47ba8b11833dda832f5234e2f63fbbbcafbf10ff9493
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:15:04Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
```
