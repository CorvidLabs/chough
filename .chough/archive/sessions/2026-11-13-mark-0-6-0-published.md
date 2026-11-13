---
kind: session
id: 2026-11-13-mark-0-6-0-published
aliases: [mark-0-6-0-published]
objective: Mark the 0.6.0 release published in the roadmap with its install observation (releasing.md step 9)
stage: done
stage_status: archived
iteration: 1
native_change: mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
criteria: [LIFE-25]
cases:
  - TC-cli-native-workflow-bundle-consistency
  - TC-cli-documented-flags-exist
  - TC-cli-doc-links-resolve
  - TC-cli-docs-name-pinned-versions
  - TC-cli-roadmap-claims-no-cold-start-pass
observations:
  - 2026-11-13-clean-cargo-install-chough-version-0-6-0-locked-from-crates-io
  - 2026-11-13-tc-cli-native-workflow-bundle-consistency
  - 2026-11-13-tc-cli-documented-flags-exist
  - 2026-11-13-tc-cli-doc-links-resolve
  - 2026-11-13-tc-cli-docs-name-pinned-versions
  - 2026-11-13-tc-cli-roadmap-claims-no-cold-start-pass
title: Chough 0.6.0 marked published in the roadmap
summary:
  - "releasing.md step 9 for Chough 0.6.0: the roadmap's delivered table now lists the 0.6.0 release, tag v0.6.0 at 443c63ab, crates.io chough 0.6.0 and the GitHub release, with the clean install that printed chough 0.6.0."
  - "Verified: the docs cases for the roadmap pass, and the install is recorded as the coordinator's relayed observation. Next: Leif's merge."
closed: 2026-11-13
outcome: done
---
# Mark the 0.6.0 release published in the roadmap with its install observation (releasing.md step 9)

## Goal

Mark the 0.6.0 release published in the roadmap with its install observation (releasing.md step 9)

## Where it stands

- docs/roadmap-1.0.md: the 0.6.0 release row replaces the in-progress row; the install observation recorded as t370's relayed run; the five automated docs cases selected for the file pass

## Decisions and why

- Leif decided "Approve the intent: mark 0.6.0 published in the roadmap with its install observation": "Yes (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T01:43Z, answering: 'Last release step: mark 0.6.0 as published in Chough's roadmap, plus the install check result, as one small docs PR with no requirement change. OK?' (option text: 't382 opens it. Your merge of it counts as the review.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation": "Yes (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T01:43Z, answering: 'Last release step: mark 0.6.0 as published in Chough's roadmap, plus the install check result, as one small docs PR with no requirement change. OK?' (option text: 't382 opens it. Your merge of it counts as the review.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation as 0xLeif
- Leif decided "Implementation review of mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation": "Approve (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T01:57Z, answering: 'Approve PR #240, the roadmap row marking 0.6.0 published? After that, t382 ships it, CI goes green, and you say merge it.' (option text: 't382 records your review through Chough and ships. Then CI runs, then merge it.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change review mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation as 0xLeif with verdict pass

## What was verified

- Clean cargo install chough --version 0.6.0 --locked from crates.io into an empty root with a fresh target directory on macOS arm64 succeeded; the installed chough --version printed chough 0.6.0 ^2026-11-13-clean-cargo-install-chough-version-0-6-0-locked-from-crates-io
- Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1). — 6973dee04134 ^2026-11-13-tc-cli-native-workflow-bundle-consistency
- Ran cargo test --locked --test docs -- --exact documented_chough_commands_and_flags_exist for TC-cli-documented-flags-exist: passed (exit 0, tests run 1). — f2828bad028d ^2026-11-13-tc-cli-documented-flags-exist
- Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1). — ed078999e3cd ^2026-11-13-tc-cli-doc-links-resolve
- Ran cargo test --locked --test docs -- --exact docs_state_only_the_pinned_tool_versions for TC-cli-docs-name-pinned-versions: passed (exit 0, tests run 1). — 6758fc4563aa ^2026-11-13-tc-cli-docs-name-pinned-versions
- Ran cargo test --locked --test workflow_authority -- --exact the_roadmap_claims_no_passing_cold_start_for_delivered_work for TC-cli-roadmap-claims-no-cold-start-pass: passed (exit 0, tests run 1). — 1ddd713e93c8 ^2026-11-13-tc-cli-roadmap-claims-no-cold-start-pass

## Bugs

None.

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
- 2026-11-13T01:58:38Z · gate-closed · accept → accept · Implementation review of mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation ^t-e492b7
  - native_change: mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
  - revision: 25cacc8bfaea9578b8d616eeeb093cd73d53ada0
  - relayed_by:
    - decided_by: Leif
    - words: Approve (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T01:57Z, answering: 'Approve PR #240, the roadmap row marking 0.6.0 published? After that, t382 ships it, CI goes green, and you say merge it.' (option text: 't382 records your review through Chough and ships. Then CI runs, then merge it.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change review mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T01:58:40Z · advance · accept → done · native:archived ^t-d400a4
  - native_change: mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
  - gate: native:archived
  - revision: 25cacc8bfaea9578b8d616eeeb093cd73d53ada0
  - by: claude-code (hc t382)

Closed 2026-11-13: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-13-clean-cargo-install-chough-version-0-6-0-locked-from-crates-io
    claim: Clean cargo install chough --version 0.6.0 --locked from crates.io into an empty root with a fresh target directory on macOS arm64 succeeded; the installed chough --version printed chough 0.6.0
    result: passed
    revision: 443c63ab65b747f47248ad2166f8fb299da94a41
    environment: macos-arm64-crates-io
    content: b056982fd067aff750b535081ae04bbaf4da1742935833f4a5a8559c6a739214
    observed: 2026-11-13T01:47:22Z
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-native-workflow-bundle-consistency
    claim: "Ran cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent for TC-cli-native-workflow-bundle-consistency: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test workflow_authority -- --exact native_writer_examples_and_installed_workflows_are_consistent
    revision: 443c63ab65b747f47248ad2166f8fb299da94a41
    environment: local-macos-arm64
    content: b056982fd067aff750b535081ae04bbaf4da1742935833f4a5a8559c6a739214
    fingerprint: sha256:6973dee0413442e569a2efe11b3460c69e26a2849389082a2b475bd599fad134
    case: TC-cli-native-workflow-bundle-consistency
    case_digest: 519ac533c4882bb4da71e97a9cae2d9ec1c2a93425652f94f90ec555939a8e72
    observed: 2026-11-13T01:55:07Z
    duration_seconds: 2
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-documented-flags-exist
    claim: "Ran cargo test --locked --test docs -- --exact documented_chough_commands_and_flags_exist for TC-cli-documented-flags-exist: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact documented_chough_commands_and_flags_exist
    revision: 443c63ab65b747f47248ad2166f8fb299da94a41
    environment: local-macos-arm64
    content: b056982fd067aff750b535081ae04bbaf4da1742935833f4a5a8559c6a739214
    fingerprint: sha256:f2828bad028d83289bd60a55cb1432dfa06d073c6809afcb6ac521191241115b
    case: TC-cli-documented-flags-exist
    case_digest: ff7a879860dfb3617227b405f954d0b77861c0f8995eeb3c0c37b8e0fe352347
    observed: 2026-11-13T01:55:12Z
    duration_seconds: 2
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-doc-links-resolve
    claim: "Ran cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve for TC-cli-doc-links-resolve: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact relative_links_and_anchors_resolve
    revision: 443c63ab65b747f47248ad2166f8fb299da94a41
    environment: local-macos-arm64
    content: b056982fd067aff750b535081ae04bbaf4da1742935833f4a5a8559c6a739214
    fingerprint: sha256:ed078999e3cd9da2e2f63a35c288944bb5114ee96093bb445c23099feb48040d
    case: TC-cli-doc-links-resolve
    case_digest: bc8019ca3cf835f594fa494d827d7c34b8f2e9a77993c9cf1c379afc62c4fef5
    observed: 2026-11-13T01:55:14Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-docs-name-pinned-versions
    claim: "Ran cargo test --locked --test docs -- --exact docs_state_only_the_pinned_tool_versions for TC-cli-docs-name-pinned-versions: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact docs_state_only_the_pinned_tool_versions
    revision: 443c63ab65b747f47248ad2166f8fb299da94a41
    environment: local-macos-arm64
    content: b056982fd067aff750b535081ae04bbaf4da1742935833f4a5a8559c6a739214
    fingerprint: sha256:6758fc4563aab6689ce2460d80881ed08b50477bef200c06a302d6ef43996cc2
    case: TC-cli-docs-name-pinned-versions
    case_digest: 404be58afd21d5cd21d3eb5f7faed4cf91ff6ab0d49075d4eac06f227fa89015
    observed: 2026-11-13T01:55:16Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-roadmap-claims-no-cold-start-pass
    claim: "Ran cargo test --locked --test workflow_authority -- --exact the_roadmap_claims_no_passing_cold_start_for_delivered_work for TC-cli-roadmap-claims-no-cold-start-pass: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test workflow_authority -- --exact the_roadmap_claims_no_passing_cold_start_for_delivered_work
    revision: 443c63ab65b747f47248ad2166f8fb299da94a41
    environment: local-macos-arm64
    content: b056982fd067aff750b535081ae04bbaf4da1742935833f4a5a8559c6a739214
    fingerprint: sha256:1ddd713e93c88bf13482d680f3c196943081091234fb1a55c432185dd70524ea
    case: TC-cli-roadmap-claims-no-cold-start-pass
    case_digest: ceab35ad50f536bfa9c7cbd33cff7e8e1b5a6015ab16a6e4cd4caba7691cebb6
    observed: 2026-11-13T01:55:19Z
    duration_seconds: 0
    actor: claude-code (hc t382)
```
