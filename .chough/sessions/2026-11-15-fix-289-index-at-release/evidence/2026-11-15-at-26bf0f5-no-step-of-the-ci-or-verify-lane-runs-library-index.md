---
kind: evidence
id: 2026-11-15-at-26bf0f5-no-step-of-the-ci-or-verify-lane-runs-library-index
aliases: []
claim: "At 26bf0f5: no step of the ci or verify lane runs library index; release-check rebuilds then runs the strict --check (and passes on this tree); a hand-edited lesson count with its link kept fails --check naming knowledge/index.md. Full suite 762 passed; fmt and lint clean."
result: passed
evidence_kind: product-test
method: "cargo test --locked --test package no_ci_or_verify_lane_step_runs_the_knowledge_check; cargo test --locked --test vault a_hand_edited_lesson_count_fails_the_check; fledge run release-check; plus mutation checks: adding a knowledge step to the verify lane, or swapping release-check's order, makes the lane test fail"
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:244b2d5e93ce083992aa3e0b0d3446623f465e4f83bc2178e7bf35b9c12f4aa7
external: ext-73a833
criteria: [KNOW-4]
requirements: [REQ-cli-047, REQ-cli-043]
contracts: []
limitations:
  - The lane test reads fledge.toml text; a lane step that reaches library index through a nested fledge run is not followed
observed: 2026-11-15T20:23:10Z
actor:
  - agent: claude-code
    terminal: t28
---
**At 26bf0f5: no step of the ci or verify lane runs library index; release-check rebuilds then runs the strict --check (and passes on this tree); a hand-edited lesson count with its link kept fails --check naming knowledge/index.md. Full suite 762 passed; fmt and lint clean.**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:23:10Z.
