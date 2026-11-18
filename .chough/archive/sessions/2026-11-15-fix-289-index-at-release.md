---
kind: session
id: 2026-11-15-fix-289-index-at-release
aliases: [fix-289-index-at-release]
objective: "Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml"
intent_scope: KNOW
modules: [cli]
stage: done
stage_status: archived
iteration: 1
native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
criteria: [KNOW-4]
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
  - 2026-11-15-tc-cli-readme-verify-list-matches-fledge-82659a
  - 2026-11-15-tc-cli-native-workflow-bundle-consistency-9d9153
  - 2026-11-15-tc-cli-integrations-digest-detects-change-ce76cd
  - 2026-11-15-tc-cli-doc-links-resolve-a61080
  - 2026-11-15-tc-cli-documented-merge-drivers-match-c89cb2
  - 2026-11-15-tc-cli-knowledge-index-check-dc265b
  - 2026-11-15-tc-cli-build-reference-from-main-known-outdated-9cab66
closed: 2026-11-15
outcome: done
---
# Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml

## Goal

Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289": "yes I approve" (Claude Code chat (hc t28), 2026-11-15, on the #289 scope summary; list refreshed only for Flint's two announced tests (7321283) and a re-run of TC-cli-documented-merge-drivers-match); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 as 0xLeif
- 0xLeif decided "Re-approve the rebased deltas of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289": "yes I re-approve the #289 deltas" (Claude Code chat (hc t28), 2026-11-15, on the rebased REQ-history-006 and REQ-records-011 deltas; native approval recorded with specsync change approve); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 as 0xLeif
- 0xLeif decided "Implementation review of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289": "yes I accept #289" (Claude Code chat (hc t28), 2026-11-15, on the #289 implementation review (gate list ccf24edda81a, PR #297)); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change review the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 as 0xLeif with verdict pass

## What was verified

- 12 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test optional -- --exact the_librarian_is_on_by_default_in_knowledge_and_can_be_turned_off for TC-cli-librarian-default-on-knowledge: passed (exit 0, tests run 1). — ccf1c86c0a14 ^2026-11-15-tc-cli-librarian-default-on-knowledge
- Ran cargo test --locked --test librarian_trim -- --exact a_librarian_off_project_gets_no_knowledge_directory_at_done for TC-cli-librarian-off-no-knowledge-at-done: passed (exit 0, tests run 1). — 305bc353ffa3 ^2026-11-15-tc-cli-librarian-off-no-knowledge-at-done
- Ran cargo test --locked --test journal -- two_branches_that_each_add_a_post_merge_clean_with_the_generated_driver library_merge_keeps_ours_and_refuses_a_missing_file for TC-cli-generated-knowledge-merge: passed (exit 0). — cb0ad0dddc68 ^2026-11-15-tc-cli-generated-knowledge-merge
- Ran cargo test --locked --test evidence -- --exact rebuilding_the_knowledge_index_keeps_the_observation_current for TC-cli-knowledge-rebuild-keeps-observation-current: passed (exit 0, tests run 1). — 555401568484 ^2026-11-15-tc-cli-knowledge-rebuild-keeps-observation-current
- Ran cargo test --locked --test advance_to -- --exact advance_into_done_reads_native_status_once for TC-cli-advance-into-done-one-read: passed (exit 0, tests run 1). — 03942a8771d6 ^2026-11-15-tc-cli-advance-into-done-one-read
- Ran cargo test --locked --test vault -- --exact the_vault_regenerates_identically for TC-cli-vault-regenerates-identically: passed (exit 0, tests run 1). — 809e0849afb9 ^2026-11-15-tc-cli-vault-regenerates-identically
- Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: passed (exit 0, tests run 1). — 82e095634084 ^2026-11-15-tc-records-journal-and-feed
- Ran cargo test --locked --test reduce -- the_move_into_done_archives_the_session_and_clears_the_pointer the_move_into_done_keeps_a_session_with_unfolded_inbox_entries_open a_session_with_release_authorized_archives_itself_after_release for TC-history-session-archives-itself-at-done: passed (exit 0). — 4373ccc3617e ^2026-11-15-tc-history-session-archives-itself-at-done
- Ran cargo test --locked --test sessions -- --exact a_session_whose_native_change_is_archived_is_finished_without_a_native_read for TC-history-finished-session-archived-without-native-read: passed (exit 0). — 977737c896b4 ^2026-11-15-tc-history-finished-session-archived-without-native-read
- Ran cargo test --locked --test librarian_trim -- --exact done_defers_the_knowledge_index_refresh_when_nothing_is_trimmed for TC-cli-done-refreshes-knowledge-untrimmed: passed (exit 0, tests run 1). — d5ff80c5c944 ^2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed-b4eb41
- Ran cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post for TC-records-done-touches-only-its-own-post: passed (exit 0, tests run 1). — cae1115c001b ^2026-11-15-tc-records-done-touches-only-its-own-post-1be45e
- At 26bf0f5: no step of the ci or verify lane runs library index; release-check rebuilds then runs the strict --check (and passes on this tree); a hand-edited lesson count with its link kept fails --check naming knowledge/index.md. Full suite 762 passed; fmt and lint clean. — 244b2d5e93ce ^2026-11-15-at-26bf0f5-no-step-of-the-ci-or-verify-lane-runs-library-index
- Ran cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step for TC-cli-readme-verify-list-matches-fledge: passed (exit 0, tests run 1). — d40c39ada86f ^2026-11-15-tc-cli-readme-verify-list-matches-fledge-82659a
- Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1). — 6a2630ff8aad ^2026-11-15-tc-cli-native-workflow-bundle-consistency-9d9153
- Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1). — 02625857e36b ^2026-11-15-tc-cli-integrations-digest-detects-change-ce76cd
- Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1). — f553635b0743 ^2026-11-15-tc-cli-doc-links-resolve-a61080
- Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1). — 8fdca87a7de5 ^2026-11-15-tc-cli-documented-merge-drivers-match-c89cb2
- Ran cargo test --locked --test vault -- --exact index_check_names_stale_files_and_passes_after_a_rebuild for TC-cli-knowledge-index-check: passed (exit 0, tests run 1). — dcec9482844b ^2026-11-15-tc-cli-knowledge-index-check-dc265b
- Ran cargo test --locked --test cli -- --exact build_reference_from_main_4daf861_is_known_outdated for TC-cli-build-reference-from-main-known-outdated: passed (exit 0, tests run 1). — 6968532466df ^2026-11-15-tc-cli-build-reference-from-main-known-outdated-9cab66

## Bugs

None.

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
- 2026-11-15T21:15:33Z · gate-opened · accept → accept · Implementation review of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 ^t-d7ec4b
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - revision: 2952b350a68e55c2d9f7221533e5c97d72586b37
  - holds: accept
  - drives: review
  - by: claude-code (hc t28)
- 2026-11-15T21:20:23Z · gate-closed · accept → accept · Implementation review of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 ^t-77bb34
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - revision: ff99cda00d0cb8b5ef5128d99657b49637eed31b
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes I accept #289
    - where: Claude Code chat (hc t28), 2026-11-15, on the #289 implementation review (gate list ccf24edda81a, PR #297)
    - recorded_by: claude-code (hc t28)
    - native: specsync change review the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
    - actor: 0xLeif
  - by: claude-code (hc t28)
- 2026-11-15T21:20:24Z · advance · accept → done · native:archived ^t-fd1da2
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - gate: native:archived
  - revision: ff99cda00d0cb8b5ef5128d99657b49637eed31b
  - by: claude-code (hc t28)

Closed 2026-11-15: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-15-tc-cli-documented-merge-drivers-match
    claim: "Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares
    revision: 7321283c6a3194154feab7d635fd2aea08b23efd
    environment: local-macos-arm64
    content: 88fda8b77b56860cfdb57daff6b1b2b61c6c6e8a3c3c330671846a2a480d6967
    fingerprint: sha256:cf5c65f9426a04cba95a78fd6870ea4602845c43aeeec69741415dd402d8e132
    case: TC-cli-documented-merge-drivers-match
    case_digest: be40dc133895dbd3617da6f180952bf1aa526c18eb9e7656afda673040129881
    observed: 2026-11-15T20:18:11Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed
    claim: "Ran cargo test --locked --test librarian_trim -- --exact done_refreshes_the_knowledge_index_when_nothing_is_trimmed for TC-cli-done-refreshes-knowledge-untrimmed: not-run (exit 0, tests run 0)."
    result: not-run
    command: cargo test --locked --test librarian_trim -- --exact done_refreshes_the_knowledge_index_when_nothing_is_trimmed
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:db63d6872f3e8c11f6a27eb5dc14c93566e77d6805b076f897d06e8a0296032c
    case: TC-cli-done-refreshes-knowledge-untrimmed
    case_digest: dc137cbd0ff74dfce469363e89061612bbfd028a9802935c94cfff77148e9ecf
    observed: 2026-11-15T20:22:03Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-knowledge-index-check
    claim: "Ran cargo test --locked --test vault -- --exact index_check_names_stale_files_and_passes_after_a_rebuild for TC-cli-knowledge-index-check: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test vault -- --exact index_check_names_stale_files_and_passes_after_a_rebuild
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:994617393d3485bbece3f12af78004c985935c80865ad81f2300dc0f2f8576b9
    case: TC-cli-knowledge-index-check
    case_digest: 9d8e03aa6c11246bac9437a73345439438d54d93a358854132ac25fb362ea7cf
    observed: 2026-11-15T20:22:04Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-librarian-default-on-knowledge
    claim: "Ran cargo test --locked --test optional -- --exact the_librarian_is_on_by_default_in_knowledge_and_can_be_turned_off for TC-cli-librarian-default-on-knowledge: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test optional -- --exact the_librarian_is_on_by_default_in_knowledge_and_can_be_turned_off
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:ccf1c86c0a149788b17179cb0b49ad3097eb08ff8e581c88e9b2c57196856bf3
    case: TC-cli-librarian-default-on-knowledge
    case_digest: d31d4fec5a25ef0a00679b5da036f2d993503f705b04b830c18fef004737c00b
    observed: 2026-11-15T20:22:05Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-librarian-off-no-knowledge-at-done
    claim: "Ran cargo test --locked --test librarian_trim -- --exact a_librarian_off_project_gets_no_knowledge_directory_at_done for TC-cli-librarian-off-no-knowledge-at-done: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test librarian_trim -- --exact a_librarian_off_project_gets_no_knowledge_directory_at_done
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:305bc353ffa3f3828df59d8f2bec92ff4114201d42655424e1de628eea9abb29
    case: TC-cli-librarian-off-no-knowledge-at-done
    case_digest: cdb85ba6754f0ab0b40223e69a8c407febb1cfb59053209b6b432527c81f05fc
    observed: 2026-11-15T20:22:06Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-generated-knowledge-merge
    claim: "Ran cargo test --locked --test journal -- two_branches_that_each_add_a_post_merge_clean_with_the_generated_driver library_merge_keeps_ours_and_refuses_a_missing_file for TC-cli-generated-knowledge-merge: passed (exit 0)."
    result: passed
    command: cargo test --locked --test journal -- two_branches_that_each_add_a_post_merge_clean_with_the_generated_driver library_merge_keeps_ours_and_refuses_a_missing_file
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:cb0ad0dddc685e69462f979ed1b9bf2a580591cca7124f25f1284dc15f31dcc9
    case: TC-cli-generated-knowledge-merge
    case_digest: 0346e0936854daca606069c14dd376226479d780570cd92c5d1aaae210d041b7
    observed: 2026-11-15T20:22:07Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-knowledge-rebuild-keeps-observation-current
    claim: "Ran cargo test --locked --test evidence -- --exact rebuilding_the_knowledge_index_keeps_the_observation_current for TC-cli-knowledge-rebuild-keeps-observation-current: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test evidence -- --exact rebuilding_the_knowledge_index_keeps_the_observation_current
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:55540156848438f448de1a9dd959b5addfbcdc165acab10f11b9edd6ff38d188
    case: TC-cli-knowledge-rebuild-keeps-observation-current
    case_digest: 1a8cf4ac50403134723c7756b10d141bf8cbbac4d4351a16db275eb5f6ff9706
    observed: 2026-11-15T20:22:08Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-advance-into-done-one-read
    claim: "Ran cargo test --locked --test advance_to -- --exact advance_into_done_reads_native_status_once for TC-cli-advance-into-done-one-read: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test advance_to -- --exact advance_into_done_reads_native_status_once
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:03942a8771d682838269c613b74eeaa890a5769a06532181cfbb9983db9ff81c
    case: TC-cli-advance-into-done-one-read
    case_digest: 22cae0f9f046256865ba5ff8c24d78c1362420b7b22d9420c6d462c44c4530d3
    observed: 2026-11-15T20:22:09Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-native-workflow-bundle-consistency
    claim: "Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:9c731137b83c3ab5c46f176c15f7626bed358a76edb92a071be574ffc4e37f15
    case: TC-cli-native-workflow-bundle-consistency
    case_digest: 519ac533c4882bb4da71e97a9cae2d9ec1c2a93425652f94f90ec555939a8e72
    observed: 2026-11-15T20:22:10Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-readme-verify-list-matches-fledge
    claim: "Ran cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step for TC-cli-readme-verify-list-matches-fledge: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:049cdb6b7cf53281fcd3db7916bd1f8538eba43376ee6d8a94e927bf741db0bb
    case: TC-cli-readme-verify-list-matches-fledge
    case_digest: 8053fe1bd76560e758c7d613849c909d72c620944f864dc5d9bfa8e51fdae130
    observed: 2026-11-15T20:22:10Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-build-reference-from-main-known-outdated
    claim: "Ran cargo test --locked --test cli -- --exact build_reference_from_main_4daf861_is_known_outdated for TC-cli-build-reference-from-main-known-outdated: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact build_reference_from_main_4daf861_is_known_outdated
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:08d19a50f94085945fc0c4c2a098355786c90d65a3b1dc9b0d8ee3cd96edda44
    case: TC-cli-build-reference-from-main-known-outdated
    case_digest: e87e897bdf1ab06ffab9894326cd28ecb9b9faed7c463e3dc6c369087343f52f
    observed: 2026-11-15T20:22:11Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-integrations-digest-detects-change
    claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:02625857e36bfe9c700886291a3baef511c8af5f7d1b958682f3481b37efcb21
    case: TC-cli-integrations-digest-detects-change
    case_digest: aadc547832dd06247ad6e92c9b62442095049f56faf523a6ecc0c319c57cb466
    observed: 2026-11-15T20:22:11Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-vault-regenerates-identically
    claim: "Ran cargo test --locked --test vault -- --exact the_vault_regenerates_identically for TC-cli-vault-regenerates-identically: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test vault -- --exact the_vault_regenerates_identically
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:809e0849afb9283b286b73ccb4606ccde5ab446af7054aa59c15f9efff5410f8
    case: TC-cli-vault-regenerates-identically
    case_digest: ece2e4307cf72330cf0f9cc3aba08a82d2b687bf75be00165c37433dd4134360
    observed: 2026-11-15T20:22:12Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-records-journal-and-feed
    claim: "Ran cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it for TC-records-journal-and-feed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact done_writes_a_short_post_and_the_feed_lists_it
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:82e0956340845387c0abe3084caa641a844d5db7035f458a9ce786ad55886c63
    case: TC-records-journal-and-feed
    case_digest: e841dbd34c7ad1f50eea02debd501bf9ef7e39e64f6b75256a5c650edec731c2
    observed: 2026-11-15T20:22:13Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-records-done-touches-only-its-own-post
    claim: "Ran cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed for TC-records-done-touches-only-its-own-post: not-run (exit 0, tests run 0)."
    result: not-run
    command: cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:c0836d3345fdef0b5827d198386deb9cb8443b950a39ed445c8bd5daf3eae898
    case: TC-records-done-touches-only-its-own-post
    case_digest: 356b2d6b5332add74202731b95dcb54c533480dea4f4dc51b3e2ec86691b22af
    observed: 2026-11-15T20:22:14Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-history-session-archives-itself-at-done
    claim: "Ran cargo test --locked --test reduce -- the_move_into_done_archives_the_session_and_clears_the_pointer the_move_into_done_keeps_a_session_with_unfolded_inbox_entries_open a_session_with_release_authorized_archives_itself_after_release for TC-history-session-archives-itself-at-done: passed (exit 0)."
    result: passed
    command: cargo test --locked --test reduce -- the_move_into_done_archives_the_session_and_clears_the_pointer the_move_into_done_keeps_a_session_with_unfolded_inbox_entries_open a_session_with_release_authorized_archives_itself_after_release
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:4373ccc3617efbbd84e544cd5676c12bf8da1ad6d573b700d321323be3651284
    case: TC-history-session-archives-itself-at-done
    case_digest: 0f8db0da0138d44c7dee4543239712e3cc4f6795e8f4a33b80c5cad97819aa54
    observed: 2026-11-15T20:22:15Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-documented-merge-drivers-match-0dd04e
    claim: "Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:cf5c65f9426a04cba95a78fd6870ea4602845c43aeeec69741415dd402d8e132
    case: TC-cli-documented-merge-drivers-match
    case_digest: be40dc133895dbd3617da6f180952bf1aa526c18eb9e7656afda673040129881
    observed: 2026-11-15T20:22:16Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-history-finished-session-archived-without-native-read
    claim: "Ran cargo test --locked --test sessions -- --exact a_session_whose_native_change_is_archived_is_finished_without_a_native_read for TC-history-finished-session-archived-without-native-read: passed (exit 0)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_session_whose_native_change_is_archived_is_finished_without_a_native_read
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:977737c896b407664d08f0cf51f91d92a3e6235be10533aaf3182924fad3ab78
    case: TC-history-finished-session-archived-without-native-read
    case_digest: 513ca3bcffc73a6398d153bd10dc6e8d0f3e28b2d589a689baf27615ecbbe32a
    observed: 2026-11-15T20:22:16Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-doc-links-resolve
    claim: "Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:bb4942c1e5abe34c4a37b140267c582a9c70678bec7d4ccd5089358e1ae72b5a
    case: TC-cli-doc-links-resolve
    case_digest: bc8019ca3cf835f594fa494d827d7c34b8f2e9a77993c9cf1c379afc62c4fef5
    observed: 2026-11-15T20:22:17Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed-5c2986
    claim: "Ran cargo test --locked --test librarian_trim -- --exact done_refreshes_the_knowledge_index_when_nothing_is_trimmed for TC-cli-done-refreshes-knowledge-untrimmed: not-run (exit 0, tests run 0)."
    result: not-run
    command: cargo test --locked --test librarian_trim -- --exact done_refreshes_the_knowledge_index_when_nothing_is_trimmed
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:f7fcea1e017cf0665f4a7f4ddea8246516dcc1879077371f416c6f2cc9c9f687
    case: TC-cli-done-refreshes-knowledge-untrimmed
    case_digest: dc137cbd0ff74dfce469363e89061612bbfd028a9802935c94cfff77148e9ecf
    observed: 2026-11-15T20:22:20Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-records-done-touches-only-its-own-post-e4268d
    claim: "Ran cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed for TC-records-done-touches-only-its-own-post: not-run (exit 0, tests run 0)."
    result: not-run
    command: cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post_and_the_feed
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:c0836d3345fdef0b5827d198386deb9cb8443b950a39ed445c8bd5daf3eae898
    case: TC-records-done-touches-only-its-own-post
    case_digest: 356b2d6b5332add74202731b95dcb54c533480dea4f4dc51b3e2ec86691b22af
    observed: 2026-11-15T20:22:21Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-done-refreshes-knowledge-untrimmed-b4eb41
    claim: "Ran cargo test --locked --test librarian_trim -- --exact done_defers_the_knowledge_index_refresh_when_nothing_is_trimmed for TC-cli-done-refreshes-knowledge-untrimmed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test librarian_trim -- --exact done_defers_the_knowledge_index_refresh_when_nothing_is_trimmed
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:d5ff80c5c944fd04edada23296150ae59647b3b32ec71211ca9a4830e795fda7
    case: TC-cli-done-refreshes-knowledge-untrimmed
    case_digest: b7ad95278fb3adca8592e60b1beae40fb60a70a81abd9026e590ffbd389d6ca3
    observed: 2026-11-15T20:22:43Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-records-done-touches-only-its-own-post-1be45e
    claim: "Ran cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post for TC-records-done-touches-only-its-own-post: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact done_changes_no_knowledge_file_but_its_own_post
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:cae1115c001b4f7295d2649c636f47d79eb52cb5e09852e5fd03ccf40d8be2bc
    case: TC-records-done-touches-only-its-own-post
    case_digest: e2ff2bda6f7f2ce508865a955c4b2de1dd33ad1596cf2c7c0b3bdb4222b0d8b8
    observed: 2026-11-15T20:22:44Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-at-26bf0f5-no-step-of-the-ci-or-verify-lane-runs-library-index
    claim: "At 26bf0f5: no step of the ci or verify lane runs library index; release-check rebuilds then runs the strict --check (and passes on this tree); a hand-edited lesson count with its link kept fails --check naming knowledge/index.md. Full suite 762 passed; fmt and lint clean."
    result: passed
    revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
    environment: local-macos-arm64
    content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
    fingerprint: sha256:244b2d5e93ce083992aa3e0b0d3446623f465e4f83bc2178e7bf35b9c12f4aa7
    observed: 2026-11-15T20:23:10Z
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-native-workflow-bundle-consistency-9d9153
    claim: "Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent
    revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
    environment: local-macos-arm64
    content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
    fingerprint: sha256:6a2630ff8aad00762fc16c8573a5129433f5253fa75dc4337fd2462ad242ee2f
    case: TC-cli-native-workflow-bundle-consistency
    case_digest: 519ac533c4882bb4da71e97a9cae2d9ec1c2a93425652f94f90ec555939a8e72
    observed: 2026-11-15T21:09:06Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-readme-verify-list-matches-fledge-82659a
    claim: "Ran cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step for TC-cli-readme-verify-list-matches-fledge: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step
    revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
    environment: local-macos-arm64
    content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
    fingerprint: sha256:d40c39ada86f3bb4c91080aedef5b0fe1ae6ffe54cd8ea11ff69f0807a984706
    case: TC-cli-readme-verify-list-matches-fledge
    case_digest: 8053fe1bd76560e758c7d613849c909d72c620944f864dc5d9bfa8e51fdae130
    observed: 2026-11-15T21:09:06Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-doc-links-resolve-a61080
    claim: "Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve
    revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
    environment: local-macos-arm64
    content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
    fingerprint: sha256:f553635b07439395390b28dfc44172e6e00029514e59db033c231e83f842b645
    case: TC-cli-doc-links-resolve
    case_digest: bc8019ca3cf835f594fa494d827d7c34b8f2e9a77993c9cf1c379afc62c4fef5
    observed: 2026-11-15T21:09:07Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-integrations-digest-detects-change-ce76cd
    claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest
    revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
    environment: local-macos-arm64
    content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
    fingerprint: sha256:02625857e36bfe9c700886291a3baef511c8af5f7d1b958682f3481b37efcb21
    case: TC-cli-integrations-digest-detects-change
    case_digest: aadc547832dd06247ad6e92c9b62442095049f56faf523a6ecc0c319c57cb466
    observed: 2026-11-15T21:09:07Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-documented-merge-drivers-match-c89cb2
    claim: "Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares
    revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
    environment: local-macos-arm64
    content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
    fingerprint: sha256:8fdca87a7de51458a4a15b3c70aac5773bd7ed5d39f09e0b3f0c8a31807e02cf
    case: TC-cli-documented-merge-drivers-match
    case_digest: be40dc133895dbd3617da6f180952bf1aa526c18eb9e7656afda673040129881
    observed: 2026-11-15T21:09:08Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-knowledge-index-check-dc265b
    claim: "Ran cargo test --locked --test vault -- --exact index_check_names_stale_files_and_passes_after_a_rebuild for TC-cli-knowledge-index-check: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test vault -- --exact index_check_names_stale_files_and_passes_after_a_rebuild
    revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
    environment: local-macos-arm64
    content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
    fingerprint: sha256:dcec9482844b93056b421326c1ac85d228a3d7f385a843bd90d6de809bdbb8d1
    case: TC-cli-knowledge-index-check
    case_digest: 9d8e03aa6c11246bac9437a73345439438d54d93a358854132ac25fb362ea7cf
    observed: 2026-11-15T21:09:09Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-cli-build-reference-from-main-known-outdated-9cab66
    claim: "Ran cargo test --locked --test cli -- --exact build_reference_from_main_4daf861_is_known_outdated for TC-cli-build-reference-from-main-known-outdated: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact build_reference_from_main_4daf861_is_known_outdated
    revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
    environment: local-macos-arm64
    content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
    fingerprint: sha256:6968532466df4484bc80cb442dd592b8b1f8756e03674c0064e8464916cde0ec
    case: TC-cli-build-reference-from-main-known-outdated
    case_digest: e87e897bdf1ab06ffab9894326cd28ecb9b9faed7c463e3dc6c369087343f52f
    observed: 2026-11-15T21:09:10Z
    duration_seconds: 0
    actor: claude-code (hc t28)
```
