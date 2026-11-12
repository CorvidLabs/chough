---
kind: bug
id: 2026-11-02-native-workflow-guidance-exceeds-the-bounded-skill-entrypoint
aliases:
  - BUG-native-workflow-guidance-exceeds-the-bounded-skill-entrypoint-0c432bf829e08bf33dfc41b9
title: Native workflow guidance exceeds the bounded skill entrypoint
status: verified
classification: product-defect
scope: in-scope
impact: Blocks required package verification and increases resume context
criteria: [LIFE-3]
requirements: [REQ-cli-029]
cases: []
evidence:
  - OBS-product-test-17dea5d3d3022872bfc0784f
  - OBS-product-test-472fe5550a1dbc62492fe07a
  - OBS-product-test-89d0bc200d842657b5c0c5dd
receipts:
  - id: 2026-11-02-required-fledge-verification-and-rust1-88-qualification-both
    aliases: OBS-product-test-17dea5d3d3022872bfc0784f
    claim: "Required Fledge verification and Rust1.88 qualification both failed the unchanged existing package assertion: SKILL.md was7115bytes, exceeding its6144byte limit. The handoff step assertions and limit were not changed."
    result: failed
    revision: 825a3805530456005e361dfa6d96d93eee52cd09
    environment: local-macos-arm64
    content: e3fd39f88a024fa12257d83d3094f6877916766529fd4f5890f6cf072064f05d
    fingerprint: sha256:e6e415a7f3bdeefc9555e0153407f9d143ac7cc666e5cd5461fa012e6e166b43
    observed: 2026-11-02T16:22:07Z
  - id: 2026-11-02-the-unchanged-handoff-6144-byte-package-test-and-real-bundle
    aliases: OBS-product-test-472fe5550a1dbc62492fe07a
    claim: The unchanged handoff/6144-byte package test and real bundle-upgrade regression both pass after the main skill was condensed to 6121 bytes. The handoff step, size assertion, native authority rules and edited-copy protection remain intact.
    result: passed
    revision: 825a3805530456005e361dfa6d96d93eee52cd09
    environment: local-macos-arm64
    content: 0144dd6eb3d9ce9a9265f74835a307d50f9702355f5c2f6cd65c9f1765980b94
    fingerprint: sha256:c6346b1082ae9b1957126a96db923c70bcc96ce78f2dd2ab9e4c126ec3f7d385
    observed: 2026-11-02T16:24:53Z
  - id: OBS-product-test-89d0bc200d842657b5c0c5dd
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
environment: local-macos-arm64
revision: 825a3805530456005e361dfa6d96d93eee52cd09
found_by: Required qualification in the issue42 implementation
related: []
created: 2026-11-02T16:22:09Z
updated: 2026-11-02T16:35:08Z
---
# Native workflow guidance exceeds the bounded skill entrypoint

**What happened:** The shipped main skill exceeds its existing6144byte bound
Actual: The main skill is7115bytes and the unchanged package assertion fails on both Rust toolchains

**Steps:** The new orchestration section adds detailed policy to the main skill

**Expected:** Keep the existing handoff step and total main skill below6144bytes, with details in the lifecycle reference

**Why it matters:** Blocks required package verification and increases resume context

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T16:22:09Z · Reported (Required qualification in the issue42 implementation); evidence: OBS-product-test-17dea5d3d3022872bfc0784f
- 2026-11-02T16:24:54Z · status: open -> verified. Condensed the main entrypoint and kept detailed authority/provenance rules in lifecycle.md; the original handoff and 6144-byte assertions remain unchanged.; evidence: OBS-product-test-472fe5550a1dbc62492fe07a
- 2026-11-02T16:35:08Z · The final full verify lane also passes the unchanged size/handoff assertion and upgrade regression; all197 tests passed.; evidence: OBS-product-test-89d0bc200d842657b5c0c5dd
