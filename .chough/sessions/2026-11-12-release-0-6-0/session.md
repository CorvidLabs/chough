---
kind: session
id: 2026-11-12-release-0-6-0
aliases: [release-0-6-0]
objective: "Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs"
intent_scope: ""
modules: []
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
pr: ""
criteria: [KNOW-3, KNOW-4, KNOW-6, KNOW-15, KNOW-24, DRIVE-27]
requirements: []
cases: []
design: []
files: []
observations:
  - 2026-11-12-tc-records-a-0-5-pointer-still-names-its-session
  - 2026-11-12-tc-records-a-0-5-pointer-still-names-its-session-b81047
  - 2026-11-12-tc-cli-init-preserves-knowledge
  - 2026-11-12-tc-cli-init-unbounded-knowledge-refused
  - 2026-11-12-tc-cli-vault-crlf-readme-unmatched-backtick
  - 2026-11-12-tc-cli-vault-links-and-copies-no-text
  - 2026-11-12-tc-cli-vault-readme-upgrade
  - 2026-11-12-tc-cli-vault-regenerates-identically
  - 2026-11-12-tc-cli-vault-rules-note
  - 2026-11-12-tc-cli-vault-snapshots-and-briefs-stay
  - 2026-11-12-tc-records-tree-and-readme
  - 2026-11-12-the-know-gap-list-216-is-empty-computed-from-hi-knowledge-md
  - 2026-11-12-tc-cli-empty-change-dir-not-open
  - 2026-11-12-tc-cli-empty-change-dir-not-open-483046
  - 2026-11-12-dogfood-check-the-0-5-0-build-on-a-clone-of-the-migrated-tree
  - 2026-11-12-tc-cli-gate-summary-counts-from-the-changes-own-base
  - 2026-11-12-tc-cli-gate-summary-counts-from-the-changes-own-base-afcf46
  - 2026-11-12-tc-cli-190-gate-summary-counts-the-change
  - 2026-11-12-tc-cli-gate-summary-says-when-there-is-no-base
  - 2026-11-12-tc-records-the-migrate-preview-warns-of-open-changes
  - 2026-11-12-tc-records-the-migrate-preview-warns-of-open-changes-1c9708
  - 2026-11-12-tc-records-migrate-from-0-5
  - 2026-11-12-reproduced-the-jackdaw-finding-t38f-on-a-rehearsal-migrated
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Draft the re-scope: REQ-records-011 (titles, summaries, --record, backfill), REQ-records-006 (case-definition temporary roots), the checkpoint options' requirement, skills/ in paths; then the gate and Leif's quoted words"
---
# Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs

## Goal

Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs

## Where it stands

## Decisions and why

- decided: the 0.6.0 release preparation is the dogfood cycle, one small native change driven with the 0.6.0 build only, scoped by t370 (relaying Leif's plan): Cargo 0.6.0 and the dated CHANGELOG with dogfood results, D1, the CodeQL helper rename, the KNOW case links and runs, #148 red then green, and settling the four flaky-test bugs; no tag, release or merge without Leif
- Leif decided "Approve the scope of the 0.6.0 release preparation": "Yes I approve" (coordinator chat (Claude Code, hc t370), 2026-11-12T14:44Z, replying to the coordinator's summary of the 0.6.0 release-prep scope; D2 deferred by Leif's choice 'Fix after 0.6.0' at 2026-11-12T14:46Z; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood as 0xLeif
- decided: re-scoped the release prep on Leif's 'Block the tag' (2026-11-12T15:14Z): D2's REQ-cli-061 delta, the migrate preview warning (a REQ-records-009 delta) and D1's pointer clarification (a REQ-history-006 delta, since delta modules must match the affected specs cli, history and records, which own the code this change touches); SpecSync offers no command to clear no_spec_change, which approve refuses with a public contract change, so I set it false in state.json by hand (a direct lifecycle write); rehearsed approve and change check --commit in a throwaway clone with a synthetic actor first, verified
- Leif decided "Approve the re-scoped 0.6.0 release preparation": "Yes, go ahead" (coordinator chat (Claude Code, hc t370), 2026-11-12T15:24Z, answering the coordinator's plain summary of the three contract sentences (REQ-cli-061, REQ-records-009, REQ-history-006) for the re-scoped 0.6.0 release prep; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood as 0xLeif

## What was verified

- [Ran cargo test --locked --test migrate -- --exact a_pointer_naming_a_migrated_key_still_names_its_session for TC-records-a-0-5-pointer-still-names-its-session: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-records-a-0-5-pointer-still-names-its-session.md)
- [Ran cargo test --locked --test migrate -- --exact a_pointer_naming_a_migrated_key_still_names_its_session for TC-records-a-0-5-pointer-still-names-its-session: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-records-a-0-5-pointer-still-names-its-session-b81047.md)
- [Ran cargo test --locked --test knowledge_default -- --exact init_preserves_an_existing_knowledge_directory for TC-cli-init-preserves-knowledge: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-init-preserves-knowledge.md)
- [Ran cargo test --locked --test knowledge_default -- --exact init_still_succeeds_and_writes_no_knowledge_when_the_index_cannot_be_bounded for TC-cli-init-unbounded-knowledge-refused: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-init-unbounded-knowledge-refused.md)
- [Ran cargo test --locked --test vault -- --exact crlf_earlier_readmes_upgrade_and_unmatched_backticks_keep_the_trace_list for TC-cli-vault-crlf-readme-unmatched-backtick: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-vault-crlf-readme-unmatched-backtick.md)
- [Ran cargo test --locked --test vault -- --exact the_index_links_lessons_bundles_and_the_journal_and_copies_no_text for TC-cli-vault-links-and-copies-no-text: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-vault-links-and-copies-no-text.md)
- [Ran cargo test --locked --test vault -- --exact an_untouched_earlier_readme_is_upgraded_and_an_edited_one_kept for TC-cli-vault-readme-upgrade: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-vault-readme-upgrade.md)
- [Ran cargo test --locked --test vault -- --exact the_vault_regenerates_identically for TC-cli-vault-regenerates-identically: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-vault-regenerates-identically.md)
- [Ran cargo test --locked --test vault -- --exact the_rules_note_links_rules_skills_and_decisions_that_exist for TC-cli-vault-rules-note: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-vault-rules-note.md)
- [Ran cargo test --locked --test vault -- --exact snapshots_and_briefs_stay_as_they_are_and_snapshot_and_archive_are_gone for TC-cli-vault-snapshots-and-briefs-stay: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-vault-snapshots-and-briefs-stay.md)
- [Ran cargo test --locked --test records -- --exact the_tree_holds_only_the_listed_paths for TC-records-tree-and-readme: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-records-tree-and-readme.md)
- [The KNOW gap list (#216) is empty: computed from hi/knowledge.md and the recorded evidence (stories, bugs and this session's entries), all 17 live KNOW criteria have an active case whose latest receipt passed, and all 18 retired ones show as retired](evidence/2026-11-12-the-know-gap-list-216-is-empty-computed-from-hi-knowledge-md.md)
- [Red for #148: #157's regression test, applied to the code just before #157 (8f620c39, the first parent of its merge b0fa5b3d), fails: an empty change directory left by a branch switch counts as an open native change](evidence/2026-11-12-tc-cli-empty-change-dir-not-open.md)
- [Ran cargo test --locked --test evidence -- --exact an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change for TC-cli-empty-change-dir-not-open: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-empty-change-dir-not-open-483046.md)
- [Dogfood check: the 0.5.0 build on a clone of the migrated tree reports no open session, refuses bug list at the first Markdown bug and writes nothing for those reads; its session new writes a 0.5.x session file, after which the 0.6.0 build refuses the tree as refused:unmigrated naming that file](evidence/2026-11-12-dogfood-check-the-0-5-0-build-on-a-clone-of-the-migrated-tree.md)
- [Ran cargo test --locked --test gates -- --exact a_second_change_on_one_branch_counts_from_its_own_base_commit for TC-cli-gate-summary-counts-from-the-changes-own-base: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-cli-gate-summary-counts-from-the-changes-own-base.md)
- [Ran cargo test --locked --test gates -- --exact a_second_change_on_one_branch_counts_from_its_own_base_commit for TC-cli-gate-summary-counts-from-the-changes-own-base: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-gate-summary-counts-from-the-changes-own-base-afcf46.md)
- [Ran cargo test --locked --test gates -- --exact gate_summary_counts_only_the_changes_own_files for TC-cli-190-gate-summary-counts-the-change: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-190-gate-summary-counts-the-change.md)
- [Ran cargo test --locked --test gates -- --exact a_summary_with_no_base_says_so_instead_of_a_broken_diff_command for TC-cli-gate-summary-says-when-there-is-no-base: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-gate-summary-says-when-there-is-no-base.md)
- [Ran cargo test --locked --test migrate -- --exact the_preview_names_open_native_changes_that_make_apply_refuse for TC-records-the-migrate-preview-warns-of-open-changes: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-records-the-migrate-preview-warns-of-open-changes.md)
- [Ran cargo test --locked --test migrate -- --exact the_preview_names_open_native_changes_that_make_apply_refuse for TC-records-the-migrate-preview-warns-of-open-changes: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-records-the-migrate-preview-warns-of-open-changes-1c9708.md)
- [Ran cargo test --locked --test migrate for TC-records-migrate-from-0-5: passed (exit 0).](evidence/2026-11-12-tc-records-migrate-from-0-5.md)
- [Reproduced the jackdaw finding (t38f, on a rehearsal-migrated jackdaw tree whose case files set XDG_DATA_HOME under the temporary root, 4 findings) with the 0.6.0 build at 8d310373 in a throwaway project: a committed case file whose command binding sets XDG_DATA_HOME to a path under the temporary root (argv env XDG_DATA_HOME=<tmp>/probe-data true) made chough doctor report a leak finding (match system-path, the case file, line 19) with the fix chough doctor --fix --redact, which would rewrite the case's deliberate test setting and break the case](evidence/2026-11-12-reproduced-the-jackdaw-finding-t38f-on-a-rehearsal-migrated.md)

## Open threads

## Timeline

- 2026-11-12T14:40:28Z · advance · intent → design · Intent: the release preparation and the dogfood findings, scoped by t370 ^t-6b111f
  - revision: 166d1ea1d37971e4c95ee8ba02249a1a46b2a0d2
  - by: claude-code (hc t382)
- 2026-11-12T14:40:28Z · skip · design → plan · Release preparation and CLI fixes; nothing visual ^t-692d4f
  - revision: 166d1ea1d37971e4c95ee8ba02249a1a46b2a0d2
  - by: claude-code (hc t382)
- 2026-11-12T14:42:58Z · gate-opened · plan → plan · Approve the scope of the 0.6.0 release preparation ^t-676797
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 166d1ea1d37971e4c95ee8ba02249a1a46b2a0d2
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-12T14:53:15Z · gate-closed · plan → plan · Approve the scope of the 0.6.0 release preparation ^t-a4e2d6
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: b940bd6c1d1ee3c58522479e412f62841f01c4e5
  - relayed_by: {"decided_by":"Leif","words":"Yes I approve","where":"coordinator chat (Claude Code, hc t370), 2026-11-12T14:44Z, replying to the coordinator's summary of the 0.6.0 release-prep scope; D2 deferred by Leif's choice 'Fix after 0.6.0' at 2026-11-12T14:46Z; relayed by claude-code-t370","recorded_by":"claude-code-t382","native":"specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood","actor":"0xLeif"}
  - by: claude-code (hc t382)
- 2026-11-12T14:53:38Z · direct-native-write · plan → plan ·  ^t-a5ea23
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 92ae99a6c8ea2d7679a76f95ac9846dfbca5c0fc
  - direct_write: {"change":"prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood","files":[{"path":"state.json","change":"changed"}],"before_state":"draft","after_state":"draft"}
  - by: claude-code (hc t382)
- 2026-11-12T14:53:38Z · advance · plan → build · Scope approved by Leif at the gate (relayed, run as 0xLeif) ^t-aaf8e7
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - gate: native:approved
  - revision: 92ae99a6c8ea2d7679a76f95ac9846dfbca5c0fc
  - by: claude-code (hc t382)
- 2026-11-12T15:15:28Z · direct-native-write · build → build ·  ^t-1f4145
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 9b82a84361c74df793d321e369a7e8c435ce9fb6
  - direct_write: {"change":"prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"approved","after_state":"draft"}
  - by: claude-code (hc t382)
- 2026-11-12T15:15:28Z · back · build → plan · Leif re-scoped the release prep: D2 (REQ-cli-061 delta) and the migrate preview warning join 0.6.0 ('Block the tag', 2026-11-12T15:14Z); native change sent back to draft ^t-2c4fc4
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 9b82a84361c74df793d321e369a7e8c435ce9fb6
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, checkpoint references: 0 case(s) and 15 observation(s) to re-check with `chough status`
  - by: claude-code (hc t382)
- 2026-11-12T15:21:31Z · gate-opened · plan → plan · Approve the re-scoped 0.6.0 release preparation ^t-2b270e
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 15b3212a03f78a07c143467ff4eb21a0379e0144
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-12T15:24:30Z · gate-closed · plan → plan · Approve the re-scoped 0.6.0 release preparation ^t-ddbf99
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 7b1794d936f4e292f6431478e68611f0ed092ccf
  - relayed_by: {"decided_by":"Leif","words":"Yes, go ahead","where":"coordinator chat (Claude Code, hc t370), 2026-11-12T15:24Z, answering the coordinator's plain summary of the three contract sentences (REQ-cli-061, REQ-records-009, REQ-history-006) for the re-scoped 0.6.0 release prep; relayed by claude-code-t370","recorded_by":"claude-code-t382","native":"specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood","actor":"0xLeif"}
  - by: claude-code (hc t382)
- 2026-11-12T15:25:02Z · direct-native-write · plan → plan ·  ^t-a07144
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 7b1794d936f4e292f6431478e68611f0ed092ccf
  - direct_write: {"change":"prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood","files":[{"path":"state.json","change":"changed"}],"before_state":"draft","after_state":"draft"}
  - by: claude-code (hc t382)
- 2026-11-12T15:25:02Z · advance · plan → build · Leif approved the re-scoped definition (relayed by t370, 15:24Z); build D2 and the migrate preview warning red then green, and integrate the flake fixes ^t-3dd925
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - gate: native:approved
  - revision: 7b1794d936f4e292f6431478e68611f0ed092ccf
  - by: claude-code (hc t382)
- 2026-11-12T15:42:40Z · direct-native-write · build → build ·  ^t-6f1951
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: f4855ab4801f4e0684bdd3a35b82b3041c3a6be1
  - direct_write: {"change":"prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"approved","after_state":"draft"}
  - by: claude-code (hc t382)
- 2026-11-12T15:42:40Z · back · build → plan · Leif re-scoped the release prep (15:35Z to 15:41Z, relayed by t370): the journal fixes with checkpoint --title/--summary and journal add --record plus the backfill, lessons, and temporary-root paths in case definitions not counting as leaks; native change sent back to draft ^t-f4a04a
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: f4855ab4801f4e0684bdd3a35b82b3041c3a6be1
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, checkpoint references: 0 case(s) and 23 observation(s) to re-check with `chough status`
  - by: claude-code (hc t382)

