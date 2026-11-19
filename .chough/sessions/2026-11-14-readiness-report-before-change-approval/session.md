---
kind: session
id: 2026-11-14-readiness-report-before-change-approval
aliases: [readiness-report-before-change-approval]
objective: One read-only command lists everything still missing before a change is approved, and names the command that fixes each piece (DRIVE-29).
intent_scope: DRIVE
modules: [cli]
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: readiness-report-before-change-approval
pr: ""
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
cases: []
design: []
files: []
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
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of readiness-report-before-change-approval
    holds: accept
    drives: review
    opened: 2026-11-18T15:00:11Z
  - gate: Scope approval of readiness-report-before-change-approval with the own-work wording (native approval)
    holds: accept
    drives: approve
    opened: 2026-11-19T03:57:21Z
run_consents: []
next: Wait for Leif's implementation review of readiness-report-before-change-approval
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

## What was verified

- [Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-change-ready-draft-fixes.md)
- [Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-change-ready-stale-delta.md)
- [Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-change-ready-carried-delta.md)
- [Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-change-ready-undocumented-export.md)
- [Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-change-ready-cross-module-case.md)
- [Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-change-ready-outside-path.md)
- [Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-change-ready-no-spec-change.md)
- [Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-change-ready-unknown-change.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-change-ready-missing-specsync.md)
- [Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-change-ready-fast-path.md)
- [Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-carried-delta.md)
- [Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-cross-module-case.md)
- [Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-draft-fixes.md)
- [Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-fast-path.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-missing-specsync.md)
- [Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-no-spec-change.md)
- [Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-outside-path.md)
- [Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-stale-delta.md)
- [Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-undocumented-export.md)
- [Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-unknown-change.md)
- [Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-carried-delta-c74f5b.md)
- [Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-cross-module-case-b06d7d.md)
- [Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-draft-fixes-d3f705.md)
- [Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-fast-path-dd926a.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-missing-specsync-fcea02.md)
- [Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-no-spec-change-473e45.md)
- [Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-outside-path-36e6bd.md)
- [Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-stale-delta-dde404.md)
- [Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-undocumented-export-b7af39.md)
- [Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-unknown-change-542c9b.md)
- [Ran cargo test --locked --test ready -- --exact a_delta_that_already_carries_the_canonical_move_is_not_named for TC-cli-change-ready-carried-delta: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-carried-delta-5b3cd3.md)
- [Ran cargo test --locked --test ready -- --exact a_case_that_links_another_modules_requirement_is_named for TC-cli-change-ready-cross-module-case: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-cross-module-case-8b4128.md)
- [Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-draft-fixes-b7fa59.md)
- [Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-change-ready-fast-path: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-fast-path-f3cec5.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-missing-specsync-ec0126.md)
- [Ran cargo test --locked --test ready -- --exact a_no_spec_change_names_every_delta_for_removal for TC-cli-change-ready-no-spec-change: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-no-spec-change-e0665b.md)
- [Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-outside-path-56bda7.md)
- [Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-stale-delta-7b9dee.md)
- [Ran cargo test --locked --test ready -- --exact an_undocumented_export_in_a_declared_file_is_named for TC-cli-change-ready-undocumented-export: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-undocumented-export-97768a.md)
- [Ran cargo test --locked --test ready -- --exact an_unknown_change_names_status for TC-cli-change-ready-unknown-change: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-ready-unknown-change-37efe5.md)

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
