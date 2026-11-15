---
kind: session
id: 2026-11-15-fix-289-index-at-release
aliases: [fix-289-index-at-release]
objective: "Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml"
intent_scope: KNOW
modules: [cli]
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
pr: ""
criteria: [KNOW-4]
requirements: []
cases: []
design: []
files: []
observations:
  - 2026-11-15-tc-cli-documented-merge-drivers-match
  - 2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed
  - 2026-11-15-tc-cli-knowledge-index-check
  - 2026-11-15-tc-cli-librarian-default-on-knowledge
  - 2026-11-15-tc-cli-librarian-off-no-knowledge-at-done
  - 2026-11-15-tc-cli-generated-knowledge-merge
  - 2026-11-15-tc-cli-knowledge-rebuild-keeps-observation-current
  - 2026-11-15-tc-cli-advance-into-done-one-read
  - 2026-11-15-tc-cli-readme-verify-list-matches-fledge
  - 2026-11-15-tc-cli-native-workflow-bundle-consistency
  - 2026-11-15-tc-cli-integrations-digest-detects-change
  - 2026-11-15-tc-cli-build-reference-from-main-known-outdated
  - 2026-11-15-tc-cli-vault-regenerates-identically
  - 2026-11-15-tc-records-journal-and-feed
  - 2026-11-15-tc-records-done-touches-only-its-own-post
  - 2026-11-15-tc-history-session-archives-itself-at-done
  - 2026-11-15-tc-history-finished-session-archived-without-native-read
  - 2026-11-15-tc-cli-documented-merge-drivers-match-0dd04e
  - 2026-11-15-tc-cli-doc-links-resolve
  - 2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed-5c2986
  - 2026-11-15-tc-records-done-touches-only-its-own-post-e4268d
  - 2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed-b4eb41
  - 2026-11-15-tc-records-done-touches-only-its-own-post-1be45e
  - 2026-11-15-at-26bf0f5-no-step-of-the-ci-or-verify-lane-runs-library-index
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Gate \"Re-approve the rebased deltas of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289\" closed (approved); run `chough stage advance` to move on."
---
# Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml

## Goal

Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289": "yes I approve" (Claude Code chat (hc t28), 2026-11-15, on the #289 scope summary; list refreshed only for Flint's two announced tests (7321283) and a re-run of TC-cli-documented-merge-drivers-match); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 as 0xLeif
- 0xLeif decided "Re-approve the rebased deltas of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289": "yes I re-approve the #289 deltas" (Claude Code chat (hc t28), 2026-11-15, on the rebased REQ-history-006 and REQ-records-011 deltas; native approval recorded with specsync change approve); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 as 0xLeif

## What was verified

- [Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-documented-merge-drivers-match.md)
- [Ran cargo test --locked --test librarian_trim -- --exact done_refreshes_the_knowledge_index_when_nothing_is_trimmed for TC-cli-done-refreshes-knowledge-untrimmed: not-run (exit 0, tests run 0).](evidence/2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed.md)
- [Ran cargo test --locked --test vault -- --exact index_check_names_stale_files_and_passes_after_a_rebuild for TC-cli-knowledge-index-check: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-knowledge-index-check.md)
- [Ran cargo test --locked --test optional -- --exact the_librarian_is_on_by_default_in_knowledge_and_can_be_turned_off for TC-cli-librarian-default-on-knowledge: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-librarian-default-on-knowledge.md)
- [Ran cargo test --locked --test librarian_trim -- --exact a_librarian_off_project_gets_no_knowledge_directory_at_done for TC-cli-librarian-off-no-knowledge-at-done: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-librarian-off-no-knowledge-at-done.md)
- [Ran cargo test --locked --test journal -- two_branches_that_each_add_a_post_merge_clean_with_the_generated_driver library_merge_keeps_ours_and_refuses_a_missing_file for TC-cli-generated-knowledge-merge: passed (exit 0).](evidence/2026-11-15-tc-cli-generated-knowledge-merge.md)
- [Ran cargo test --locked --test evidence -- --exact rebuilding_the_knowledge_index_keeps_the_observation_current for TC-cli-knowledge-rebuild-keeps-observation-current: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-knowledge-rebuild-keeps-observation-current.md)
- [Ran cargo test --locked --test advance_to -- --exact advance_into_done_reads_native_status_once for TC-cli-advance-into-done-one-read: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-advance-into-done-one-read.md)
- [Ran cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step for TC-cli-readme-verify-list-matches-fledge: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-readme-verify-list-matches-fledge.md)
- [Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-native-workflow-bundle-consistency.md)
- [Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-integrations-digest-detects-change.md)
- [Ran cargo test --locked --test cli -- --exact build_reference_from_main_4daf861_is_known_outdated for TC-cli-build-reference-from-main-known-outdated: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-build-reference-from-main-known-outdated.md)
- [Ran cargo test --locked --test vault -- --exact the_vault_regenerates_identically for TC-cli-vault-regenerates-identically: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-vault-regenerates-identically.md)
- [Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-records-journal-and-feed.md)
- [Ran cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed for TC-records-done-touches-only-its-own-post: not-run (exit 0, tests run 0).](evidence/2026-11-15-tc-records-done-touches-only-its-own-post.md)
- [Ran cargo test --locked --test reduce -- the_move_into_done_archives_the_session_and_clears_the_pointer the_move_into_done_keeps_a_session_with_unfolded_inbox_entries_open a_session_with_release_authorized_archives_itself_after_release for TC-history-session-archives-itself-at-done: passed (exit 0).](evidence/2026-11-15-tc-history-session-archives-itself-at-done.md)
- [Ran cargo test --locked --test sessions -- --exact a_session_whose_native_change_is_archived_is_finished_without_a_native_read for TC-history-finished-session-archived-without-native-read: passed (exit 0).](evidence/2026-11-15-tc-history-finished-session-archived-without-native-read.md)
- [Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-documented-merge-drivers-match-0dd04e.md)
- [Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-doc-links-resolve.md)
- [Ran cargo test --locked --test librarian_trim -- --exact done_refreshes_the_knowledge_index_when_nothing_is_trimmed for TC-cli-done-refreshes-knowledge-untrimmed: not-run (exit 0, tests run 0).](evidence/2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed-5c2986.md)
- [Ran cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed for TC-records-done-touches-only-its-own-post: not-run (exit 0, tests run 0).](evidence/2026-11-15-tc-records-done-touches-only-its-own-post-e4268d.md)
- [Ran cargo test --locked --test librarian_trim -- --exact done_defers_the_knowledge_index_refresh_when_nothing_is_trimmed for TC-cli-done-refreshes-knowledge-untrimmed: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed-b4eb41.md)
- [Ran cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post for TC-records-done-touches-only-its-own-post: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-records-done-touches-only-its-own-post-1be45e.md)
- [At 26bf0f5: no step of the ci or verify lane runs library index; release-check rebuilds then runs the strict --check (and passes on this tree); a hand-edited lesson count with its link kept fails --check naming knowledge/index.md. Full suite 762 passed; fmt and lint clean.](evidence/2026-11-15-at-26bf0f5-no-step-of-the-ci-or-verify-lane-runs-library-index.md)

## Open threads

## Timeline

- 2026-11-15T18:23:57Z · start · intent → intent · Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml ^t-133098
  - revision: 4d3c1a5cfddc9e6710be2dad1c39162816adacf0
  - by: claude-code (hc t28)
- 2026-11-15T18:23:57Z · skip · intent → design · Intent is #289 and Leif's choice in chat of design B ("B"): the index is rebuilt and checked only at release; KNOW-4 is unchanged ^t-7aba5e
  - revision: 4d3c1a5cfddc9e6710be2dad1c39162816adacf0
  - by: claude-code (hc t28)
- 2026-11-15T18:23:57Z · skip · design → plan · No visual design: a lane, a release step and documentation ^t-325e4d
  - revision: 4d3c1a5cfddc9e6710be2dad1c39162816adacf0
  - by: claude-code (hc t28)
- 2026-11-15T18:24:54Z · gate-opened · plan → plan · Approve the scope of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 ^t-9f612b
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - revision: c08afc2ad5ce36675fe01bf5640a5322e17f31ba
  - holds: plan
  - drives: approve
  - by: claude-code (hc t28)
- 2026-11-15T20:18:20Z · direct-native-write · plan → plan · the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-3477ca
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - revision: 7321283c6a3194154feab7d635fd2aea08b23efd
  - direct_write:
    - change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: claude-code (hc t28)
- 2026-11-15T20:18:20Z · gate-closed · plan → plan · Approve the scope of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 ^t-26e2ca
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - revision: 7321283c6a3194154feab7d635fd2aea08b23efd
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes I approve
    - where: Claude Code chat (hc t28), 2026-11-15, on the #289 scope summary; list refreshed only for Flint's two announced tests (7321283) and a re-run of TC-cli-documented-merge-drivers-match
    - recorded_by: claude-code (hc t28)
    - summary_digest: ac908fdb93a42f40b3fb98e9807339189dbcfa45b53b6c760fcaf8fdf04771b8
    - list: faa979714b50
    - native: specsync change approve the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
    - actor: 0xLeif
  - by: claude-code (hc t28)
- 2026-11-15T20:21:30Z · advance · plan → build · native:approved ^t-841e80
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - gate: native:approved
  - revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
  - by: claude-code (hc t28)
- 2026-11-15T20:21:34Z · advance · build → test · native:approved ^t-a11394
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - gate: native:approved
  - revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
  - by: claude-code (hc t28)
- 2026-11-15T20:23:21Z · advance · test → accept · native:approved ^t-2c8d0d
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - gate: native:approved
  - revision: b47ef247957e3f59d3a9e50fa3e4f4b2cb1197cd
  - by: claude-code (hc t28)
- 2026-11-15T20:30:22Z · gate-opened · accept → accept · Re-approve the rebased deltas of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 ^t-e96222
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - revision: 128f5478baf0d6150d02a8d55ec1130ef7acea76
  - holds: accept
  - drives: approve
  - by: claude-code (hc t28)
- 2026-11-15T20:39:35Z · direct-native-write · accept → accept · the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state stays verifying ^t-dbaf99
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - revision: 128f5478baf0d6150d02a8d55ec1130ef7acea76
  - direct_write:
    - change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: verifying
  - by: claude-code (hc t28)
- 2026-11-15T20:39:35Z · gate-closed · accept → accept · Re-approve the rebased deltas of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 ^t-c80dde
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - revision: 128f5478baf0d6150d02a8d55ec1130ef7acea76
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes I re-approve the #289 deltas
    - where: Claude Code chat (hc t28), 2026-11-15, on the rebased REQ-history-006 and REQ-records-011 deltas; native approval recorded with specsync change approve
    - recorded_by: claude-code (hc t28)
    - native: specsync change approve the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
    - actor: 0xLeif
  - by: claude-code (hc t28)
