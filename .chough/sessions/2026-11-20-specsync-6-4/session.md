---
kind: session
id: 2026-11-20-specsync-6-4
aliases: [specsync-6-4]
objective: "Chough moves to SpecSync 6.4.0: change ready asks SpecSync's approval dry run and stops copying its delta-set rule, spec history is recorded in fragments with no spec merge line, and change ready leaves .specsync/hashes.json alone (#404, #406, #307)"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
pr: ""
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
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Gate \"Re-approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments with the skill-release fixture added (native approval)\" closed (approved); run `chough stage advance` to move on."
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
