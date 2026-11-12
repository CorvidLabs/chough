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
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Integrate the four flake fixes from t37f (timing) and t38b (lock timeout, verify lane): re-run each red and green, record bugs verified; then change check --commit and Leif's review via chough change ship --close-gate"
---
# Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs

## Goal

Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs

## Where it stands

## Decisions and why

- decided: the 0.6.0 release preparation is the dogfood cycle, one small native change driven with the 0.6.0 build only, scoped by t370 (relaying Leif's plan): Cargo 0.6.0 and the dated CHANGELOG with dogfood results, D1, the CodeQL helper rename, the KNOW case links and runs, #148 red then green, and settling the four flaky-test bugs; no tag, release or merge without Leif
- Leif decided "Approve the scope of the 0.6.0 release preparation": "Yes I approve" (coordinator chat (Claude Code, hc t370), 2026-11-12T14:44Z, replying to the coordinator's summary of the 0.6.0 release-prep scope; D2 deferred by Leif's choice 'Fix after 0.6.0' at 2026-11-12T14:46Z; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood as 0xLeif

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

