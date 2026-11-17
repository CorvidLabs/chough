---
kind: session
id: 2026-11-16-fix-docs
aliases: [fix-docs]
objective: The docs, the workflow skill, and the host templates say what current main does, so a local agent can follow them.
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-docs
pr: ""
criteria: [FLOW-18]
requirements: []
cases: []
design: []
files: []
observations:
  - 2026-11-17-tc-cli-0101
  - 2026-11-17-tc-cli-0090
  - 2026-11-17-tc-cli-0091
  - 2026-11-17-tc-cli-0110
  - 2026-11-17-tc-cli-0112
  - 2026-11-17-tc-cli-native-workflow-bundle-consistency
  - 2026-11-17-tc-cli-change-ship-closes-gate-and-moves-to-done
  - 2026-11-17-tc-cli-change-ship-refusal-passes-through
  - 2026-11-17-tc-cli-change-ship-checks-before-running
  - 2026-11-17-tc-cli-change-ship-names-remaining-step
  - 2026-11-17-tc-cli-change-ship-outlives-read-bound
  - 2026-11-17-tc-cli-change-walkthrough-runs-chough
  - 2026-11-17-tc-cli-change-ship-prints-a-short-summary-and-its-reports
  - 2026-11-17-tc-cli-change-ship-cut-summary-names-the-cut-and-json
  - 2026-11-17-tc-cli-stage-diagram-matches-the-helper
  - 2026-11-17-tc-cli-inline-diagrams-match-sources
  - 2026-11-17-tc-cli-documented-flags-exist
  - 2026-11-17-tc-cli-doc-links-resolve
  - 2026-11-17-tc-cli-documented-merge-drivers-match
  - 2026-11-17-tc-cli-docs-name-pinned-versions
  - 2026-11-17-tc-cli-closeout-closes-review-gate
  - 2026-11-17-tc-cli-skill-615b587b-known-outdated
  - 2026-11-17-tc-cli-cli-command-table-matches-help
  - 2026-11-17-tc-cli-docs-name-what-cases-run-returns
  - 2026-11-17-tc-cli-docs-show-a-new-project-and-a-delta
  - 2026-11-17-tc-cli-docs-show-the-requirement-evidence-table
  - 2026-11-17-tc-cli-docs-show-a-minimal-hi-file-and-requirements-md
  - 2026-11-17-tc-cli-evidence-add-needs-a-criterion-and-reads-the-delta
  - 2026-11-17-tc-cli-stage-skip-docs-say-it-skips-the-current-stage
  - 2026-11-17-tc-cli-accept-step-checks-through-chough
  - 2026-11-17-tc-cli-docs-show-a-lessons-dated-form
  - 2026-11-17-tc-cli-integrations-templates
  - 2026-11-17-tc-cli-integrations-manifest
  - 2026-11-17-tc-cli-integrations-digest-detects-change
  - 2026-11-17-tc-cli-integrations-no-runtime-dependency
  - 2026-11-17-tc-cli-integrations-readme
  - 2026-11-17-tc-cli-integrations-skill-file-exists-after-init
  - 2026-11-17-tc-cli-bug-id-upgrade-note
  - 2026-11-17-tc-cli-roadmap-claims-no-cold-start-pass
  - 2026-11-17-tc-cli-walkthrough-opens-new-project
  - 2026-11-17-tc-cli-skill-routine-fast-path
  - 2026-11-17-tc-cli-skill-names-advance-to
  - 2026-11-17-tc-cli-specsync-merge-driver-wired
  - 2026-11-17-tc-cli-supported-platforms
  - 2026-11-17-tc-cli-docs-name-the-root-library-folder
  - 2026-11-17-tc-records-migration-docs-name-every-key
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-docs
    holds: accept
    drives: review
    opened: 2026-11-17T14:23:12Z
run_consents: []
next: Continue accept
title: Docs and host templates match current main
summary:
  - FLOW-18. The docs, workflow fast path, and five host templates now say what current main does. Forty-six automated docs-linked cases passed locally. Scope approval and this implementation review are relayed claims from 0xLeif. Next is to archive fix-docs and merge the pull request when CI is green.
---
# Describe the current objective before beginning work.

## Goal

Describe the current objective before beginning work.

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-docs": "make a PR and keep improving this and then make sure that we update all the documents and all everything's accurate" (Grok session on 2026-11-16, relayed by grok-t1w); recorded by grok-t1w; a relayed claim, not authenticated; ran specsync change approve fix-docs as 0xLeif

## What was verified

- [Ran fledge run exercise for TC-cli-0101: passed (exit 0).](evidence/2026-11-17-tc-cli-0101.md)
- [Ran cargo test --locked --test package -- --exact the_shipped_bundle_holds_no_em_or_en_dash for TC-cli-0090: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-0090.md)
- [Ran cargo test --locked --test package -- --exact dashed_copies_from_main_2756dc2_are_known_outdated_and_updated for TC-cli-0091: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-0091.md)
- [Ran cargo test --locked --test package -- --exact versions_agree_across_readme_agents_ci_and_specsync_config for TC-cli-0110: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-0110.md)
- [Ran cargo test --locked --test package -- --exact a_skill_copy_from_main_before_the_release_prep_is_known_outdated_and_updated for TC-cli-0112: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-0112.md)
- [Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-native-workflow-bundle-consistency.md)
- [Ran cargo test --locked --test change -- --exact ship_with_the_review_relay_ships_closes_the_gate_and_moves_to_done for TC-cli-change-ship-closes-gate-and-moves-to-done: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-change-ship-closes-gate-and-moves-to-done.md)
- [Ran cargo test --locked --test change -- --exact a_specsync_ship_refusal_passes_through_and_records_nothing for TC-cli-change-ship-refusal-passes-through: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-change-ship-refusal-passes-through.md)
- [Ran cargo test --locked --test change -- --exact ship_refuses_before_running_without_the_relay_or_outside_accept for TC-cli-change-ship-checks-before-running: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-change-ship-checks-before-running.md)
- [Ran cargo test --locked --test change -- --exact a_refused_done_move_after_ship_names_the_remaining_step for TC-cli-change-ship-names-remaining-step: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-change-ship-names-remaining-step.md)
- [Ran cargo test --locked --test change -- --exact ship_outlives_the_native_read_bound for TC-cli-change-ship-outlives-read-bound: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-change-ship-outlives-read-bound.md)
- [Ran cargo test --locked --test change -- --exact the_walkthrough_and_closeout_run_chough_change for TC-cli-change-walkthrough-runs-chough: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-change-walkthrough-runs-chough.md)
- [Ran cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json for TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-change-ship-prints-a-short-summary-and-its-reports.md)
- [Ran cargo test --locked --bin chough -- --exact change::tests::the_summary_names_the_cut_and_json_when_it_is_longer_than_twelve_lines for TC-cli-change-ship-cut-summary-names-the-cut-and-json: passed (exit 0).](evidence/2026-11-17-tc-cli-change-ship-cut-summary-names-the-cut-and-json.md)
- [Ran cargo test --locked --test docs -- --exact stage_diagrams_draw_only_the_helpers_stages_and_moves for TC-cli-stage-diagram-matches-the-helper: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-stage-diagram-matches-the-helper.md)
- [Ran cargo test --locked --test docs -- --exact inline_diagrams_equal_their_sources_and_svgs_carry_every_label for TC-cli-inline-diagrams-match-sources: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-inline-diagrams-match-sources.md)
- [Ran cargo test --locked --test docs -- --exact documented_chough_commands_and_flags_exist for TC-cli-documented-flags-exist: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-documented-flags-exist.md)
- [Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-doc-links-resolve.md)
- [Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-documented-merge-drivers-match.md)
- [Ran cargo test --locked --test docs -- --exact docs_state_only_the_pinned_tool_versions for TC-cli-docs-name-pinned-versions: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-docs-name-pinned-versions.md)
- [Ran cargo test --locked --test docs -- --exact closeout_guidance_closes_the_review_gate_with_the_relayed_decision for TC-cli-closeout-closes-review-gate: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-closeout-closes-review-gate.md)
- [Ran cargo test --locked --test docs -- --exact skill_files_from_main_615b587b_are_known_outdated_and_edited_copies_stay for TC-cli-skill-615b587b-known-outdated: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-skill-615b587b-known-outdated.md)
- [Ran cargo test --locked --test docs -- --exact cli_reference_command_table_matches_the_helpers_help for TC-cli-cli-command-table-matches-help: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-cli-command-table-matches-help.md)
- [Ran cargo test --locked --test cases -- --exact the_cli_reference_names_what_cases_run_returns for TC-cli-docs-name-what-cases-run-returns: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-docs-name-what-cases-run-returns.md)
- [Ran cargo test --locked --test docs -- --exact a_new_project_path_and_the_delta_format_are_documented for TC-cli-docs-show-a-new-project-and-a-delta: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-docs-show-a-new-project-and-a-delta.md)
- [Ran cargo test --locked --test docs -- --exact the_requirement_evidence_table_in_a_changes_testing_md_is_documented for TC-cli-docs-show-the-requirement-evidence-table: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-docs-show-the-requirement-evidence-table.md)
- [Ran cargo test --locked --test docs -- --exact the_new_project_section_shows_a_minimal_hi_file_and_requirements_md for TC-cli-docs-show-a-minimal-hi-file-and-requirements-md: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-docs-show-a-minimal-hi-file-and-requirements-md.md)
- [Ran cargo test --locked --test cases -- --exact evidence_add_needs_a_criterion_and_resolves_the_sessions_delta_requirements for TC-cli-evidence-add-needs-a-criterion-and-reads-the-delta: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-evidence-add-needs-a-criterion-and-reads-the-delta.md)
- [Ran cargo test --locked --test docs -- --exact stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design for TC-cli-stage-skip-docs-say-it-skips-the-current-stage: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-stage-skip-docs-say-it-skips-the-current-stage.md)
- [Ran cargo test --locked --test docs -- --exact the_accept_steps_first_check_runs_through_chough for TC-cli-accept-step-checks-through-chough: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-accept-step-checks-through-chough.md)
- [Ran cargo test --locked --test docs -- --exact a_lessons_dated_form_is_documented_and_its_example_counts for TC-cli-docs-show-a-lessons-dated-form: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-docs-show-a-lessons-dated-form.md)
- [Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-integrations-templates.md)
- [Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-integrations-manifest.md)
- [Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-integrations-digest-detects-change.md)
- [Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-integrations-no-runtime-dependency.md)
- [Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-integrations-readme.md)
- [Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-integrations-skill-file-exists-after-init.md)
- [Ran cargo test --locked --test bugs -- --exact the_cli_documentation_tells_older_versions_to_upgrade_for_new_form_bug_ids for TC-cli-bug-id-upgrade-note: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-bug-id-upgrade-note.md)
- [Ran cargo test --locked --test workflow_authority -- --exact the_roadmap_claims_no_passing_cold_start_for_delivered_work for TC-cli-roadmap-claims-no-cold-start-pass: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-roadmap-claims-no-cold-start-pass.md)
- [Ran cargo test --locked --test cli -- --exact the_walkthrough_begins_with_the_new_project_setup_path for TC-cli-walkthrough-opens-new-project: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-walkthrough-opens-new-project.md)
- [Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-skill-routine-fast-path: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-skill-routine-fast-path.md)
- [Ran cargo test --locked --test package -- --exact the_stages_reference_from_main_01f1d023_is_known_outdated_and_names_advance_to for TC-cli-skill-names-advance-to: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-skill-names-advance-to.md)
- [Ran cargo test --locked --test package -- --exact spec_files_merge_through_specsync_s_driver_in_this_repository for TC-cli-specsync-merge-driver-wired: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-specsync-merge-driver-wired.md)
- [Ran cargo test --locked --test package -- --exact supported_platforms_are_macos_and_linux_with_windows_through_wsl for TC-cli-supported-platforms: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-supported-platforms.md)
- [Ran cargo test --locked --test docs -- --exact the_knowledge_table_names_the_repository_root_library for TC-cli-docs-name-the-root-library-folder: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-cli-docs-name-the-root-library-folder.md)
- [Ran cargo test --locked --test migrate -- --exact the_migration_docs_name_every_key_the_preview_and_apply_print for TC-records-migration-docs-name-every-key: passed (exit 0, tests run 1).](evidence/2026-11-17-tc-records-migration-docs-name-every-key.md)

## Open threads

## Timeline

- 2026-11-16T23:16:52Z · start · intent → intent · fix-docs ^t-18b833
  - revision: 0fd9fd229744da039e9d43a5ce1df6dfe5ddc8d7
  - by: grok-t1w (grok, hc t1w)
- 2026-11-16T23:17:07Z · advance · intent → design ·  ^t-205edb
  - revision: 0fd9fd229744da039e9d43a5ce1df6dfe5ddc8d7
  - by: grok-t1w (grok, hc t1w)
- 2026-11-16T23:17:07Z · skip · design → plan · no visual change ^t-ac0aaa
  - revision: 0fd9fd229744da039e9d43a5ce1df6dfe5ddc8d7
  - by: grok-t1w (grok, hc t1w)
- 2026-11-16T23:19:48Z · gate-opened · plan → plan · Scope approval of fix-docs ^t-a088fc
  - native_change: fix-docs
  - revision: 0fd9fd229744da039e9d43a5ce1df6dfe5ddc8d7
  - holds: plan
  - drives: approve
  - by: grok-t1w (grok, hc t1w)
- 2026-11-16T23:19:48Z · gate-closed · plan → plan · Scope approval of fix-docs ^t-1a5521
  - native_change: fix-docs
  - revision: 0fd9fd229744da039e9d43a5ce1df6dfe5ddc8d7
  - relayed_by:
    - decided_by: 0xLeif
    - words: make a PR and keep improving this and then make sure that we update all the documents and all everything's accurate
    - where: Grok session on 2026-11-16, relayed by grok-t1w
    - recorded_by: grok-t1w
    - native: specsync change approve fix-docs
    - actor: 0xLeif
  - by: grok-t1w (grok, hc t1w)
- 2026-11-16T23:24:59Z · advance · plan → build · Docs and templates edited; targeted package, integrations and docs tests passed. Review is still open. ^t-9d7130
  - native_change: fix-docs
  - gate: native:approved
  - revision: 0fd9fd229744da039e9d43a5ce1df6dfe5ddc8d7
  - by: grok-t1w (grok, hc t1w)
- 2026-11-17T14:20:36Z · advance · build → test · Targeted package, integrations, and docs tests passed on 5cb7e11a. no-commit check verified. ^t-ee7ea6
  - native_change: fix-docs
  - gate: native:verifying
  - revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
  - by: grok-t1w (grok, hc t1w)
- 2026-11-17T14:22:53Z · advance · test → accept · 46 automated docs-linked cases passed on local. Three non-automated cases were not run: TC-cli-0102, TC-cli-0106, TC-cli-0133. ^t-f13d40
  - native_change: fix-docs
  - gate: native:verifying
  - revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
  - by: grok-t1w (grok, hc t1w)
- 2026-11-17T14:23:12Z · gate-opened · accept → accept · Implementation review of fix-docs ^t-d719d8
  - native_change: fix-docs
  - revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
  - holds: accept
  - drives: review
  - by: grok-t1w (grok, hc t1w)
