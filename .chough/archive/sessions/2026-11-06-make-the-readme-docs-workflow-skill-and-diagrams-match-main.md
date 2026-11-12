---
kind: session
id: 2026-11-06-make-the-readme-docs-workflow-skill-and-diagrams-match-main
aliases:
  - make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
objective: Make the README, docs, workflow skill and diagrams match main, with checks that fail when they drift (FLOW-18)
intent_scope: FLOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
pr: ""
criteria: [FLOW-18, FLOW-13, FLOW-16, FLOW-2]
requirements: []
cases:
  - TC-cli-stage-diagram-matches-the-helper
  - TC-cli-inline-diagrams-match-sources
  - TC-cli-readme-helper-table-covers-help
  - TC-cli-readme-verify-list-matches-fledge
  - TC-cli-documented-flags-exist
  - TC-cli-doc-links-resolve
  - TC-cli-documented-merge-drivers-match
  - TC-cli-docs-name-pinned-versions
  - TC-cli-closeout-closes-review-gate
  - TC-cli-skill-615b587b-known-outdated
design: []
files: []
observations:
  - OBS-tc-cli-closeout-closes-review-gate-f701d940bd72712557fae2f9
  - OBS-tc-cli-doc-links-resolve-a914604eb66262d26986c932
  - OBS-tc-cli-docs-name-pinned-versions-2909e52082ea849ecbdd5980
  - OBS-tc-cli-documented-flags-exist-c93c273024940d8058480e32
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
  - OBS-tc-cli-closeout-closes-review-gate-cd10b909d16761f96c0f38f5
  - OBS-tc-cli-doc-links-resolve-9692f41fb7a6065250612657
  - OBS-tc-cli-docs-name-pinned-versions-7e78ff292af0e904bf82da7b
  - OBS-tc-cli-documented-flags-exist-aaa7b6981b174c0d0b11f192
  - OBS-tc-cli-documented-merge-drivers-match-896b1ba90244d08fe9b7d0dc
  - OBS-tc-cli-inline-diagrams-match-sources-55fef8ab48aa2dc94c3117df
  - OBS-tc-cli-readme-helper-table-covers-help-224c0ab2a60e621f228f3ca7
  - OBS-tc-cli-readme-verify-list-matches-fledge-d13dd6b7d5df8164ce7a5e9c
  - OBS-tc-cli-skill-615b587b-known-outdated-1c6bc1821e91c3d7b6231c86
  - OBS-tc-cli-stage-diagram-matches-the-helper-688bc12b578de1eb30607fb1
  - OBS-tc-cli-closeout-closes-review-gate-60aba156e72e424eea8b1b43
  - OBS-tc-cli-closeout-closes-review-gate-2b35e8a8e6a4584f4d27d6fd
bugs:
  - BUG-readme-docs-workflow-skill-and-diagrams-drift-from-what-main-does-c235415b190ff7654315964d
blockers: []
waiting_on_human: []
run_consents: []
next: "After Leif's re-review (recorded by orc): ship, close the review gate with his relayed words, stage advance, commit, audit --strict and pre-push, push, CI, merge pinned"
closed: 2026-11-06
outcome: done
receipts:
  - id: OBS-tc-cli-closeout-closes-review-gate-f701d940bd72712557fae2f9
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-doc-links-resolve-a914604eb66262d26986c932
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
  - id: OBS-tc-cli-documented-flags-exist-c93c273024940d8058480e32
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
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
  - id: OBS-tc-cli-closeout-closes-review-gate-cd10b909d16761f96c0f38f5
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-doc-links-resolve-9692f41fb7a6065250612657
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-docs-name-pinned-versions-7e78ff292af0e904bf82da7b
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-documented-flags-exist-aaa7b6981b174c0d0b11f192
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-documented-merge-drivers-match-896b1ba90244d08fe9b7d0dc
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-inline-diagrams-match-sources-55fef8ab48aa2dc94c3117df
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-readme-helper-table-covers-help-224c0ab2a60e621f228f3ca7
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-readme-verify-list-matches-fledge-d13dd6b7d5df8164ce7a5e9c
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-skill-615b587b-known-outdated-1c6bc1821e91c3d7b6231c86
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-stage-diagram-matches-the-helper-688bc12b578de1eb30607fb1
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
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
---
# Make the README, docs, workflow skill and diagrams match main, with checks that fail when they drift (FLOW-18)

## Goal

Make the README, docs, workflow skill and diagrams match main, with checks that fail when they drift (FLOW-18)

## Where it stands

- Failing tests: 7039f68b (tests/docs.rs, 8 of 10 FLOW-18 checks fail on the base; flags and links pass)
- Docs, skill and diagrams corrected; stages, lifecycle and ownership re-rendered with Mermaid 11.12.0 and PNGs inspected; tests/docs.rs passes 10 of 10; cargo test 487 passed
- Passing run: ten FLOW-18 cases passed on local-macos-arm64 after the fix; bug verified
- Merged main (#149, #157, #159, #151) and reran the ten FLOW-18 cases on the merged tree: all passed
- Confirmed the archive-first order orc relayed (local verify, human review, ship archive commit, push/CI) in the skill and docs; fixed the skill's 'clear gate' step name failing first
- Reopened for #166's session close (Leif, 'Reopen + one-word fix'): the README helper row names close; the README case failed first at 15cdaef2 and passes, and all ten FLOW-18 cases pass on the merged tree
- After merging #168 (orc: 'yes, add the doctor row ... and also add close to docs/cli.md's session command row'): the README case failed first at e017ebe5 ('no row names doctor'), the README helper table has a doctor row and cli.md's session row names close; all ten FLOW-18 cases and cargo test (532) pass

## Decisions and why

- Leif (0xLeif) decided "Intent approval of FLOW-18 (docs accuracy)": "FLOW-18, one PR" (orc tab, 2026-11-06 morning MST, relayed by orc (hc t18) to ch-docs-accuracy: capture FLOW-18 exactly as proposed; skill fixes and their KNOWN_PREVIOUS rows in this same docs PR, appended at the end clear of #151; failing checks first; leave cli.md:25 to #158; do not touch #69's lines); recorded by claude (ch-docs-accuracy); a relayed claim, not authenticated
- decided: one PR holds the docs, skill and diagram fixes with their KNOWN_PREVIOUS rows because Leif chose "FLOW-18, one PR"
- rejected: splitting the workflow skill fixes into another pull request because Leif chose one PR
- rejected: capturing the criterion as LIFE-17 because it would edit the same INTENT.md count line as #160 (LIFE-16)
- decided: KNOWN_PREVIOUS rows go at the end of the list because #151 inserts at its top
- decided: docs/cli.md:25 (bug row, bug merge) is left to #158 and #69's lines are not touched because those pull requests change them
- Leif (0xLeif) decided "Scope approval of make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18": "Approve" (orc tab, 2026-11-06, relayed by orc (hc t18) to ch-docs-accuracy; orc recorded the native definition approval as 0xLeif at c8a74aff); recorded by claude (ch-docs-accuracy); a relayed claim, not authenticated
- decided: commit the rebuilt knowledge index after the passing runs because CI's library index --check needs it, and report those observations as historical by #136 rather than loop reruns
- Leif (0xLeif) decided "Implementation review of make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18": "pass" (orc tab, 2026-11-06, relayed by orc (hc t18) to ch-docs-accuracy; orc recorded the native scoped review as 0xLeif, verdict pass at 24cb2231 on PR #164); recorded by claude (ch-docs-accuracy); a relayed claim, not authenticated
- Leif (0xLeif) decided to reopen FLOW-18 for the README helper row: "Reopen + one-word fix" (orc tab, 2026-11-06, relayed by orc (hc t18) to ch-docs-accuracy; orc recorded the SpecSync reopen as 0xLeif, 'reopened for fresh verification'); recorded by claude (ch-docs-accuracy); a relayed claim, not authenticated
- Leif (0xLeif) decided "Implementation review of make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18": "pass" (orc tab, 2026-11-06, relayed by orc (hc t18) to ch-docs-accuracy; re-review of the FLOW-18 reopen diff (README close and doctor rows, cli.md session row); orc recorded the native scoped review as 0xLeif, verdict pass at ae62fafe on PR #164); recorded by claude (ch-docs-accuracy); a relayed claim, not authenticated

## What was verified

- OBS-tc-cli-closeout-closes-review-gate-f701d940bd72712557fae2f9
- OBS-tc-cli-doc-links-resolve-a914604eb66262d26986c932
- OBS-tc-cli-docs-name-pinned-versions-2909e52082ea849ecbdd5980
- OBS-tc-cli-documented-flags-exist-c93c273024940d8058480e32
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
- OBS-tc-cli-closeout-closes-review-gate-cd10b909d16761f96c0f38f5
- OBS-tc-cli-doc-links-resolve-9692f41fb7a6065250612657
- OBS-tc-cli-docs-name-pinned-versions-7e78ff292af0e904bf82da7b
- OBS-tc-cli-documented-flags-exist-aaa7b6981b174c0d0b11f192
- OBS-tc-cli-documented-merge-drivers-match-896b1ba90244d08fe9b7d0dc
- OBS-tc-cli-inline-diagrams-match-sources-55fef8ab48aa2dc94c3117df
- OBS-tc-cli-readme-helper-table-covers-help-224c0ab2a60e621f228f3ca7
- OBS-tc-cli-readme-verify-list-matches-fledge-d13dd6b7d5df8164ce7a5e9c
- OBS-tc-cli-skill-615b587b-known-outdated-1c6bc1821e91c3d7b6231c86
- OBS-tc-cli-stage-diagram-matches-the-helper-688bc12b578de1eb30607fb1
- OBS-tc-cli-closeout-closes-review-gate-60aba156e72e424eea8b1b43
- OBS-tc-cli-closeout-closes-review-gate-2b35e8a8e6a4584f4d27d6fd

## Open threads

## Timeline

- 2026-11-06T15:31:32Z · gate-opened · intent → intent · Intent approval of FLOW-18 (docs accuracy) ^t-fc80f1
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - revision: 615b587b33a1ef3a569f9557e2782a6be3df2f0f
- 2026-11-06T15:31:32Z · gate-closed · intent → intent · Intent approval of FLOW-18 (docs accuracy) ^t-c73be9
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - revision: 615b587b33a1ef3a569f9557e2782a6be3df2f0f
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"FLOW-18, one PR","where":"orc tab, 2026-11-06 morning MST, relayed by orc (hc t18) to ch-docs-accuracy: capture FLOW-18 exactly as proposed; skill fixes and their KNOWN_PREVIOUS rows in this same docs PR, appended at the end clear of #151; failing checks first; leave cli.md:25 to #158; do not touch #69's lines","recorded_by":"claude (ch-docs-accuracy)"}
- 2026-11-06T15:31:32Z · advance · intent → design · Intent approved as a whole by Leif's relayed choice "FLOW-18, one PR" ^t-530df6
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - revision: 615b587b33a1ef3a569f9557e2782a6be3df2f0f
- 2026-11-06T15:31:32Z · skip · design → plan · No product visuals or interactions change; the Mermaid diagrams are documentation, re-rendered with docs/diagrams/render.mjs and inspected as docs/verification.md describes ^t-3a0d2c
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - revision: 615b587b33a1ef3a569f9557e2782a6be3df2f0f
- 2026-11-06T15:48:35Z · gate-opened · plan → plan · Scope approval of make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18 ^t-35c72c
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - revision: 7039f68b5d4a84f4506a53e8d720e7ac24c5d316
- 2026-11-06T15:52:43Z · gate-closed · plan → plan · Scope approval of make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18 ^t-ecf85b
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - revision: c8a74aff6672d252eb0e3441c9bb432f40242b73
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"Approve","where":"orc tab, 2026-11-06, relayed by orc (hc t18) to ch-docs-accuracy; orc recorded the native definition approval as 0xLeif at c8a74aff","recorded_by":"claude (ch-docs-accuracy)"}
- 2026-11-06T15:52:45Z · advance · plan → build · Scope approved: Leif's relayed 'Approve'; native definition approval by 0xLeif at c8a74aff ^t-f8b686
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - gate: native:approved
  - revision: c8a74aff6672d252eb0e3441c9bb432f40242b73
- 2026-11-06T16:28:22Z · advance · build → test · Build done: docs, skill and diagrams corrected; materialized; verify lane and strict audit pass ^t-95c8a1
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - gate: native:verifying
  - revision: de02abd046f1140085215bbb0f3b64f369d0eae1
- 2026-11-06T16:29:14Z · advance · test → accept · Ten FLOW-18 cases passed on local-macos-arm64; failing-first bug verified ^t-406078
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - gate: native:verifying
  - revision: de02abd046f1140085215bbb0f3b64f369d0eae1
- 2026-11-06T16:29:46Z · gate-opened · accept → accept · Implementation review of make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18 ^t-75ae84
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - revision: de02abd046f1140085215bbb0f3b64f369d0eae1
- 2026-11-06T18:07:28Z · gate-closed · accept → accept · Implementation review of make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18 ^t-6a8828
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - revision: 4c3937231ba64326d253aaea79ed40c0b19ce78f
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"pass","where":"orc tab, 2026-11-06, relayed by orc (hc t18) to ch-docs-accuracy; orc recorded the native scoped review as 0xLeif, verdict pass at 24cb2231 on PR #164","recorded_by":"claude (ch-docs-accuracy)"}
- 2026-11-06T18:07:31Z · advance · accept → done · native:archived ^t-353981
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - gate: native:archived
  - revision: 4c3937231ba64326d253aaea79ed40c0b19ce78f
- 2026-11-06T20:12:45Z · back · done → build · Leif reopened FLOW-18 ('Reopen + one-word fix', relayed by orc hc t18; SpecSync reopen recorded as 0xLeif, 'reopened for fresh verification'): after merging main, #166's session close is missing from the README helper row and tests/docs.rs fails ^t-2cb03e
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - gate: native:verifying
  - revision: 15cdaef2c48f5ce02f9fd5caaa6fdf5dc246f8d4
  - refresh: build: implementation against the current definition, test: rerun affected cases; earlier observations may be stale, accept: native verification and the human implementation review, checkpoint references: 10 case(s) and 32 observation(s) to re-check with `chough status`
- 2026-11-06T20:16:47Z · advance · build → test · native:verifying ^t-93773f
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - gate: native:verifying
  - revision: c09f3ed40a28467ee77503c1c82d78c1fa7e6b78
- 2026-11-06T20:17:46Z · advance · test → accept · native:verifying ^t-e7a9a6
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - gate: native:verifying
  - revision: c09f3ed40a28467ee77503c1c82d78c1fa7e6b78
- 2026-11-06T20:18:14Z · gate-opened · accept → accept · Implementation review of make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18 ^t-b7bbf5
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - revision: c09f3ed40a28467ee77503c1c82d78c1fa7e6b78
- 2026-11-06T20:46:36Z · gate-closed · accept → accept · Implementation review of make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18 ^t-fed602
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - revision: 414e414646d6cbf301a9679396f8781e58dce91d
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"pass","where":"orc tab, 2026-11-06, relayed by orc (hc t18) to ch-docs-accuracy; re-review of the FLOW-18 reopen diff (README close and doctor rows, cli.md session row); orc recorded the native scoped review as 0xLeif, verdict pass at ae62fafe on PR #164","recorded_by":"claude (ch-docs-accuracy)"}
- 2026-11-06T20:47:22Z · advance · accept → done · native:archived ^t-c530eb
  - native_change: make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18
  - gate: native:archived
  - revision: 414e414646d6cbf301a9679396f8781e58dce91d


Closed 2026-11-06: done.
