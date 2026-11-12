---
kind: session
id: 2026-11-06-give-real-assertions-to-the-nine-criteria-the-audit-found-with
aliases: [give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138]
objective: Give real assertions to the nine criteria with no selected assertion (#138, audit assertion review, plan C6)
intent_scope: LIFE
modules: [cli, reporting]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
pr: ""
criteria: [LIFE-3, LIFE-7, FLOW-13, KNOW-16]
requirements:
  - REQ-cli-002
  - REQ-cli-007
  - REQ-cli-012
  - REQ-cli-029
  - REQ-cli-031
  - REQ-cli-045
  - REQ-reporting-003
cases:
  - TC-cli-lesson-labels-in-skill
  - TC-cli-no-lesson-store
  - TC-cli-one-checkpoint-store-no-aps
  - TC-cli-bug-id-upgrade-note
  - TC-cli-authority-before-approval
  - TC-cli-claimed-approvals-open-no-gate
  - TC-cli-skipped-cold-start-not-a-pass
  - TC-cli-roadmap-claims-no-cold-start-pass
  - TC-cli-walkthrough-opens-new-project
  - TC-cli-librarian-off-references-skipped
  - TC-cli-no-reset-drops-no-observation
  - TC-reporting-measure-report-pairs-binaries
design: []
files: []
observations:
  - OBS-product-test-914b2391e3fbf93b4a7a8d94
  - OBS-tc-cli-authority-before-approval-fbd5bf934b0f879a36fd1094
  - OBS-tc-cli-bug-id-upgrade-note-416010ee81c8f8c006a70056
  - OBS-tc-cli-claimed-approvals-open-no-gate-e6df9aafec11a9035810789b
  - OBS-tc-cli-lesson-labels-in-skill-f56aae1735b81683065cb7da
  - OBS-tc-cli-librarian-off-references-skipped-78c662772da0ad1678335bb3
  - OBS-tc-cli-no-lesson-store-d377f20930cd989dae866fe0
  - OBS-tc-cli-no-reset-drops-no-observation-4375674fc4309dbcaa7720ac
  - OBS-tc-cli-one-checkpoint-store-no-aps-a847bd2418a95115590ef735
  - OBS-tc-cli-roadmap-claims-no-cold-start-pass-8f438197307c7964dc5f82cc
  - OBS-tc-cli-skipped-cold-start-not-a-pass-cd3d98c4de358380c172e437
  - OBS-tc-cli-walkthrough-opens-new-project-a6255a1be6e806b186429fc5
  - OBS-tc-reporting-measure-report-pairs-binaries-55ce02c72535d3475d27da9e
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Wait for Leif's implementation review through the orc; the orc records the native review, then this agent ships, finalizes, moves to done, pushes and merges on green plus corvid-agent
closed: 2026-11-06
outcome: done
receipts:
  - id: OBS-product-test-914b2391e3fbf93b4a7a8d94
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-06-tc-cli-authority-before-approval
    aliases: OBS-tc-cli-authority-before-approval-fbd5bf934b0f879a36fd1094
    claim: "Ran cargo test --locked --test workflow_authority -- --exact the_skill_records_authority_before_approval_and_labels_never_decide for TC-cli-authority-before-approval: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test workflow_authority -- --exact the_skill_records_authority_before_approval_and_labels_never_decide
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:7ccb6917f1278e655720a46d40c14fc63b07d16898cf33703761ef98d532fd6c
    case: TC-cli-authority-before-approval
    case_digest: 7fcbf9baecc13b05ef096a15346db8482aa2fbe6894795c7e457f09122172628
    observed: 2026-11-06T05:11:48Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-bug-id-upgrade-note
    aliases: OBS-tc-cli-bug-id-upgrade-note-416010ee81c8f8c006a70056
    claim: "Ran cargo test --locked --test bugs -- --exact the_cli_documentation_tells_older_versions_to_upgrade_for_new_form_bug_ids for TC-cli-bug-id-upgrade-note: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact the_cli_documentation_tells_older_versions_to_upgrade_for_new_form_bug_ids
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:a6acf79607475a82f2fe60f1746137f87c96e1655bd238a84bfac410ed4841f3
    case: TC-cli-bug-id-upgrade-note
    case_digest: 8b0990f42852eab3729b518979243b7e9b061314eefcdaab67e5aae32884d7fe
    observed: 2026-11-06T05:11:40Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-claimed-approvals-open-no-gate
    aliases: OBS-tc-cli-claimed-approvals-open-no-gate-e6df9aafec11a9035810789b
    claim: "Ran cargo test --locked --test workflow_authority -- --exact claimed_approvals_in_chough_records_never_open_the_build_gate for TC-cli-claimed-approvals-open-no-gate: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test workflow_authority -- --exact claimed_approvals_in_chough_records_never_open_the_build_gate
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:d8ae52bb350e3dc812ff13686d372fec33c8410f01560c590755d96467b257ff
    case: TC-cli-claimed-approvals-open-no-gate
    case_digest: 360345c0a0d26d554e3361e38a124bfb9e93f256cca423b9e94244899c19890a
    observed: 2026-11-06T05:11:55Z
    duration_seconds: 1
  - id: 2026-11-06-tc-cli-lesson-labels-in-skill
    aliases: OBS-tc-cli-lesson-labels-in-skill-f56aae1735b81683065cb7da
    claim: "Ran cargo test --locked --test cli -- --exact the_skill_states_lesson_labels_sources_and_limits_and_that_decisions_outrank_lessons for TC-cli-lesson-labels-in-skill: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact the_skill_states_lesson_labels_sources_and_limits_and_that_decisions_outrank_lessons
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:8c844df7006898dd224b957c4389aa6fac5b3a9df0788006da06f29a070cdae4
    case: TC-cli-lesson-labels-in-skill
    case_digest: 6835e82c23fdec63c35ef066524afab9cd78afe5951961c15bd19048fd5abee8
    observed: 2026-11-06T05:11:11Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-librarian-off-references-skipped
    aliases: OBS-tc-cli-librarian-off-references-skipped-78c662772da0ad1678335bb3
    claim: "Ran cargo test --locked --test reduce -- --exact with_the_librarian_off_done_leaves_the_references_and_reports_skipped for TC-cli-librarian-off-references-skipped: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact with_the_librarian_off_done_leaves_the_references_and_reports_skipped
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:0983e61d66fe2c7cb5e2aa090d10284f1a39c45261da2c92ee02eb01b8d15e63
    case: TC-cli-librarian-off-references-skipped
    case_digest: b46c4c4e6c6f1ad2c44f5c5264f6b3a98c68c1ace7367ac634ec5300fff4a13e
    observed: 2026-11-06T05:12:14Z
    duration_seconds: 1
  - id: 2026-11-06-tc-cli-no-lesson-store
    aliases: OBS-tc-cli-no-lesson-store-d377f20930cd989dae866fe0
    claim: "Ran cargo test --locked --test cli -- --exact chough_points_to_lessons_and_keeps_no_lesson_store_index_or_ledger for TC-cli-no-lesson-store: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact chough_points_to_lessons_and_keeps_no_lesson_store_index_or_ledger
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:797c2e2d30d767066e564424210411606e84a04861cf98a1706e743bff272919
    case: TC-cli-no-lesson-store
    case_digest: f071e175bd328a461efd22c1970619870a43f3a708fc3f18710aadc8eb3f05d5
    observed: 2026-11-06T05:11:16Z
    duration_seconds: 1
  - id: 2026-11-06-tc-cli-no-reset-drops-no-observation
    aliases: OBS-tc-cli-no-reset-drops-no-observation-4375674fc4309dbcaa7720ac
    claim: "Ran cargo test --locked --test reduce -- --exact with_no_reset_time_done_drops_no_observation_reference for TC-cli-no-reset-drops-no-observation: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact with_no_reset_time_done_drops_no_observation_reference
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:8484a9401441b2c2db8b52fd229b17c15b2c6eef8c31bbaf1dd2617e0971c8fd
    case: TC-cli-no-reset-drops-no-observation
    case_digest: fd6ddb27b89f970b73f92b506ade82e49027b75cb4c9a4643f129edf16835f87
    observed: 2026-11-06T05:12:27Z
    duration_seconds: 5
  - id: 2026-11-06-tc-cli-one-checkpoint-store-no-aps
    aliases: OBS-tc-cli-one-checkpoint-store-no-aps-a847bd2418a95115590ef735
    claim: "Ran cargo test --locked --test stages -- --exact one_checkpoint_store_and_no_aps_adapter_in_either_mode for TC-cli-one-checkpoint-store-no-aps: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact one_checkpoint_store_and_no_aps_adapter_in_either_mode
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:2fd91525ec0860e747c65800839e19f4fdcf3044af79dcd4b5e1af61b2428b77
    case: TC-cli-one-checkpoint-store-no-aps
    case_digest: af6a8cacd374b680d8bf6acdf2384a6b35b25a60e9d58bacb184fef96eedfd34
    observed: 2026-11-06T05:11:28Z
    duration_seconds: 1
  - id: 2026-11-06-tc-cli-roadmap-claims-no-cold-start-pass
    aliases: OBS-tc-cli-roadmap-claims-no-cold-start-pass-8f438197307c7964dc5f82cc
    claim: "Ran cargo test --locked --test workflow_authority -- --exact the_roadmap_claims_no_passing_cold_start_for_delivered_work for TC-cli-roadmap-claims-no-cold-start-pass: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test workflow_authority -- --exact the_roadmap_claims_no_passing_cold_start_for_delivered_work
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:d03d6ff0d440e81f4257c9d3e3dd46dd3a5e7ae758b23055fb0cf911e51555db
    case: TC-cli-roadmap-claims-no-cold-start-pass
    case_digest: 12c2f616ab4dd2a48ac50ba284d0b6fa92457cbeb890bb94a57a4b175d0ba8c8
    observed: 2026-11-06T05:12:05Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-skipped-cold-start-not-a-pass
    aliases: OBS-tc-cli-skipped-cold-start-not-a-pass-cd3d98c4de358380c172e437
    claim: "Ran cargo test --locked --test cases -- --exact a_skipped_cold_start_is_never_recorded_or_read_as_a_pass for TC-cli-skipped-cold-start-not-a-pass: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact a_skipped_cold_start_is_never_recorded_or_read_as_a_pass
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:468f275932aca07494223b2b95a374497965206d2e80a8c01a439a06fa8b22d6
    case: TC-cli-skipped-cold-start-not-a-pass
    case_digest: 049a3afa988e7dc4f3dc09226d37dbd61777f5d68a0db9a40a9d7a22fd8b7957
    observed: 2026-11-06T05:12:01Z
    duration_seconds: 2
  - id: 2026-11-06-tc-cli-walkthrough-opens-new-project
    aliases: OBS-tc-cli-walkthrough-opens-new-project-a6255a1be6e806b186429fc5
    claim: "Ran cargo test --locked --test cli -- --exact the_walkthrough_begins_with_the_new_project_setup_path for TC-cli-walkthrough-opens-new-project: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact the_walkthrough_begins_with_the_new_project_setup_path
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:d5d239164f9f57ce69a8cb51439d1e13ca7ff27538d8fc50e98bc9985c0e82d7
    case: TC-cli-walkthrough-opens-new-project
    case_digest: fcc0af3572fabbb389e19012fcaad1d387c15c061fd4490d08abcb4614803e1a
    observed: 2026-11-06T05:12:09Z
    duration_seconds: 0
  - id: 2026-11-06-tc-reporting-measure-report-pairs-binaries
    aliases: OBS-tc-reporting-measure-report-pairs-binaries-55ce02c72535d3475d27da9e
    claim: "Ran cargo test --locked --test timing -- --exact measure_report_pairs_two_binaries_on_one_root_and_keeps_native_wait_apart for TC-reporting-measure-report-pairs-binaries: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact measure_report_pairs_two_binaries_on_one_root_and_keeps_native_wait_apart
    revision: b87a108798e4199ff5100d1e29dab7162b8e322e
    environment: local-macos-arm64
    content: 0d2e31d59d5b53ebee1b2162ea550a0c0e22a291d947951607dfc7e5a18e9830
    fingerprint: sha256:a55e336f5bad676452df521d7f922f7ac2ce0a401e00933c604dacff873ca77a
    case: TC-reporting-measure-report-pairs-binaries
    case_digest: 0b27db6ab792b2b6ef3db3a79ff5096c342875bbb314347cfb76c7ef59c982b9
    observed: 2026-11-06T05:11:06Z
    duration_seconds: 8
---
# Give real assertions to the nine criteria with no selected assertion (#138, audit assertion review, plan C6)

## Goal

Give real assertions to the nine criteria with no selected assertion (#138, audit assertion review, plan C6)

## Where it stands

- C6 definition drafted with Leif's choices (both A); measure_report_pairs_two_binaries_on_one_root_and_keeps_native_wait_apart fails first (no script yet)
- examples/measure-report.sh and eleven more assertions for the nine criteria; 12 cases pass on macOS; mutation round recorded as OBS-product-test-914b2391e3fbf93b4a7a8d94 (11 of 11 caught)

## Decisions and why

- Leif (0xLeif), 2026-11-05 21:52 MST, through the orc (t18): 'Both recommended': REQ-cli-029/AC-5 asserted in Chough's evidence model plus ROADMAP text; REQ-reporting-003's measurement criterion through examples/measure-report.sh plus a test.
- failed: opening the gate 'Scope decision of give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138' with checkpoint --awaiting-human because of the KNOW-29 history regression (legacy history anchor changed); the gate waits in the orc, not in the session, until chough's fix lands
- Leif (0xLeif), through the orc (t18), 'Approve' for the C6 definition; the orc recorded the native approval on 8833f048. The session stays at intent until chough's KNOW-29 history fix lets stage events be written.
- decided: stage moves resumed after chough#141's fix (#142, 90e5de6f) merged into this branch; intent -> design (skipped) -> plan -> build -> test -> accept
- decided: after merging main (#140, D7) the authority test follows the skill's new wording: plan.md 'An actor label alone never supplies human authority' now continues ', and neither Chough nor SpecSync authenticates who decided' (also required now), and SKILL.md says 'actor labels are not human decisions; approvals are claims'; the placement check and the other phrases are unchanged, and the plan.md mutation still fails it (rerun log kept)
- Leif (0xLeif), 2026-11-05 22:43 MST, through the orc (standing rule): when a PR he approved only gets origin/main merged in (bookkeeping, changelog or version conflicts, no change to reviewed content) and checks re-run green, merge at the new head without asking again and say so; any change to reviewed content comes back for review.
- Leif (0xLeif), 2026-11-05 23:24 MST, through the orc: agent commits keep the worktree's git identity (0xLeif) as author; existing commits are not rewritten; native SpecSync approvals and reviews in his name come only through the orc.
- Leif (0xLeif) decided "Implementation review of give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138": "im telling you to approve and continue all of these... approve as me" (orc tab, 2026-11-06 about 07:0x MST; relayed by the orc (hc terminal t18), who recorded the native review as 0xLeif at 7894feaa); recorded by ch-c456 (Claude Opus 5.5); a relayed claim, not authenticated

## What was verified

- OBS-product-test-914b2391e3fbf93b4a7a8d94
- OBS-tc-cli-authority-before-approval-fbd5bf934b0f879a36fd1094
- OBS-tc-cli-bug-id-upgrade-note-416010ee81c8f8c006a70056
- OBS-tc-cli-claimed-approvals-open-no-gate-e6df9aafec11a9035810789b
- OBS-tc-cli-lesson-labels-in-skill-f56aae1735b81683065cb7da
- OBS-tc-cli-librarian-off-references-skipped-78c662772da0ad1678335bb3
- OBS-tc-cli-no-lesson-store-d377f20930cd989dae866fe0
- OBS-tc-cli-no-reset-drops-no-observation-4375674fc4309dbcaa7720ac
- OBS-tc-cli-one-checkpoint-store-no-aps-a847bd2418a95115590ef735
- OBS-tc-cli-roadmap-claims-no-cold-start-pass-8f438197307c7964dc5f82cc
- OBS-tc-cli-skipped-cold-start-not-a-pass-cd3d98c4de358380c172e437
- OBS-tc-cli-walkthrough-opens-new-project-a6255a1be6e806b186429fc5
- OBS-tc-reporting-measure-report-pairs-binaries-55ce02c72535d3475d27da9e

## Open threads

## Timeline

- 2026-11-06T05:22:07Z · advance · intent → design ·  ^t-308193
  - native_change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
  - revision: 8e24840a6f87f96913ccf267e6253ca4299d45b8
- 2026-11-06T05:22:08Z · skip · design → plan · Design not applicable: tests, a measurement script and docs, with no visuals or interactions ^t-b11e65
  - native_change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
  - revision: 8e24840a6f87f96913ccf267e6253ca4299d45b8
- 2026-11-06T05:22:18Z · advance · plan → build · native:verifying ^t-f56cf3
  - native_change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
  - gate: native:verifying
  - revision: 8e24840a6f87f96913ccf267e6253ca4299d45b8
- 2026-11-06T05:22:30Z · advance · build → test · native:verifying ^t-dbaa7e
  - native_change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
  - gate: native:verifying
  - revision: 8e24840a6f87f96913ccf267e6253ca4299d45b8
- 2026-11-06T05:22:41Z · advance · test → accept · native:verifying ^t-69b6d5
  - native_change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
  - gate: native:verifying
  - revision: 8e24840a6f87f96913ccf267e6253ca4299d45b8
- 2026-11-06T05:22:58Z · gate-opened · accept → accept · Implementation review of give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138 ^t-25c15d
  - native_change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
  - revision: 8e24840a6f87f96913ccf267e6253ca4299d45b8
- 2026-11-06T14:08:53Z · gate-closed · accept → accept · Implementation review of give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138 ^t-37288e
  - native_change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
  - revision: c7af84bdd527d09dafd312b794bd9ebf529a97e2
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"im telling you to approve and continue all of these... approve as me","where":"orc tab, 2026-11-06 about 07:0x MST; relayed by the orc (hc terminal t18), who recorded the native review as 0xLeif at 7894feaa","recorded_by":"ch-c456 (Claude Opus 5.5)"}
- 2026-11-06T14:09:05Z · advance · accept → done · native:archived ^t-7c1a70
  - native_change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
  - gate: native:archived
  - revision: c7af84bdd527d09dafd312b794bd9ebf529a97e2


Closed 2026-11-06: done.
