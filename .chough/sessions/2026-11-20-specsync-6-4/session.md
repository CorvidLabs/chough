---
kind: session
id: 2026-11-20-specsync-6-4
aliases: [specsync-6-4]
objective: "Chough moves to SpecSync 6.4.0: change ready asks SpecSync's approval dry run and stops copying its delta-set rule, spec history is recorded in fragments with no spec merge line, and change ready leaves .specsync/hashes.json alone (#404, #406, #307)"
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
pr: "429"
criteria: [DRIVE-29, DRIVE-12]
requirements: []
cases: []
design: []
files: []
observations:
  - 2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals
  - 2026-11-20-tc-cli-ready-leaves-the-delta-set-to-specsync
  - 2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0
  - 2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files
  - 2026-11-20-tc-cli-ready-leaves-out-another-change-s-work
  - 2026-11-20-tc-cli-ready-makes-no-hash-file
  - 2026-11-20-tc-cli-every-specsync-pin-names-6-4-0
  - 2026-11-20-tc-cli-specsync-merge-driver-wired
  - 2026-11-20-tc-cli-change-ready-draft-fixes
  - 2026-11-20-tc-cli-change-ready-missing-specsync
  - 2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals-b03a51
  - 2026-11-20-tc-cli-ready-leaves-the-delta-set-to-specsync-392209
  - 2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0-e43745
  - 2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files-062d19
  - 2026-11-20-tc-cli-ready-leaves-out-another-change-s-work-ffa494
  - 2026-11-20-tc-cli-ready-makes-no-hash-file-15c868
  - 2026-11-20-tc-cli-every-specsync-pin-names-6-4-0-67db23
  - 2026-11-20-tc-cli-specsync-merge-driver-wired-5af15c
  - 2026-11-20-tc-cli-change-ready-draft-fixes-24ebfb
  - 2026-11-20-tc-cli-change-ready-missing-specsync-756cca
  - 2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals-f04606
  - 2026-11-20-tc-cli-ready-leaves-the-delta-set-to-specsync-05f929
  - 2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0-021474
  - 2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files-f5ed65
  - 2026-11-20-tc-cli-ready-leaves-out-another-change-s-work-ce7f95
  - 2026-11-20-tc-cli-ready-makes-no-hash-file-d08f07
  - 2026-11-20-tc-cli-every-specsync-pin-names-6-4-0-bce2c4
  - 2026-11-20-tc-cli-ready-a-whole-project-scope-leaves-nothing-out
  - 2026-11-20-tc-cli-specsync-merge-driver-wired-a2a004
  - 2026-11-20-tc-cli-change-ready-draft-fixes-88ee6f
  - 2026-11-20-tc-cli-change-ready-missing-specsync-0c52e1
  - 2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals-51a037
  - 2026-11-20-tc-cli-ready-leaves-the-delta-set-to-specsync-45be9c
  - 2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0-4a584e
  - 2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files-6eb1dd
  - 2026-11-20-tc-cli-ready-leaves-out-another-change-s-work-7a75ed
  - 2026-11-20-tc-cli-ready-makes-no-hash-file-3b08a5
  - 2026-11-20-tc-cli-every-specsync-pin-names-6-4-0-72629b
  - 2026-11-20-tc-cli-ready-a-whole-project-scope-leaves-nothing-out-8bfa41
  - 2026-11-20-tc-cli-specsync-merge-driver-wired-b97ff6
  - 2026-11-20-tc-cli-change-ready-draft-fixes-0d0d83
  - 2026-11-20-tc-cli-change-ready-missing-specsync-ed7d72
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
title: Chough moves to SpecSync 6.4.0, and change ready asks its dry run
summary:
  - "chough change ready names what SpecSync 6.4.0's approval dry run refuses, in its words, in place of its own delta-set rule (#406, #307); it leaves SpecSync's own untracked change.lock and workflow-v2-baseline.json and another active change's work out of the paths check, a whole-project scope leaving nothing out; it no longer touches .specsync/hashes.json, which 6.4.0 keeps outside the work tree. Every pin names 6.4.0, and the spec merge line goes since 6.4.0 records history in fragments (#404)."
  - "Verified: the tester's PASS on 66 real changes at d4f520f6; tester2's recheck at d1f953a4; corvid-agent's approval at d1f953a4; main 51352d33 merged in, own lines the same; full lane green with 6.4.0 at 7eb2db10."
---
# Chough moves to SpecSync 6.4.0: change ready asks SpecSync's approval dry run and stops copying its delta-set rule, spec history is recorded in fragments with no spec merge line, and change ready leaves .specsync/hashes.json alone (#404, #406, #307)

## Goal

Chough moves to SpecSync 6.4.0: change ready asks SpecSync's approval dry run and stops copying its delta-set rule, spec history is recorded in fragments with no spec merge line, and change ready leaves .specsync/hashes.json alone (#404, #406, #307)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments (native approval)": "Approve (Recommended)" (Leif's own answers in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 18:51Z: to 'Chough #404 (moving Chough onto SpecSync 6.4.0, with the rest of #406 and #307), scope approval. ... Approve the scope?' he answered 'Approve (Recommended)'; to Q1, 'should change ready drop its own delta-set rule, since the dry run names the same missing and extra deltas?', 'A: drop it (Recommended)'; to Q2, 'should the paths check leave out another active change's work on the same branch?', 'A: yes, now (Recommended)'. It merges only after 6.4.0 is on crates.io, ideally with no other Chough PR open. Recorded by t4c itself, at the draft a9b57dd7); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments as 0xLeif
- 0xLeif decided "One send-back of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments to add four paths": "Standing OK, tell me after" (Two of Leif's standing OKs, relayed via t4c: (e) for case files only (his 'Ok', about 03:00Z on 2026-11-20), for .chough/cases/cli.change-ready.cases.toml, whose case TC-cli-change-ready-no-spec-change the approved removal of the delta-set rule (his Q1 'A: drop it') makes false; and his standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c, for three files the approved pin move makes false: skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/lifecycle.md and docs/exercises/feature-bug-loop.md. Path-only: no delta word changes, no new refusal. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments with four paths added (native approval)": "Standing OK, tell me after" (Two of Leif's standing OKs, relayed via t4c: (e) for case files only (his 'Ok', about 03:00Z on 2026-11-20), for .chough/cases/cli.change-ready.cases.toml, whose case TC-cli-change-ready-no-spec-change the approved removal of the delta-set rule (his Q1 'A: drop it') makes false; and his standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c, for three files the approved pin move makes false: skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/lifecycle.md and docs/exercises/feature-bug-loop.md. Path-only: no delta word changes, no new refusal. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments as 0xLeif
- 0xLeif decided "One send-back of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments to add the skill-release fixture and its test": "Standing OK, tell me after" (Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: tests/fixtures/skill-releases/manifest.json and tests/skill_releases.rs, which pin the current skill bundle that the approved pin move's skill reference changes, as #351 did; no delta word changes, no new refusal. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments with the skill-release fixture added (native approval)": "Standing OK, tell me after" (Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: tests/fixtures/skill-releases/manifest.json and tests/skill_releases.rs, which pin the current skill bundle that the approved pin move's skill reference changes, as #351 did; no delta word changes, no new refusal. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments as 0xLeif
- 0xLeif decided "One send-back of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments: a declared path covering the whole project leaves nothing out": "A: '.' leaves nothing out (Recommended)" (Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 22:02Z, to: 'Chough #404, an edge case the tester found in NFTMix. Under your Q2 "A", the paths check leaves out another active change's declared paths. But one NFTMix change declares the whole project ("."), so every other change's "outside the declared paths" findings disappear. Fix: a declared path that covers the whole project (".") leaves nothing out, since it doesn't show any file is that change's work. Narrower ones like "src/" still work as approved. That's one clause and one criterion in REQ-cli-067, through a send-back with a red test first.' Recorded by t4c); recorded by t4c; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments with a whole-project scope leaving nothing out (native approval)": "A: '.' leaves nothing out (Recommended)" (Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 22:02Z, to: 'Chough #404, an edge case the tester found in NFTMix. Under your Q2 "A", the paths check leaves out another active change's declared paths. But one NFTMix change declares the whole project ("."), so every other change's "outside the declared paths" findings disappear. Fix: a declared path that covers the whole project (".") leaves nothing out, since it doesn't show any file is that change's work. Narrower ones like "src/" still work as approved. That's one clause and one criterion in REQ-cli-067, through a send-back with a red test first.' Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments as 0xLeif
- 0xLeif decided "Implementation review of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments": "Go" (Standing OK 6(b): Leif's 'Go' to the six rules (Claude Code chat message of 2026-11-19, relayed by his planning session hc t4c), recording his implementation review of a batch with tester PASSes. The tester's rule-3 PASS of change ready on the 66 real open changes at d4f520f6; its NFTMix case (a change declaring '.') led to Leif's own 'A: . leaves nothing out' of 2026-11-20 about 22:02Z, recorded by t4c through a send-back that added exactly one clause and one criterion to REQ-cli-067 (f4b9df1d), built test first (d69a7c44). tester2's recheck PASS at d1f953a4: NFTMix's 32 findings back, the 66 changes matching the tester's run plus those 32, nothing new over main. corvid-agent approved #429 at a2500c4e and again at d1f953a4; its three notes answered (a docs line taken, two left as approved). Main e895943c and then 51352d33 merged in, each cleanly, t4b's script showing the 28 declared paths' own lines the same; the full lane green with SpecSync 6.4.0 at 7eb2db10. No approved sentence changed beyond Leif's '.' answer. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change review specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test ready -- --exact a_refusal_of_the_approval_dry_run_is_named_with_its_fix for TC-cli-ready-names-the-approval-dry-run-s-refusals: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_delta_is_named_by_specsync_alone for TC-cli-ready-leaves-the-delta-set-to-specsync: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-the-delta-set-to-specsync.md)
- [Ran cargo test --locked --test ready -- --exact a_specsync_without_the_dry_run_names_the_install_command for TC-cli-ready-without-a-dry-run-names-6-4-0: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0.md)
- [Ran cargo test --locked --test ready -- --exact specsync_s_own_untracked_files_are_not_outside_the_declared_paths for TC-cli-ready-leaves-out-specsync-s-own-files: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files.md)
- [Ran cargo test --locked --test ready -- --exact another_active_change_s_work_is_not_this_change_s for TC-cli-ready-leaves-out-another-change-s-work: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-out-another-change-s-work.md)
- [Ran cargo test --locked --test ready -- --exact ready_makes_no_hash_file_for_specsync_to_find for TC-cli-ready-makes-no-hash-file: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-makes-no-hash-file.md)
- [Ran cargo test --locked --test package -- --exact every_specsync_pin_names_6_4_0 for TC-cli-every-specsync-pin-names-6-4-0: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-every-specsync-pin-names-6-4-0.md)
- [Ran cargo test --locked --test package -- --exact requirements_merge_through_specsync_s_driver_and_spec_files_need_none for TC-cli-specsync-merge-driver-wired: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-specsync-merge-driver-wired.md)
- [Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-draft-fixes.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-missing-specsync.md)
- [Ran cargo test --locked --test ready -- --exact a_refusal_of_the_approval_dry_run_is_named_with_its_fix for TC-cli-ready-names-the-approval-dry-run-s-refusals: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals-b03a51.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_delta_is_named_by_specsync_alone for TC-cli-ready-leaves-the-delta-set-to-specsync: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-the-delta-set-to-specsync-392209.md)
- [Ran cargo test --locked --test ready -- --exact a_specsync_without_the_dry_run_names_the_install_command for TC-cli-ready-without-a-dry-run-names-6-4-0: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0-e43745.md)
- [Ran cargo test --locked --test ready -- --exact specsync_s_own_untracked_files_are_not_outside_the_declared_paths for TC-cli-ready-leaves-out-specsync-s-own-files: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files-062d19.md)
- [Ran cargo test --locked --test ready -- --exact another_active_change_s_work_is_not_this_change_s for TC-cli-ready-leaves-out-another-change-s-work: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-out-another-change-s-work-ffa494.md)
- [Ran cargo test --locked --test ready -- --exact ready_makes_no_hash_file_for_specsync_to_find for TC-cli-ready-makes-no-hash-file: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-makes-no-hash-file-15c868.md)
- [Ran cargo test --locked --test package -- --exact every_specsync_pin_names_6_4_0 for TC-cli-every-specsync-pin-names-6-4-0: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-every-specsync-pin-names-6-4-0-67db23.md)
- [Ran cargo test --locked --test package -- --exact requirements_merge_through_specsync_s_driver_and_spec_files_need_none for TC-cli-specsync-merge-driver-wired: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-specsync-merge-driver-wired-5af15c.md)
- [Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-draft-fixes-24ebfb.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-missing-specsync-756cca.md)
- [Ran cargo test --locked --test ready -- --exact a_refusal_of_the_approval_dry_run_is_named_with_its_fix for TC-cli-ready-names-the-approval-dry-run-s-refusals: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals-f04606.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_delta_is_named_by_specsync_alone for TC-cli-ready-leaves-the-delta-set-to-specsync: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-the-delta-set-to-specsync-05f929.md)
- [Ran cargo test --locked --test ready -- --exact a_specsync_without_the_dry_run_names_the_install_command for TC-cli-ready-without-a-dry-run-names-6-4-0: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0-021474.md)
- [Ran cargo test --locked --test ready -- --exact specsync_s_own_untracked_files_are_not_outside_the_declared_paths for TC-cli-ready-leaves-out-specsync-s-own-files: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files-f5ed65.md)
- [Ran cargo test --locked --test ready -- --exact another_active_change_s_work_is_not_this_change_s for TC-cli-ready-leaves-out-another-change-s-work: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-out-another-change-s-work-ce7f95.md)
- [Ran cargo test --locked --test ready -- --exact ready_makes_no_hash_file_for_specsync_to_find for TC-cli-ready-makes-no-hash-file: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-makes-no-hash-file-d08f07.md)
- [Ran cargo test --locked --test package -- --exact every_specsync_pin_names_6_4_0 for TC-cli-every-specsync-pin-names-6-4-0: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-every-specsync-pin-names-6-4-0-bce2c4.md)
- [Ran cargo test --locked --test ready -- --exact another_change_declaring_the_whole_project_leaves_nothing_out for TC-cli-ready-a-whole-project-scope-leaves-nothing-out: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-a-whole-project-scope-leaves-nothing-out.md)
- [Ran cargo test --locked --test package -- --exact requirements_merge_through_specsync_s_driver_and_spec_files_need_none for TC-cli-specsync-merge-driver-wired: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-specsync-merge-driver-wired-a2a004.md)
- [Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-draft-fixes-88ee6f.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-missing-specsync-0c52e1.md)
- [Ran cargo test --locked --test ready -- --exact a_refusal_of_the_approval_dry_run_is_named_with_its_fix for TC-cli-ready-names-the-approval-dry-run-s-refusals: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-names-the-approval-dry-run-s-refusals-51a037.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_delta_is_named_by_specsync_alone for TC-cli-ready-leaves-the-delta-set-to-specsync: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-the-delta-set-to-specsync-45be9c.md)
- [Ran cargo test --locked --test ready -- --exact a_specsync_without_the_dry_run_names_the_install_command for TC-cli-ready-without-a-dry-run-names-6-4-0: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0-4a584e.md)
- [Ran cargo test --locked --test ready -- --exact specsync_s_own_untracked_files_are_not_outside_the_declared_paths for TC-cli-ready-leaves-out-specsync-s-own-files: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-out-specsync-s-own-files-6eb1dd.md)
- [Ran cargo test --locked --test ready -- --exact another_active_change_s_work_is_not_this_change_s for TC-cli-ready-leaves-out-another-change-s-work: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-leaves-out-another-change-s-work-7a75ed.md)
- [Ran cargo test --locked --test ready -- --exact ready_makes_no_hash_file_for_specsync_to_find for TC-cli-ready-makes-no-hash-file: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-makes-no-hash-file-3b08a5.md)
- [Ran cargo test --locked --test package -- --exact every_specsync_pin_names_6_4_0 for TC-cli-every-specsync-pin-names-6-4-0: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-every-specsync-pin-names-6-4-0-72629b.md)
- [Ran cargo test --locked --test ready -- --exact another_change_declaring_the_whole_project_leaves_nothing_out for TC-cli-ready-a-whole-project-scope-leaves-nothing-out: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-ready-a-whole-project-scope-leaves-nothing-out-8bfa41.md)
- [Ran cargo test --locked --test package -- --exact requirements_merge_through_specsync_s_driver_and_spec_files_need_none for TC-cli-specsync-merge-driver-wired: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-specsync-merge-driver-wired-b97ff6.md)
- [Ran cargo test --locked --test ready -- --exact a_draft_names_each_missing_fix_and_passes_once_they_are_fixed for TC-cli-change-ready-draft-fixes: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-draft-fixes-0d0d83.md)
- [Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-missing-specsync-ed7d72.md)

## Open threads

## Timeline

- 2026-11-20T18:27:53Z · start · intent → intent · Chough moves to SpecSync 6.4.0: change ready asks SpecSync's approval dry run and stops copying its delta-set rule, spec history is recorded in fragments with no spec merge line, and change ready leaves .specsync/hashes.json alone (#404, #406, #307) ^t-e4dc64
  - revision: de49485fc5fb93f05bb15db7cd471023beebd8a8
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T18:27:53Z · advance · intent → design ·  ^t-130b88
  - revision: de49485fc5fb93f05bb15db7cd471023beebd8a8
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T18:37:44Z · gate-opened · design → design · Scope approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments (native approval) ^t-da1ade
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: f48c7e80182103df7135024e4f0e7382b97410e7
  - holds: design
  - drives: approve
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T18:52:11Z · gate-closed · design → design · Scope approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments (native approval) ^t-013aac
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: a9b57dd754f1d6198b28b815bb5d6fd590398901
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve (Recommended)
    - where: Leif's own answers in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 18:51Z: to 'Chough #404 (moving Chough onto SpecSync 6.4.0, with the rest of #406 and #307), scope approval. ... Approve the scope?' he answered 'Approve (Recommended)'; to Q1, 'should change ready drop its own delta-set rule, since the dry run names the same missing and extra deltas?', 'A: drop it (Recommended)'; to Q2, 'should the paths check leave out another active change's work on the same branch?', 'A: yes, now (Recommended)'. It merges only after 6.4.0 is on crates.io, ideally with no other Chough PR open. Recorded by t4c itself, at the draft a9b57dd7
    - recorded_by: t4c
    - native: specsync change approve specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T18:52:41Z · advance · design → plan ·  ^t-2f5d5b
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: df5af25555a000155084b5aa1426cb9ecaf4a657
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T18:52:41Z · advance · plan → build · native:approved ^t-152041
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - gate: native:approved
  - revision: df5af25555a000155084b5aa1426cb9ecaf4a657
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T19:08:42Z · gate-opened · build → build · One send-back of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments to add four paths ^t-12c01e
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: 4d681b44bfea21d7fcf1eea0ec75e38f2487d1ef
  - holds: build
  - drives: none
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T19:08:42Z · gate-closed · build → build · One send-back of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments to add four paths ^t-910a4e
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: 4d681b44bfea21d7fcf1eea0ec75e38f2487d1ef
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Two of Leif's standing OKs, relayed via t4c: (e) for case files only (his 'Ok', about 03:00Z on 2026-11-20), for .chough/cases/cli.change-ready.cases.toml, whose case TC-cli-change-ready-no-spec-change the approved removal of the delta-set rule (his Q1 'A: drop it') makes false; and his standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c, for three files the approved pin move makes false: skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/lifecycle.md and docs/exercises/feature-bug-loop.md. Path-only: no delta word changes, no new refusal. Recorded by t4c
    - recorded_by: t4c
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T19:08:44Z · gate-opened · build → build · Re-approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments with four paths added (native approval) ^t-20c784
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: 4d681b44bfea21d7fcf1eea0ec75e38f2487d1ef
  - holds: build
  - drives: approve
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T19:08:44Z · direct-native-write · build → build · specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-6a5966
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: 4d681b44bfea21d7fcf1eea0ec75e38f2487d1ef
  - direct_write:
    - change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T19:08:44Z · gate-closed · build → build · Re-approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments with four paths added (native approval) ^t-788327
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: 4d681b44bfea21d7fcf1eea0ec75e38f2487d1ef
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Two of Leif's standing OKs, relayed via t4c: (e) for case files only (his 'Ok', about 03:00Z on 2026-11-20), for .chough/cases/cli.change-ready.cases.toml, whose case TC-cli-change-ready-no-spec-change the approved removal of the delta-set rule (his Q1 'A: drop it') makes false; and his standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c, for three files the approved pin move makes false: skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/lifecycle.md and docs/exercises/feature-bug-loop.md. Path-only: no delta word changes, no new refusal. Recorded by t4c
    - recorded_by: t4c
    - native: specsync change approve specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T19:34:47Z · gate-opened · build → build · One send-back of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments to add the skill-release fixture and its test ^t-39b714
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: cf4fa9a21fdeabfed08e4cdc599643592fd019db
  - holds: build
  - drives: none
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T19:34:47Z · gate-closed · build → build · One send-back of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments to add the skill-release fixture and its test ^t-7db69a
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: cf4fa9a21fdeabfed08e4cdc599643592fd019db
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: tests/fixtures/skill-releases/manifest.json and tests/skill_releases.rs, which pin the current skill bundle that the approved pin move's skill reference changes, as #351 did; no delta word changes, no new refusal. Recorded by t4c
    - recorded_by: t4c
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T19:34:49Z · gate-opened · build → build · Re-approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments with the skill-release fixture added (native approval) ^t-ca2a95
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: cf4fa9a21fdeabfed08e4cdc599643592fd019db
  - holds: build
  - drives: approve
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T19:34:49Z · direct-native-write · build → build · specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-d2f6db
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: cf4fa9a21fdeabfed08e4cdc599643592fd019db
  - direct_write:
    - change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T19:34:49Z · gate-closed · build → build · Re-approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments with the skill-release fixture added (native approval) ^t-82b477
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: cf4fa9a21fdeabfed08e4cdc599643592fd019db
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: tests/fixtures/skill-releases/manifest.json and tests/skill_releases.rs, which pin the current skill bundle that the approved pin move's skill reference changes, as #351 did; no delta word changes, no new refusal. Recorded by t4c
    - recorded_by: t4c
    - native: specsync change approve specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T22:04:24Z · gate-opened · build → build · One send-back of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments: a declared path covering the whole project leaves nothing out ^t-cede7f
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: 402cda5c6189a54cd26bd1cb590f9174fd4f2dd9
  - holds: build
  - drives: none
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T22:04:24Z · gate-closed · build → build · One send-back of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments: a declared path covering the whole project leaves nothing out ^t-b76faa
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: 402cda5c6189a54cd26bd1cb590f9174fd4f2dd9
  - relayed_by:
    - decided_by: 0xLeif
    - words: A: '.' leaves nothing out (Recommended)
    - where: Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 22:02Z, to: 'Chough #404, an edge case the tester found in NFTMix. Under your Q2 "A", the paths check leaves out another active change's declared paths. But one NFTMix change declares the whole project ("."), so every other change's "outside the declared paths" findings disappear. Fix: a declared path that covers the whole project (".") leaves nothing out, since it doesn't show any file is that change's work. Narrower ones like "src/" still work as approved. That's one clause and one criterion in REQ-cli-067, through a send-back with a red test first.' Recorded by t4c
    - recorded_by: t4c
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T22:04:34Z · gate-opened · build → build · Re-approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments with a whole-project scope leaving nothing out (native approval) ^t-ec124d
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: 402cda5c6189a54cd26bd1cb590f9174fd4f2dd9
  - holds: build
  - drives: approve
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T22:04:35Z · direct-native-write · build → build · specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-de6968
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: 402cda5c6189a54cd26bd1cb590f9174fd4f2dd9
  - direct_write:
    - change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T22:04:35Z · gate-closed · build → build · Re-approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments with a whole-project scope leaving nothing out (native approval) ^t-c4d71d
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: 402cda5c6189a54cd26bd1cb590f9174fd4f2dd9
  - relayed_by:
    - decided_by: 0xLeif
    - words: A: '.' leaves nothing out (Recommended)
    - where: Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 22:02Z, to: 'Chough #404, an edge case the tester found in NFTMix. Under your Q2 "A", the paths check leaves out another active change's declared paths. But one NFTMix change declares the whole project ("."), so every other change's "outside the declared paths" findings disappear. Fix: a declared path that covers the whole project (".") leaves nothing out, since it doesn't show any file is that change's work. Narrower ones like "src/" still work as approved. That's one clause and one criterion in REQ-cli-067, through a send-back with a red test first.' Recorded by t4c
    - recorded_by: t4c
    - native: specsync change approve specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T23:43:49Z · advance · build → test · native:verifying ^t-b07151
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - gate: native:verifying
  - revision: 7eb2db10031f5d2b254b117103f2a2986e40eddc
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T23:43:49Z · advance · test → accept · native:verifying ^t-0a139e
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - gate: native:verifying
  - revision: 7eb2db10031f5d2b254b117103f2a2986e40eddc
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T23:43:49Z · gate-opened · accept → accept · Implementation review of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments ^t-6aab8c
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: 7eb2db10031f5d2b254b117103f2a2986e40eddc
  - holds: accept
  - drives: review
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T23:45:10Z · gate-closed · accept → accept · Implementation review of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments ^t-1dba42
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: f2781de4769eb919640066a080bdfea8138505fb
  - relayed_by:
    - decided_by: 0xLeif
    - words: Go
    - where: Standing OK 6(b): Leif's 'Go' to the six rules (Claude Code chat message of 2026-11-19, relayed by his planning session hc t4c), recording his implementation review of a batch with tester PASSes. The tester's rule-3 PASS of change ready on the 66 real open changes at d4f520f6; its NFTMix case (a change declaring '.') led to Leif's own 'A: . leaves nothing out' of 2026-11-20 about 22:02Z, recorded by t4c through a send-back that added exactly one clause and one criterion to REQ-cli-067 (f4b9df1d), built test first (d69a7c44). tester2's recheck PASS at d1f953a4: NFTMix's 32 findings back, the 66 changes matching the tester's run plus those 32, nothing new over main. corvid-agent approved #429 at a2500c4e and again at d1f953a4; its three notes answered (a docs line taken, two left as approved). Main e895943c and then 51352d33 merged in, each cleanly, t4b's script showing the 28 declared paths' own lines the same; the full lane green with SpecSync 6.4.0 at 7eb2db10. No approved sentence changed beyond Leif's '.' answer. Recorded by t4c
    - recorded_by: t4c
    - native: specsync change review specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T23:45:12Z · advance · accept → done · native:archived ^t-fd9a27
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - gate: native:archived
  - revision: f2781de4769eb919640066a080bdfea8138505fb
  - by: t4c (claude-opus-5-5, hc t4c)
