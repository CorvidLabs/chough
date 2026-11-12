---
kind: bug
id: 2026-11-06-readme-docs-workflow-skill-and-diagrams-drift-from-what-main
aliases:
  - BUG-readme-docs-workflow-skill-and-diagrams-drift-from-what-main-does-c235415b190ff7654315964d
title: README, docs, workflow skill and diagrams drift from what main does
status: verified
classification: product-defect
scope: in-scope
impact: An agent or person following the docs or skill resumes from the wrong files, clears instead of closes the review gate, or uses a stale SpecSync version
criteria: [FLOW-18]
requirements: []
cases:
  - TC-cli-stage-diagram-matches-the-helper
  - TC-cli-inline-diagrams-match-sources
  - TC-cli-readme-helper-table-covers-help
  - TC-cli-readme-verify-list-matches-fledge
  - TC-cli-documented-merge-drivers-match
  - TC-cli-docs-name-pinned-versions
  - TC-cli-closeout-closes-review-gate
  - TC-cli-skill-615b587b-known-outdated
evidence:
  - OBS-tc-cli-closeout-closes-review-gate-f701d940bd72712557fae2f9
  - OBS-tc-cli-docs-name-pinned-versions-2909e52082ea849ecbdd5980
  - OBS-tc-cli-documented-merge-drivers-match-a6a7136235f460d66b8cd4c5
  - OBS-tc-cli-inline-diagrams-match-sources-a2caa50c479ca76178fd5488
  - OBS-tc-cli-readme-helper-table-covers-help-e3e1ed19fd5068c522abfa4c
  - OBS-tc-cli-readme-verify-list-matches-fledge-34c4416330af263a86df70bb
  - OBS-tc-cli-skill-615b587b-known-outdated-1022470cf6daf1cd3999060a
  - OBS-tc-cli-stage-diagram-matches-the-helper-8d60c622b0c484760515ef72
  - OBS-tc-cli-closeout-closes-review-gate-b339c520f241aa7d51419325
  - OBS-tc-cli-doc-links-resolve-d1b7f80e0de48d21d290b46b
  - OBS-tc-cli-docs-name-pinned-versions-649dd7f6872b1a18528a7a8a
  - OBS-tc-cli-documented-flags-exist-f2f748d7e624f91a66ac98ad
  - OBS-tc-cli-documented-merge-drivers-match-09959a868f64ea9e18ed6931
  - OBS-tc-cli-inline-diagrams-match-sources-65edaa71d824af2d092395cc
  - OBS-tc-cli-readme-helper-table-covers-help-1077f9c657e1c00321428c20
  - OBS-tc-cli-readme-verify-list-matches-fledge-b711ae13adcaf9fd2298aabd
  - OBS-tc-cli-skill-615b587b-known-outdated-d50e606ec6e682d8a8bfd7f3
  - OBS-tc-cli-stage-diagram-matches-the-helper-8bb583e87a2fcff6b736e681
  - OBS-tc-cli-closeout-closes-review-gate-60aba156e72e424eea8b1b43
  - OBS-tc-cli-closeout-closes-review-gate-2b35e8a8e6a4584f4d27d6fd
receipts:
  - id: OBS-tc-cli-closeout-closes-review-gate-f701d940bd72712557fae2f9
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-06-tc-cli-docs-name-pinned-versions
    aliases: OBS-tc-cli-docs-name-pinned-versions-2909e52082ea849ecbdd5980
    claim: "Ran cargo test --locked --test docs -- --exact docs_state_only_the_pinned_tool_versions for TC-cli-docs-name-pinned-versions: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact docs_state_only_the_pinned_tool_versions
    revision: 7039f68b5d4a84f4506a53e8d720e7ac24c5d316
    environment: local-macos-arm64
    content: 804c78228734dd5dc1fc0ad7420b856c47349370f70c8447f0b532abcb6b7fa4
    fingerprint: sha256:31e7debc74c98d2552b33975df0877cb126d621f9c2bdf6a15c73af329f1dc02
    case: TC-cli-docs-name-pinned-versions
    case_digest: 404be58afd21d5cd21d3eb5f7faed4cf91ff6ab0d49075d4eac06f227fa89015
    observed: 2026-11-06T15:47:39Z
    duration_seconds: 1
  - id: 2026-11-06-tc-cli-documented-merge-drivers-match
    aliases: OBS-tc-cli-documented-merge-drivers-match-a6a7136235f460d66b8cd4c5
    claim: "Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares
    revision: 7039f68b5d4a84f4506a53e8d720e7ac24c5d316
    environment: local-macos-arm64
    content: 804c78228734dd5dc1fc0ad7420b856c47349370f70c8447f0b532abcb6b7fa4
    fingerprint: sha256:58adcd03ab39f0fe9e234857621ccdb1aad84ffcafec69c6aee8f3bac2a95e81
    case: TC-cli-documented-merge-drivers-match
    case_digest: be40dc133895dbd3617da6f180952bf1aa526c18eb9e7656afda673040129881
    observed: 2026-11-06T15:47:24Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-inline-diagrams-match-sources
    aliases: OBS-tc-cli-inline-diagrams-match-sources-a2caa50c479ca76178fd5488
    claim: "Ran cargo test --locked --test docs -- --exact inline_diagrams_equal_their_sources_and_svgs_carry_every_label for TC-cli-inline-diagrams-match-sources: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact inline_diagrams_equal_their_sources_and_svgs_carry_every_label
    revision: 7039f68b5d4a84f4506a53e8d720e7ac24c5d316
    environment: local-macos-arm64
    content: 804c78228734dd5dc1fc0ad7420b856c47349370f70c8447f0b532abcb6b7fa4
    fingerprint: sha256:fb3b94bfb215984c7f50c34a5856b959dadcfe412afae8a3c991943c60a5f192
    case: TC-cli-inline-diagrams-match-sources
    case_digest: ba2ebb9f9acf2cdbd35c734e3d01bcb2cbfc658dcba5513f5beed0218e10eb54
    observed: 2026-11-06T15:46:08Z
    duration_seconds: 0
  - id: OBS-tc-cli-readme-helper-table-covers-help-e3e1ed19fd5068c522abfa4c
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-06-tc-cli-readme-verify-list-matches-fledge
    aliases: OBS-tc-cli-readme-verify-list-matches-fledge-34c4416330af263a86df70bb
    claim: "Ran cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step for TC-cli-readme-verify-list-matches-fledge: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step
    revision: 7039f68b5d4a84f4506a53e8d720e7ac24c5d316
    environment: local-macos-arm64
    content: 804c78228734dd5dc1fc0ad7420b856c47349370f70c8447f0b532abcb6b7fa4
    fingerprint: sha256:ae6eaa71d5e87f065cf2dcfe60eca4e858346a1856c1e4a5963a79f765b0d698
    case: TC-cli-readme-verify-list-matches-fledge
    case_digest: 8053fe1bd76560e758c7d613849c909d72c620944f864dc5d9bfa8e51fdae130
    observed: 2026-11-06T15:46:35Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-skill-615b587b-known-outdated
    aliases: OBS-tc-cli-skill-615b587b-known-outdated-1022470cf6daf1cd3999060a
    claim: "Ran cargo test --locked --test docs -- --exact skill_files_from_main_615b587b_are_known_outdated_and_edited_copies_stay for TC-cli-skill-615b587b-known-outdated: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact skill_files_from_main_615b587b_are_known_outdated_and_edited_copies_stay
    revision: 7039f68b5d4a84f4506a53e8d720e7ac24c5d316
    environment: local-macos-arm64
    content: 804c78228734dd5dc1fc0ad7420b856c47349370f70c8447f0b532abcb6b7fa4
    fingerprint: sha256:2d2d0b36a9fe8019d8463279bbcedcb132013a0e9a2984d391673e57dd5a398b
    case: TC-cli-skill-615b587b-known-outdated
    case_digest: 18cc8d5a8c9336ab326b33d150964c9f63024c8434db1503f8353dbd7dc50e34
    observed: 2026-11-06T15:48:06Z
    duration_seconds: 1
  - id: 2026-11-06-tc-cli-stage-diagram-matches-the-helper
    aliases: OBS-tc-cli-stage-diagram-matches-the-helper-8d60c622b0c484760515ef72
    claim: "Ran cargo test --locked --test docs -- --exact stage_diagrams_draw_only_the_helpers_stages_and_moves for TC-cli-stage-diagram-matches-the-helper: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact stage_diagrams_draw_only_the_helpers_stages_and_moves
    revision: 7039f68b5d4a84f4506a53e8d720e7ac24c5d316
    environment: local-macos-arm64
    content: 804c78228734dd5dc1fc0ad7420b856c47349370f70c8447f0b532abcb6b7fa4
    fingerprint: sha256:0e5fcbf11a1fbdeb54968c6379c7a276f713dc918cfb920546f4a21f7457afbf
    case: TC-cli-stage-diagram-matches-the-helper
    case_digest: 195d94cc4c4af4256e3e2a8ae7ccdb72e67f12700ca6ab357f4ae6af77018051
    observed: 2026-11-06T15:45:54Z
    duration_seconds: 1
  - id: OBS-tc-cli-closeout-closes-review-gate-b339c520f241aa7d51419325
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-doc-links-resolve-d1b7f80e0de48d21d290b46b
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-docs-name-pinned-versions-649dd7f6872b1a18528a7a8a
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-documented-flags-exist-f2f748d7e624f91a66ac98ad
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-documented-merge-drivers-match-09959a868f64ea9e18ed6931
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-inline-diagrams-match-sources-65edaa71d824af2d092395cc
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-readme-helper-table-covers-help-1077f9c657e1c00321428c20
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-readme-verify-list-matches-fledge-b711ae13adcaf9fd2298aabd
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-skill-615b587b-known-outdated-d50e606ec6e682d8a8bfd7f3
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-06-tc-cli-stage-diagram-matches-the-helper-f7c635
    aliases: OBS-tc-cli-stage-diagram-matches-the-helper-8bb583e87a2fcff6b736e681
    claim: "Ran cargo test --locked --test docs -- --exact stage_diagrams_draw_only_the_helpers_stages_and_moves for TC-cli-stage-diagram-matches-the-helper: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact stage_diagrams_draw_only_the_helpers_stages_and_moves
    revision: de02abd046f1140085215bbb0f3b64f369d0eae1
    environment: local-macos-arm64
    content: e83058e7ddc9242de3b60d0c55c55c6dcf136256e0bd9f5e277696f7d428d248
    fingerprint: sha256:c138b556c1013f81ef0259b498365e5eb56b32fd177aa2e20c24a2076279e8bb
    case: TC-cli-stage-diagram-matches-the-helper
    case_digest: 195d94cc4c4af4256e3e2a8ae7ccdb72e67f12700ca6ab357f4ae6af77018051
    observed: 2026-11-06T16:28:25Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-closeout-closes-review-gate
    aliases: OBS-tc-cli-closeout-closes-review-gate-60aba156e72e424eea8b1b43
    claim: "Ran cargo test --locked --test docs -- --exact closeout_guidance_closes_the_review_gate_with_the_relayed_decision for TC-cli-closeout-closes-review-gate: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact closeout_guidance_closes_the_review_gate_with_the_relayed_decision
    revision: 3cf076e051299c30e1f8b146e4c00c6e9459538b
    environment: local-macos-arm64
    content: a70f09df6fe5a4a80ccdca68efdd31e596ae4c5f6507de59757ec5b004c0ccdb
    fingerprint: sha256:e1c8e40c7f40d1be514c2a13e8500580533e7de70b945580959d3c044fdd39dd
    case: TC-cli-closeout-closes-review-gate
    case_digest: 97a61423dadaa094eab715faeb0f1087bd99c7afb7ab96732e4cedf39aa01313
    observed: 2026-11-06T16:43:37Z
    duration_seconds: 0
  - id: OBS-tc-cli-closeout-closes-review-gate-2b35e8a8e6a4584f4d27d6fd
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
environment: local-macos-arm64
revision: 7039f68b5d4a84f4506a53e8d720e7ac24c5d316
found_by: ch-docs-accuracy audit, ../ch-docs-findings.md
related: []
created: 2026-11-06T15:48:27Z
updated: 2026-11-06T16:44:08Z
---
# README, docs, workflow skill and diagrams drift from what main does

**What happened:** Docs, the workflow skill and two diagrams describe behavior main no longer has: a skip edge from intent to plan, a test to accept gate without the unlinked failure, 'Optional librarian', 'clear the gate', SpecSync 6.1 and 6.1.2 as current, a README helper table without history, session, inbox, bug merge and skill remove-legacy, a cli.md .gitattributes block missing four lines, and the 615b587b skill files reading as current
Actual: 8 of 10 checks in tests/docs.rs fail; the flag and link checks pass

**Steps:** cargo test --locked --test docs at 7039f68b (base 615b587b plus the FLOW-18 checks)
1. cargo test --locked --test docs

**Expected:** tests/docs.rs passes: every page, the skill and the diagrams match the helper, fledge.toml, .gitattributes and the CI pins (FLOW-18)

**Why it matters:** An agent or person following the docs or skill resumes from the wrong files, clears instead of closes the review gate, or uses a stale SpecSync version

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-06T15:48:27Z · Reported (ch-docs-accuracy audit, ../ch-docs-findings.md); evidence: OBS-tc-cli-closeout-closes-review-gate-f701d940bd72712557fae2f9, OBS-tc-cli-docs-name-pinned-versions-2909e52082ea849ecbdd5980, OBS-tc-cli-documented-merge-drivers-match-a6a7136235f460d66b8cd4c5, OBS-tc-cli-inline-diagrams-match-sources-a2caa50c479ca76178fd5488, OBS-tc-cli-readme-helper-table-covers-help-e3e1ed19fd5068c522abfa4c, OBS-tc-cli-readme-verify-list-matches-fledge-34c4416330af263a86df70bb, OBS-tc-cli-skill-615b587b-known-outdated-1022470cf6daf1cd3999060a, OBS-tc-cli-stage-diagram-matches-the-helper-8d60c622b0c484760515ef72
- 2026-11-06T16:29:02Z · status: open -> verified. Fixed by de02abd0: tests/docs.rs passes 10 of 10 at the materialized change; all ten FLOW-18 cases passed on local-macos-arm64 after the failures of 7039f68b; evidence: OBS-tc-cli-closeout-closes-review-gate-b339c520f241aa7d51419325, OBS-tc-cli-doc-links-resolve-d1b7f80e0de48d21d290b46b, OBS-tc-cli-docs-name-pinned-versions-649dd7f6872b1a18528a7a8a, OBS-tc-cli-documented-flags-exist-f2f748d7e624f91a66ac98ad, OBS-tc-cli-documented-merge-drivers-match-09959a868f64ea9e18ed6931, OBS-tc-cli-inline-diagrams-match-sources-65edaa71d824af2d092395cc, OBS-tc-cli-readme-helper-table-covers-help-1077f9c657e1c00321428c20, OBS-tc-cli-readme-verify-list-matches-fledge-b711ae13adcaf9fd2298aabd, OBS-tc-cli-skill-615b587b-known-outdated-d50e606ec6e682d8a8bfd7f3, OBS-tc-cli-stage-diagram-matches-the-helper-8bb583e87a2fcff6b736e681
- 2026-11-06T16:43:44Z · status: verified -> open. Reopened: the strengthened closeout check (3cf076e0) finds "clear gate" in the skill's lifecycle reference and its .agents copy; evidence: OBS-tc-cli-closeout-closes-review-gate-60aba156e72e424eea8b1b43
- 2026-11-06T16:44:08Z · status: open -> verified. Fixed: the lifecycle reference's change shape says close gate (--close-gate); the closeout check passes; evidence: OBS-tc-cli-closeout-closes-review-gate-2b35e8a8e6a4584f4d27d6fd
