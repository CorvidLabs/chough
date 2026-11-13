---
kind: session
id: 2026-11-13-mark-0-6-0-published
aliases: [mark-0-6-0-published]
objective: Mark the 0.6.0 release published in the roadmap with its install observation (releasing.md step 9)
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
pr: ""
criteria: [LIFE-25]
requirements: []
cases:
  - TC-cli-native-workflow-bundle-consistency
  - TC-cli-documented-flags-exist
  - TC-cli-doc-links-resolve
  - TC-cli-docs-name-pinned-versions
  - TC-cli-roadmap-claims-no-cold-start-pass
design: []
files: []
observations:
  - 2026-11-13-clean-cargo-install-chough-version-0-6-0-locked-from-crates-io
  - 2026-11-13-tc-cli-native-workflow-bundle-consistency
  - 2026-11-13-tc-cli-documented-flags-exist
  - 2026-11-13-tc-cli-doc-links-resolve
  - 2026-11-13-tc-cli-docs-name-pinned-versions
  - 2026-11-13-tc-cli-roadmap-claims-no-cold-start-pass
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
    holds: accept
    drives: review
    opened: 2026-11-13T01:56:00Z
run_consents: []
next: Continue accept
title: Chough 0.6.0 marked published in the roadmap
summary:
  - "releasing.md step 9 for Chough 0.6.0: the roadmap's delivered table now lists the 0.6.0 release, tag v0.6.0 at 443c63ab, crates.io chough 0.6.0 and the GitHub release, with the clean install that printed chough 0.6.0."
  - "Verified: the docs cases for the roadmap pass, and the install is recorded as the coordinator's relayed observation. Next: Leif's merge."
---
# Mark the 0.6.0 release published in the roadmap with its install observation (releasing.md step 9)

## Goal

Mark the 0.6.0 release published in the roadmap with its install observation (releasing.md step 9)

## Where it stands

- docs/roadmap-1.0.md: the 0.6.0 release row replaces the in-progress row; the install observation recorded as t370's relayed run; the five automated docs cases selected for the file pass

## Decisions and why

- Leif decided "Approve the intent: mark 0.6.0 published in the roadmap with its install observation": "Yes (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T01:43Z, answering: 'Last release step: mark 0.6.0 as published in Chough's roadmap, plus the install check result, as one small docs PR with no requirement change. OK?' (option text: 't382 opens it. Your merge of it counts as the review.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation": "Yes (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T01:43Z, answering: 'Last release step: mark 0.6.0 as published in Chough's roadmap, plus the install check result, as one small docs PR with no requirement change. OK?' (option text: 't382 opens it. Your merge of it counts as the review.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation as 0xLeif

## What was verified

- [Clean cargo install chough --version 0.6.0 --locked from crates.io into an empty root with a fresh target directory on macOS arm64 succeeded; the installed chough --version printed chough 0.6.0](evidence/2026-11-13-clean-cargo-install-chough-version-0-6-0-locked-from-crates-io.md)
- [Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-native-workflow-bundle-consistency.md)
- [Ran cargo test --locked --test docs -- --exact documented_chough_commands_and_flags_exist for TC-cli-documented-flags-exist: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-documented-flags-exist.md)
- [Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-doc-links-resolve.md)
- [Ran cargo test --locked --test docs -- --exact docs_state_only_the_pinned_tool_versions for TC-cli-docs-name-pinned-versions: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-docs-name-pinned-versions.md)
- [Ran cargo test --locked --test workflow_authority -- --exact the_roadmap_claims_no_passing_cold_start_for_delivered_work for TC-cli-roadmap-claims-no-cold-start-pass: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-roadmap-claims-no-cold-start-pass.md)

## Open threads

## Timeline

- 2026-11-13T01:43:03Z · start · intent → intent · Mark the 0.6.0 release published in the roadmap with its install observation (releasing.md step 9) ^t-7e7baa
  - revision: 443c63ab65b747f47248ad2166f8fb299da94a41
  - by: claude-code (hc t382)
- 2026-11-13T01:44:45Z · gate-opened · intent → intent · Approve the intent: mark 0.6.0 published in the roadmap with its install observation ^t-13cd92
  - revision: 443c63ab65b747f47248ad2166f8fb299da94a41
  - holds: intent
  - drives: none
  - by: claude-code (hc t382)
- 2026-11-13T01:44:45Z · gate-closed · intent → intent · Approve the intent: mark 0.6.0 published in the roadmap with its install observation ^t-25e06a
  - revision: 443c63ab65b747f47248ad2166f8fb299da94a41
  - relayed_by:
    - decided_by: Leif
    - words: Yes (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T01:43Z, answering: 'Last release step: mark 0.6.0 as published in Chough's roadmap, plus the install check result, as one small docs PR with no requirement change. OK?' (option text: 't382 opens it. Your merge of it counts as the review.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
  - by: claude-code (hc t382)
- 2026-11-13T01:44:45Z · advance · intent → design · Leif approved the intent (01:43Z, relayed by t370) ^t-e5a00d
  - revision: 443c63ab65b747f47248ad2166f8fb299da94a41
  - by: claude-code (hc t382)
- 2026-11-13T01:44:45Z · skip · design → plan · Documentation-only release row; no visuals or interactions change ^t-be06f2
  - revision: 443c63ab65b747f47248ad2166f8fb299da94a41
  - by: claude-code (hc t382)
- 2026-11-13T01:46:43Z · gate-opened · plan → plan · Approve the scope of mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation ^t-e35c06
  - native_change: mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
  - revision: 443c63ab65b747f47248ad2166f8fb299da94a41
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T01:46:43Z · gate-closed · plan → plan · Approve the scope of mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation ^t-b0a04f
  - native_change: mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
  - revision: 443c63ab65b747f47248ad2166f8fb299da94a41
  - relayed_by:
    - decided_by: Leif
    - words: Yes (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T01:43Z, answering: 'Last release step: mark 0.6.0 as published in Chough's roadmap, plus the install check result, as one small docs PR with no requirement change. OK?' (option text: 't382 opens it. Your merge of it counts as the review.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T01:46:44Z · advance · plan → build · Leif approved the scope (01:43Z, relayed by t370) ^t-a68a7b
  - native_change: mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
  - gate: native:approved
  - revision: 443c63ab65b747f47248ad2166f8fb299da94a41
  - by: claude-code (hc t382)
- 2026-11-13T01:55:43Z · advance · build → test · The roadmap row is written and its docs cases pass ^t-282c48
  - native_change: mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
  - gate: native:approved
  - revision: 443c63ab65b747f47248ad2166f8fb299da94a41
  - by: claude-code (hc t382)
- 2026-11-13T01:55:43Z · advance · test → accept · The roadmap row is written and its docs cases pass ^t-c719ae
  - native_change: mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
  - gate: native:approved
  - revision: 443c63ab65b747f47248ad2166f8fb299da94a41
  - by: claude-code (hc t382)
- 2026-11-13T01:56:00Z · gate-opened · accept → accept · Implementation review of mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation ^t-aa9bb9
  - native_change: mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
  - revision: 443c63ab65b747f47248ad2166f8fb299da94a41
  - holds: accept
  - drives: review
  - by: claude-code (hc t382)
