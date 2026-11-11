---
change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
artifact: testing
---

# Testing

## Requirement evidence

Each case binds one cargo test with the `cargo-test` adapter, in a case file of its
own per module, `.chough/cases/<module>.human-first-0-6-0.cases.toml`. A case may
link a requirement this change adds or modifies only after approval, while its delta
is unchanged; until `change check` materializes the delta, those cases are defined
in build and recorded in test. Cases for every acceptance criterion of each
requirement are inside the named test, one assertion block per criterion.

| Requirement | Case | Test | Shows |
|---|---|---|---|
| REQ-records-001 | TC-records-files-are-markdown-and-tolerant | tests/records.rs `every_record_is_markdown_and_keeps_human_edits` | front matter subset, unknown keys kept, prose kept, refusals with file and line, bounds |
| REQ-records-002 | TC-records-tree-and-readme | tests/records.rs `the_tree_holds_only_the_listed_paths` | init and a full cycle write only the tree's paths; README create-only |
| REQ-records-003 | TC-records-date-slug-names-and-aliases | tests/records.rs `names_are_date_and_slug_with_aliases` | slug rules, clash suffix, refused second clash, alias resolution and conflicts, merge of two branches |
| REQ-records-004 | TC-records-session-story-sections | tests/records.rs `the_session_record_is_checkpoint_and_story` | keys, sections, decisions and questions in the story only |
| REQ-records-005 | TC-records-proof-split-and-external | tests/records.rs `logs_stay_local_and_failures_keep_key_lines` | local logs, digest, key failing lines with redaction, external artifacts by ID and fingerprint |
| REQ-records-006 | TC-records-leak-guard | tests/leaks.rs `no_write_leaks_and_doctor_finds_old_leaks` | each pattern kind refused or redacted, value never echoed, doctor finding, verify lane fails |
| REQ-records-007 | TC-records-actor-recorded | tests/records.rs `transitions_evidence_and_bugs_record_the_acting_agent` | override and detected values, none when unknown, unsafe terminal dropped |
| REQ-records-008 | TC-records-approval-as-the-human | tests/approval.rs `a_gate_decision_approves_as_the_configured_human` | approve and review run as the configured actor through a SpecSync stub; refusals; no default name in sources |
| REQ-records-009 | TC-records-migrate-from-0-5 | tests/migrate.rs `migrate_previews_applies_and_keeps_every_byte_in_git` | preview, refusal on uncommitted files, mapping, receipt digests, idempotence, interrupted apply, removed IDs |
| REQ-records-010 | TC-records-unmigrated-fails-closed | tests/migrate.rs `an_unmigrated_tree_is_refused_with_the_migrate_command` | every command refused, allowed commands run, mixed tree |
| REQ-records-011 | TC-records-journal-and-feed | tests/journal.rs `done_writes_a_short_post_and_the_feed_lists_it` | post at done, archive link, never overwritten, feed determinism, index names the timeline |
| REQ-records-012 | TC-records-only-committed-bytes-leave | tests/records.rs `nothing_leaves_the_tree_unless_head_holds_it` | finish keeps uncommitted files, migrate refuses changed files, 40 stories untouched |
| REQ-records-013 | TC-records-key-screenshots | tests/records.rs `small_key_screenshots_are_committed_beside_the_story` | metadata stripped, bounds and remedies, moved beside the story at finish |
| REQ-history-002 | TC-history-append-only-timeline | tests/history.rs `transitions_are_append_only_timeline_entries` | grammar, unique IDs, atomic with the checkpoint, rewritten timeline found by doctor |
| REQ-history-003 | TC-history-timeline-bounds | tests/history.rs `the_timeline_is_bounded` | entry and count bounds, next reads 128, timing partial |
| REQ-history-005 | TC-history-backfill-on-entries | tests/history.rs `a_backfilled_entry_keeps_its_fields` | backfill fields round trip |
| REQ-history-006 | TC-history-165-earlier-archives-stay | tests/sessions.rs `done_leaves_earlier_archived_stories_in_place` | the #165 path: seven earlier stories, `knowledge/` untracked, nothing else changes |
| REQ-history-006 | TC-history-sessions-in-folders | tests/sessions.rs `sessions_live_in_folders_and_finish_into_one_story` | new, use, list, finish, kept_open cases, release, close, shelve |
| REQ-history-007 | TC-history-local-only-ignored | tests/sessions.rs `local_state_is_ignored_by_git` | every local file ignored |
| REQ-history-009 | TC-history-inbox-in-agent-folder | tests/sessions.rs `inbox_entries_live_in_the_agent_folder` | lock, whole publication, fold once |
| REQ-history-010 | TC-history-relay-fields-on-gate-entries | tests/history.rs `relay_fields_live_on_gate_closed_entries` | four values together, native step fields, refusals |
| REQ-history-012 | TC-history-doctor-findings-0-6 | tests/doctor.rs `doctor_reports_and_fixes_0_6_debt` | unmigrated, leak, unfinished archive, timeline rewritten, alias conflict, existing findings |
| REQ-history-013 | TC-history-direct-write-entries | tests/history.rs `direct_write_entries_are_known_to_timing` | fields, refusal elsewhere, timing complete |
| REQ-history-014 | TC-history-relay-summary-digests | tests/history.rs `relay_digests_come_in_pairs` | both or neither |
| REQ-history-015 | TC-history-append-only-corrections | tests/history.rs `a_correction_appends_and_is_shown_without_rewriting_history` | correction entry, archived story append only, refusals, bounded list |
| REQ-cli-007 | TC-cli-checkpoint-in-session-front-matter | tests/cli.rs `the_checkpoint_is_the_session_front_matter` | round trip across processes, typed refusals, native-change refusals unchanged |
| REQ-cli-011 | TC-cli-case-run-writes-entry-and-local-log | tests/cases.rs `a_case_run_writes_an_entry_and_a_local_log` | existing execution assertions ported, log path, names |
| REQ-cli-012 | TC-cli-bugs-are-markdown | tests/bugs.rs `bugs_are_markdown_with_an_append_only_log` | existing bug assertions ported, Log append-only, aliases, receipts after archive |
| REQ-cli-014 | TC-cli-delivery-kinds-as-receipts | tests/stages.rs `delivery_kinds_survive_as_receipts` | kinds kept in the story |
| REQ-cli-015 | TC-cli-librarian-links-only | tests/knowledge_default.rs `the_librarian_copies_nothing` | removed commands, snapshots and briefs kept |
| REQ-cli-017 | TC-cli-evidence-entries | tests/evidence.rs `evidence_entries_carry_their_receipt` | keys, kinds, freshness parity for receipts, response keys |
| REQ-cli-019 | TC-cli-0110 and TC-cli-bug-0022-toolchain-matches-ci | tests/package.rs `versions_agree_across_readme_agents_ci_and_specsync_config`, `toolchain_task_names_a_version_that_differs` | the config comment joins the version check (BUG-0001); the toolchain task fails on a differing version (BUG-0022) |
| REQ-cli-021 | TC-cli-next-reads-the-story | tests/next.rs `next_reads_decisions_and_questions_from_the_story` | existing bounds ported, lesson bundles match the index |
| REQ-cli-022 | TC-cli-bug-0020-guard-uses-effective-approval and TC-cli-delta-guard-refusals-name-the-agent-actor (rewritten) | tests/native_check.rs `the_delta_guard_compares_with_the_effective_approval`; the ported guard tests | the effective, digest-matching approval (BUG-0020); the remedy names the scope gate, and the agent only for authority given in advance |
| REQ-cli-023 | TC-cli-names-and-draws | tests/ids.rs `new_records_get_date_slug_names_without_retry` | existing identity assertions ported to names |
| REQ-cli-029 | TC-cli-authority-before-approval (rewritten) | tests/workflow_authority.rs `authority_given_in_advance_names_the_agent_and_quotes_the_human` | a gate decision is the human's; authority given in advance names the agent as `<agent>-<terminal>`, for an approval or a review, with the quoted words (FLOW-24); a refused agent actor is reported, never retried as the human; the skill references agree |
| REQ-cli-030 | TC-cli-237-gate-holds-its-stage | tests/gates.rs `a_gate_holds_only_the_stage_it_names` | the #237 regression and the ported gate assertions |
| REQ-cli-032 | TC-cli-session-record-capacity | tests/cli.rs `the_session_record_is_bounded_at_one_mebibyte` | the 1 MiB and 400 bounds, timing at the cap |
| REQ-cli-034 | the ported REQ-cli-034 cases | tests/cli.rs, the existing size tests | spec files read at any size; no `library snapshot` exception; the 0.6.0 record bounds |
| REQ-cli-037 | TC-cli-active-session-folder | tests/sessions.rs `the_active_session_is_resolved_once` | pointer rules, mid-command switch, init with objective |
| REQ-cli-040 | TC-cli-one-writer-lock | tests/cli.rs `record_writers_share_one_lock` | busy refusal, two bug notes both kept |
| REQ-cli-042 | TC-cli-archived-finalization-over-record-bound (rewritten) | tests/cli.rs `an_archived_finalization_over_the_record_bound_still_tells_a_finished_session` | native sources read at full size by the finished-session check, timing and the index; no prune reader |
| REQ-cli-043 | TC-cli-index-links-every-lesson | tests/vault.rs `the_index_is_a_reading_guide_that_links_every_lesson` | index contents, refresh at done, check passes after done |
| REQ-cli-044 | TC-cli-vault-holds-only-its-own | tests/vault.rs `the_vault_holds_only_what_has_no_other_home` | allowed paths only, no copied text, links resolve |
| REQ-cli-047 | TC-cli-markdown-bug-merge | tests/bugs.rs `bug_merge_unions_logs_and_takes_one_sided_edits` | driver, attributes, index check |
| REQ-cli-050 | TC-cli-close-gate-drives-native-step | tests/gates.rs `closing_a_driving_gate_runs_the_native_step_first` | existing relay assertions ported, drives none vs approve |
| REQ-cli-052 | TC-cli-advance-to-one-read | tests/advance_to.rs `advance_to_records_entries_on_one_read` | entries instead of events, one read into done |
| REQ-cli-059 | TC-cli-ship-reviews-as-human | tests/change.rs `ship_records_review_as_the_human_then_ships` | review then ship through a stub, refusals |
| REQ-cli-061 | TC-cli-gate-record-one-driving-gate | tests/gates.rs `gate_record_runs_at_most_one_native_step` | the new refusal and stage column |
| REQ-reporting-001 | TC-reporting-timing-from-timelines | tests/timing.rs `timing_reads_timelines_and_receipts` | segments from open and archived timelines, partial on unreadable |

## Every open Chough bug, with its regression case

Leif widened 0.6.0 to fix every open Chough bug ("all bugs?", 2026-11-10). A row still
open is fixed in build and closed only with a passing observation of its case recorded
after a failing one, or, for a flake, with the stress result recorded honestly. A row
already fixed elsewhere is closed with a link to the fix, and its case stays as the
regression. The remaining Chough work is BUG-0020, BUG-0022, BUG-0023, the lock-timeout
and timing-budget flakes, #132, and the issues #12, #14 item 3, #76, #165, #167, #190,
#201, #234, #236 and #237.

| Bug | Fix in 0.6.0 | Requirement | Case | Failing first |
|---|---|---|---|---|
| #237 a gate blocks every advance | a gate holds only the stage it names (option A, pending Leif) | REQ-cli-030 | TC-cli-237-gate-holds-its-stage | a release gate refuses build to test on 0.5.0 |
| #165 done deletes an earlier archived session | no roll-up; nothing removes another session's story | REQ-history-006, REQ-records-012 | TC-history-165-earlier-archives-stay | the 0.5.x layout loses `release-a` on 0.5.0 |
| #12 Chough records and SpecSync verification stale each other | a real native closeout end to end: cases run, commit, `change check --commit`, review as the human, ship, done, with no stale verification and every observation still current; the documented order stays in the accept reference | REQ-cli-004, REQ-cli-059 | TC-cli-12-native-closeout-stays-current (`tests/native.rs` `a_real_native_closeout_keeps_both_sides_current`, real SpecSync at the CI pin) | the case is run first at the 0.5.0 code to record where it stands; if it already passes there, the bug closes on that evidence and the case stays as the regression |
| #190 gate summaries count Chough's records and merged main | count from the merge base with the default branch, leaving out `.chough/`, the knowledge directory and SpecSync bookkeeping | REQ-cli-061 | TC-cli-190-gate-summary-counts-the-change (`tests/gates.rs` `gate_summary_counts_only_the_changes_own_files`) | the fixture counts 501 files on 0.5.0 |
| #76 next guidance for a fresh agent | items 2 and 5 are fixed on main (TC-cli-0121, TC-cli-0126 kept); items 1, 3 and 4 get the wording in REQ-cli-021 | REQ-cli-021 | TC-cli-76-next-guides-a-fresh-agent (`tests/next.rs` `next_guides_a_fresh_agent_from_init_to_accept`) | the init, test and accept actions differ on 0.5.0 |
| #14 env, colour codes, external artifacts, field errors | items 1, 2 and 4 shipped in #16 and stay proven; item 3 is REQ-records-005 | REQ-cli-009, REQ-cli-011, REQ-cli-054, REQ-records-005 | TC-cli-0097, TC-cli-0098, TC-cli-0099, TC-cli-0092 (ported) and TC-records-proof-split-and-external | item 3 refuses an absolute artifact on 0.5.0 |
| #132 fork and descriptor inheritance flakes | not reproduced since 2026-11-06. Stand-ins are already written through a temporary name and renamed; the lock refusal names its holder's process ID; the in-process lock test waits a bounded time for a forked child's inherited descriptor; a stress case records how often the lock and stand-in tests fail under a parallel suite | REQ-cli-040 | TC-cli-132-fork-inheritance-stress (`tests/cli.rs` `lock_and_stand_in_tests_survive_parallel_forks`) | the stress run on 0.5.0 is recorded as observed; if neither failure reproduces, the bug says so and stays open with the diagnostics in place |
| BUG-0001 the SpecSync config comment names stale versions | already fixed on main (the comment says 6.2.0): close the record with that link; the comment also joins the version agreement check so it cannot drift again | REQ-cli-019 | TC-cli-0110 (rewritten) | a stale comment fails the check |
| BUG-0020 the delta guard uses the latest approval | compare with the effective approval, the digest-matching one, for every workflow version | REQ-cli-022 | TC-cli-bug-0020-guard-uses-effective-approval (`tests/native_check.rs` `the_delta_guard_compares_with_the_effective_approval`) | three old-version approvals refuse wrongly on 0.5.0 |
| BUG-0022 local tools differ from the CI pins | a `toolchain` Fledge task, first in the verify lane, fails naming each version that differs, and evidence from a differing toolchain says so | REQ-cli-019 | TC-cli-bug-0022-toolchain-matches-ci (`tests/package.rs` `toolchain_task_names_a_version_that_differs`) | no such check exists on 0.5.0 |
| BUG-0023 cold-start output is hard to scan | `status` leads with the same short summary as `next` | REQ-cli-021 | TC-cli-bug-0023-status-leads-with-summary (`tests/next.rs`) | the summary is missing on 0.5.0 |
| BUG-…-empty-change-directory (#148) | fixed by #157 for the index: close the record with that link; 0.6.0 extends the rule to every reader of open changes | REQ-cli-043 | TC-cli-empty-change-dir-not-open (rewritten) | the case is rerun on 0.5.0 for `next`, `gate list` and `doctor` |
| BUG-…-history-writes-refuse-once-the-adopting-session-rolls-into-the-vault | fixed by #142: close the record with that link; the roll-up and the legacy anchor also go, and migration puts the legacy entries in the adopter's story | REQ-records-009 | TC-records-migrate-vaulted-adopter (`tests/migrate.rs` `a_vaulted_adopter_migrates_and_writes_work`) | the 0.5.x fixture is built at the release before #142, where the writes refuse |
| BUG-…-full-macos-lane-checkpoint-writer-lock-timeout | root cause unverified; same family as #132. Each writer child reports how long it waited, and the stress case runs the writer test under the full parallel suite on macOS | REQ-cli-040 | TC-cli-132-fork-inheritance-stress | as #132 |
| BUG-…-timing-native-budget-…-no-initial-stub-call | the budget clock starts at the first native read, as REQ-reporting-002 already bounds only time spent waiting on native reads | REQ-reporting-002 | TC-reporting-budget-starts-at-first-read (`tests/timing.rs` `a_slow_record_load_does_not_spend_the_native_budget`) | a record load slower than the budget starts no read on 0.5.0 |
| BUG-…-timing-trace-…-cross-a-second | fixed by #172 (TC-reporting-timing-trace-straddles-a-second): close the record with that link; the case also runs in the stress run | REQ-reporting-002 | TC-reporting-timing-trace-straddles-a-second | not needed: fixed before this change |

BUG-0002 and BUG-0003 describe SpecSync's own behaviour, fixed upstream in spec-sync
#823 and #824 on 2026-11-01. A spec-sync worker (t37b) checked both again on spec-sync
main 46f55652 (SpecSync 6.2.0) and neither reproduces: editing
an approved delta turns `approval_valid` false and `change check` refuses, and a
failing gate records `passed: false`. 0.6.0 closes both as `fixed-upstream`, citing
those PRs and that check, never as fixed or verified by Chough (REQ-cli-012). Chough ran no check of
its own for them.

## Failing first

- TC-history-165-earlier-archives-stay starts as a test in the 0.5.x layout on the
  0.5.0 code. It is recorded failing there (the earlier archived session leaves the
  tree) and linked to a local bug before any implementation, then rewritten for the
  0.6.0 layout and recorded passing.
- TC-cli-237-gate-holds-its-stage is recorded failing on 0.5.0 the same way (an
  advance at build is refused by a gate meant for release) and linked to a bug.
- Every other new test is written before its implementation and seen failing once.
  Ported tests keep their assertions; a test is deleted only with the requirement
  that retires its behaviour (REQ-cli-033, 041, 045, 046, 048, 049, 051 and
  REQ-history-001, 004, 008, 011), and tasks.md names each one.

## Suites and platforms

- `fledge run fmt`, `fledge run lint`, `fledge run test` (the full suite) on every
  increment, macOS arm64.
- `fledge lanes run verify`, with the new `leaks` step and the existing `cases`,
  `knowledge`, `intent`, `spec` and `audit` steps, on macOS and in the Linux images
  (`fledge run linux-verify`) before accept.
- Windows is best-effort through WSL (FLOW-17, FLOW-21): no Windows run is claimed.

## Dogfood checks (recorded as evidence)

- `chough migrate` preview and `--apply` on a throwaway clone and then on this
  branch; receipt digests compared with the previous commit.
- The 0.6.0 build refuses an unmigrated clone of main with `refused:unmigrated`.
- This session's move into done in 0.6.0: its story, journal post, feed entry and an
  unchanged archive of earlier stories.
- CI's archive authentication over the migrated tree.

Running and recording cases, the human scope approval and the human acceptance are
plain steps, not checkboxes.
