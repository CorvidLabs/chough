---
kind: session
id: 2026-11-12-release-0-6-0
aliases: [release-0-6-0]
objective: "Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs"
intent_scope: ""
modules: []
stage: build
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
  - 2026-11-12-tc-records-journal-post-uses-the-title-and-summary
  - 2026-11-12-tc-records-journal-post-derives-a-title-and-body
  - 2026-11-12-tc-records-journal-add-record-backfills-a-story
  - 2026-11-12-tc-records-doctor-case-temp-paths-are-not-leaks
  - 2026-11-12-tc-records-journal-and-feed
  - 2026-11-12-tc-records-journal-post-uses-the-title-and-summary-8c54e8
  - 2026-11-12-tc-records-journal-post-derives-a-title-and-body-4b516b
  - 2026-11-12-tc-records-journal-add-record-backfills-a-story-92c347
  - 2026-11-12-tc-records-doctor-case-temp-paths-are-not-leaks-f46304
  - 2026-11-12-tc-records-journal-and-feed-d0e01b
  - 2026-11-12-tc-records-journal-posts-record-time
  - 2026-11-12-tc-cli-changelog-0-6-0-names-only-what-ships
  - 2026-11-12-tc-cli-docs-name-what-cases-run-returns
  - 2026-11-12-tc-records-migration-docs-name-every-key
  - 2026-11-12-tc-cli-changelog-0-6-0-names-only-what-ships-81b73a
  - 2026-11-12-tc-cli-docs-name-what-cases-run-returns-f3b95c
  - 2026-11-12-tc-records-migration-docs-name-every-key-8f1252
  - 2026-11-12-tc-cli-next-at-plan-names-the-scope-gate
  - 2026-11-12-tc-cli-gate-summary-shows-the-scope
  - 2026-11-12-tc-history-relay-fields-are-labelled-lines
  - 2026-11-12-tc-history-direct-write-fields-are-labelled-lines
  - 2026-11-12-tc-history-story-receipts-in-time-order
  - 2026-11-12-red-rerun-of-the-timing-trace-flake-tests-timing-rs-timing
  - 2026-11-12-green-rerun-of-the-timing-trace-flake-tests-timing-rs-timing
  - 2026-11-12-red-rerun-of-the-timing-native-budget-flake-tests-timing-rs-a
  - 2026-11-12-green-rerun-of-the-timing-native-budget-flake-tests-timing-rs
  - 2026-11-12-red-rerun-of-the-verify-lane-flake-at-85dd828c-6-of-12-loaded
  - 2026-11-12-green-rerun-of-the-verify-lane-flake-at-b6fcd0e9-t38b-s
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Backfill the 69 journal posts from t38c's drafts with journal add --record; build j, k, l, d and the blank line red then green; integrate t37f's c, e, i and t38b's f, g, h; record the flake reruns
---
# Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs

## Goal

Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs

## Where it stands

- After #239 squash-merged as a2e30b8c, git diff 977bd246 origin/main printed 0 lines (checked by t370 and by t382), so origin/main was merged into leif/human-first-records with -s ours as 2852997c, whose tree equals its first parent's (git diff HEAD^1 HEAD printed 0 lines); #238 then showed 54 files, only the release-prep work; no force-push
- D2 red then green (acd2b0b0, d5381eef), the migrate preview warning red then green (0fa8cb7f, 8daaad23), the journal and case-definition temporary paths red then green (423169de, 69b60a68)

## Decisions and why

- decided: the 0.6.0 release preparation is the dogfood cycle, one small native change driven with the 0.6.0 build only, scoped by t370 (relaying Leif's plan): Cargo 0.6.0 and the dated CHANGELOG with dogfood results, D1, the CodeQL helper rename, the KNOW case links and runs, #148 red then green, and settling the four flaky-test bugs; no tag, release or merge without Leif
- Leif decided "Approve the scope of the 0.6.0 release preparation": "Yes I approve" (coordinator chat (Claude Code, hc t370), 2026-11-12T14:44Z, replying to the coordinator's summary of the 0.6.0 release-prep scope; D2 deferred by Leif's choice 'Fix after 0.6.0' at 2026-11-12T14:46Z; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood as 0xLeif
- decided: re-scoped the release prep on Leif's 'Block the tag' (2026-11-12T15:14Z): D2's REQ-cli-061 delta, the migrate preview warning (a REQ-records-009 delta) and D1's pointer clarification (a REQ-history-006 delta, since delta modules must match the affected specs cli, history and records, which own the code this change touches); SpecSync offers no command to clear no_spec_change, which approve refuses with a public contract change, so I set it false in state.json by hand (a direct lifecycle write); rehearsed approve and change check --commit in a throwaway clone with a synthetic actor first, verified
- Leif decided "Approve the re-scoped 0.6.0 release preparation": "Yes, go ahead" (coordinator chat (Claude Code, hc t370), 2026-11-12T15:24Z, answering the coordinator's plain summary of the three contract sentences (REQ-cli-061, REQ-records-009, REQ-history-006) for the re-scoped 0.6.0 release prep; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood as 0xLeif
- Leif decided "Approve the 0.6.0 release prep re-scoped for the journal, lessons and temporary paths in case definitions": "yes put the journal fixes in 0.6.0 / yes add lessons in 0.6.0 / Yes, add both (Recommended) / Yes (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-12: 'yes put the journal fixes in 0.6.0' at 15:35Z (short titles, real summaries, the backfill); 'yes add lessons in 0.6.0' at 15:37Z; at 15:41Z 'Yes, add both (Recommended)' for chough checkpoint --title/--summary and chough journal add --record with a derived fallback, and 'Yes (Recommended)' for /tmp paths in test-case definitions not counting as leaks; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood as 0xLeif
- Leif decided "Approve the 0.6.0 release prep re-scoped for the fresh-eyes findings and the migrated blank line": "Yes, fix all (Recommended) / Yes, fix it (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-12: at 16:26Z 'Yes, fix all (Recommended)' for every fresh-eyes finding (bugs and gaps c, d, e; messages f, g, h, i; requirement changes j, k, l), and at 16:38Z 'Yes, fix it (Recommended)' for the migrated session file's stray blank line; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood as 0xLeif

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
- [Ran cargo test --locked --test journal -- --exact the_post_uses_the_title_and_summary_the_agent_set for TC-records-journal-post-uses-the-title-and-summary: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-records-journal-post-uses-the-title-and-summary.md)
- [Ran cargo test --locked --test journal -- --exact without_a_title_or_summary_the_post_derives_both_from_the_story for TC-records-journal-post-derives-a-title-and-body: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-records-journal-post-derives-a-title-and-body.md)
- [Ran cargo test --locked --test journal -- --exact journal_add_record_backfills_an_archived_story for TC-records-journal-add-record-backfills-a-story: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-records-journal-add-record-backfills-a-story.md)
- [Ran cargo test --locked --test doctor -- --exact a_temporary_path_in_a_case_definition_is_not_a_leak for TC-records-doctor-case-temp-paths-are-not-leaks: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-records-doctor-case-temp-paths-are-not-leaks.md)
- [Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-records-journal-and-feed.md)
- [Ran cargo test --locked --test journal -- --exact the_post_uses_the_title_and_summary_the_agent_set for TC-records-journal-post-uses-the-title-and-summary: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-records-journal-post-uses-the-title-and-summary-8c54e8.md)
- [Ran cargo test --locked --test journal -- --exact without_a_title_or_summary_the_post_derives_both_from_the_story for TC-records-journal-post-derives-a-title-and-body: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-records-journal-post-derives-a-title-and-body-4b516b.md)
- [Ran cargo test --locked --test journal -- --exact journal_add_record_backfills_an_archived_story for TC-records-journal-add-record-backfills-a-story: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-records-journal-add-record-backfills-a-story-92c347.md)
- [Ran cargo test --locked --test doctor -- --exact a_temporary_path_in_a_case_definition_is_not_a_leak for TC-records-doctor-case-temp-paths-are-not-leaks: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-records-doctor-case-temp-paths-are-not-leaks-f46304.md)
- [Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-records-journal-and-feed-d0e01b.md)
- [Ran cargo test --locked --test journal -- --exact posts_record_a_time_and_sort_newest_first_by_it for TC-records-journal-posts-record-time: passed (exit 0).](evidence/2026-11-12-tc-records-journal-posts-record-time.md)
- [Ran cargo test --locked --test changelog -- --exact the_0_6_0_also_list_names_nothing_0_6_0_removed for TC-cli-changelog-0-6-0-names-only-what-ships: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-cli-changelog-0-6-0-names-only-what-ships.md)
- [Ran cargo test --locked --test cases -- --exact the_cli_reference_names_what_cases_run_returns for TC-cli-docs-name-what-cases-run-returns: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-cli-docs-name-what-cases-run-returns.md)
- [Ran cargo test --locked --test migrate -- --exact the_migration_docs_name_every_key_the_preview_and_apply_print for TC-records-migration-docs-name-every-key: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-records-migration-docs-name-every-key.md)
- [Ran cargo test --locked --test changelog -- --exact the_0_6_0_also_list_names_nothing_0_6_0_removed for TC-cli-changelog-0-6-0-names-only-what-ships: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-changelog-0-6-0-names-only-what-ships-81b73a.md)
- [Ran cargo test --locked --test cases -- --exact the_cli_reference_names_what_cases_run_returns for TC-cli-docs-name-what-cases-run-returns: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-cli-docs-name-what-cases-run-returns-f3b95c.md)
- [Ran cargo test --locked --test migrate -- --exact the_migration_docs_name_every_key_the_preview_and_apply_print for TC-records-migration-docs-name-every-key: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-records-migration-docs-name-every-key-8f1252.md)
- [Ran cargo test --locked --test next -- --exact at_plan_next_names_the_scope_gate_and_never_specsyncs_approve for TC-cli-next-at-plan-names-the-scope-gate: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-cli-next-at-plan-names-the-scope-gate.md)
- [Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_the_changes_delta_and_plan_first for TC-cli-gate-summary-shows-the-scope: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-cli-gate-summary-shows-the-scope.md)
- [Ran cargo test --locked --test relay -- --exact a_relayed_decision_is_written_as_labelled_lines_and_reads_back for TC-history-relay-fields-are-labelled-lines: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-history-relay-fields-are-labelled-lines.md)
- [Ran cargo test --locked --test direct_writes -- --exact a_direct_write_is_written_as_labelled_lines for TC-history-direct-write-fields-are-labelled-lines: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-history-direct-write-fields-are-labelled-lines.md)
- [Ran cargo test --locked --test journal -- --exact a_storys_receipts_are_in_observation_time_order for TC-history-story-receipts-in-time-order: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-history-story-receipts-in-time-order.md)
- [Red rerun of the timing trace flake: tests/timing.rs::timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit built with Rust 1.96.0 at 046e737c (before #172) in a throwaway worktree passed 0 of 20 under load (20 busy loops and 8 loops that each write a new executable and run it once, the macOS first-exec assessment load), 2 lanes x 10 runs, inside the heavy-tests lock, with t37f's rerun.sh and stress.sh copied unchanged](evidence/2026-11-12-red-rerun-of-the-timing-trace-flake-tests-timing-rs-timing.md)
- [Green rerun of the timing trace flake: tests/timing.rs::timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit built with Rust 1.96.0 at b6fcd0e9 (with #172 and both warm-ups) passed 20 of 20 under load (20 busy loops and 8 loops that each write a new executable and run it once, the macOS first-exec assessment load), 2 lanes x 10 runs, inside the heavy-tests lock, with t37f's rerun.sh and stress.sh copied unchanged](evidence/2026-11-12-green-rerun-of-the-timing-trace-flake-tests-timing-rs-timing.md)
- [Red rerun of the timing native-budget flake: tests/timing.rs::a_timed_out_or_unconfigured_native_read_leaves_native_times_unknown built with Rust 1.88.0 at 9c41e959 (the bug's revision) in a throwaway worktree passed 3 of 20 and failed 17 under load (20 busy loops and 8 loops that each write a new executable and run it once, the macOS first-exec assessment load), 2 lanes x 10 runs, inside the heavy-tests lock, with t37f's rerun.sh and stress.sh copied unchanged](evidence/2026-11-12-red-rerun-of-the-timing-native-budget-flake-tests-timing-rs-a.md)
- [Green rerun of the timing native-budget flake: tests/timing.rs::a_timed_out_or_unconfigured_native_read_leaves_native_times_unknown built with Rust 1.88.0 at b6fcd0e9 (this branch with t37f's warm-up 7210d0e7 and t38b's stand-in warm-up) passed 20 of 20 under load (20 busy loops and 8 loops that each write a new executable and run it once, the macOS first-exec assessment load), 2 lanes x 10 runs, inside the heavy-tests lock, with t37f's rerun.sh and stress.sh copied unchanged](evidence/2026-11-12-green-rerun-of-the-timing-native-budget-flake-tests-timing-rs.md)
- [Red rerun of the verify-lane flake at 85dd828c: 6 of 12 loaded runs failed (peak 1-minute load 80.0): cases 3 of 3 (a_descendant_holding_the_pipes_cannot_outlast_the_case_deadline and a_descendant_holding_the_pipes_after_exit_is_stopped_by_the_deadline) and stop 3 of 3 (stopping_a_case_run_while_it_saves_stops_git_and_records_nothing); t38b's flaky_lane.sh and lane_stress.py copied unchanged, under the heavy-tests lock: the cli, change, stop and cases test binaries built with Rust 1.96.0, each run 3 times at once (12 runs) beside 30 CPU busy loops and 4 writers of 32 MiB with F_FULLFSYNC](evidence/2026-11-12-red-rerun-of-the-verify-lane-flake-at-85dd828c-6-of-12-loaded.md)
- [Green rerun of the verify-lane flake at b6fcd0e9 (t38b's install-time stand-in warm-up 1888f4f5 and stub PIDs published by rename 2ee84412): 0 of 12 loaded runs failed (peak 1-minute load 93.9); t38b's flaky_lane.sh and lane_stress.py copied unchanged, under the heavy-tests lock: the cli, change, stop and cases test binaries built with Rust 1.96.0, each run 3 times at once (12 runs) beside 30 CPU busy loops and 4 writers of 32 MiB with F_FULLFSYNC](evidence/2026-11-12-green-rerun-of-the-verify-lane-flake-at-b6fcd0e9-t38b-s.md)

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
- 2026-11-12T15:46:41Z · gate-opened · plan → plan · Approve the 0.6.0 release prep re-scoped for the journal, lessons and temporary paths in case definitions ^t-2fa2d4
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 1b9548ffbc25bde92f3f611e80b5d63676033ece
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-12T15:46:41Z · gate-closed · plan → plan · Approve the 0.6.0 release prep re-scoped for the journal, lessons and temporary paths in case definitions ^t-e80901
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 1b9548ffbc25bde92f3f611e80b5d63676033ece
  - relayed_by: {"decided_by":"Leif","words":"yes put the journal fixes in 0.6.0 / yes add lessons in 0.6.0 / Yes, add both (Recommended) / Yes (Recommended)","where":"coordinator chat (Claude Code, hc t370), 2026-11-12: 'yes put the journal fixes in 0.6.0' at 15:35Z (short titles, real summaries, the backfill); 'yes add lessons in 0.6.0' at 15:37Z; at 15:41Z 'Yes, add both (Recommended)' for chough checkpoint --title/--summary and chough journal add --record with a derived fallback, and 'Yes (Recommended)' for /tmp paths in test-case definitions not counting as leaks; relayed by claude-code-t370","recorded_by":"claude-code-t382","native":"specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood","actor":"0xLeif"}
  - by: claude-code (hc t382)
- 2026-11-12T15:46:53Z · advance · plan → build · Leif approved the re-scope (journal, lessons, temporary paths in case definitions; relayed by t370, 15:35Z to 15:41Z); build the journal and doctor fixes red then green, then the backfill and lessons ^t-75459d
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - gate: native:approved
  - revision: 1b9548ffbc25bde92f3f611e80b5d63676033ece
  - by: claude-code (hc t382)
- 2026-11-12T16:37:15Z · direct-native-write · build → build ·  ^t-af7e4b
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: ad3214229cc6513156f661fbe960eaed929765cb
  - direct_write: {"change":"prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"approved","after_state":"draft"}
  - by: claude-code (hc t382)
- 2026-11-12T16:37:15Z · back · build → plan · Leif chose 'Yes, fix all (Recommended)' for the fresh-eyes findings (16:26Z, relayed by t370); native change sent back to draft for their contract text ^t-b19454
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: ad3214229cc6513156f661fbe960eaed929765cb
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, checkpoint references: 0 case(s) and 40 observation(s) to re-check with `chough status`
  - by: claude-code (hc t382)
- 2026-11-12T16:39:04Z · gate-opened · plan → plan · Approve the 0.6.0 release prep re-scoped for the fresh-eyes findings and the migrated blank line ^t-0ea94d
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 3a5dc792021065b81fbe8853199f4916afc8c069
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-12T16:39:05Z · gate-closed · plan → plan · Approve the 0.6.0 release prep re-scoped for the fresh-eyes findings and the migrated blank line ^t-92acd1
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 3a5dc792021065b81fbe8853199f4916afc8c069
  - relayed_by: {"decided_by":"Leif","words":"Yes, fix all (Recommended) / Yes, fix it (Recommended)","where":"coordinator chat (Claude Code, hc t370), 2026-11-12: at 16:26Z 'Yes, fix all (Recommended)' for every fresh-eyes finding (bugs and gaps c, d, e; messages f, g, h, i; requirement changes j, k, l), and at 16:38Z 'Yes, fix it (Recommended)' for the migrated session file's stray blank line; relayed by claude-code-t370","recorded_by":"claude-code-t382","native":"specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood","actor":"0xLeif"}
  - by: claude-code (hc t382)
- 2026-11-12T16:39:10Z · advance · plan → build · Leif approved the fresh-eyes re-scope (16:26Z, 16:38Z, relayed by t370) ^t-66baf7
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - gate: native:approved
  - revision: 3a5dc792021065b81fbe8853199f4916afc8c069
  - by: claude-code (hc t382)

