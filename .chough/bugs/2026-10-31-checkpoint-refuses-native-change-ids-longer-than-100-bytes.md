---
kind: bug
id: 2026-10-31-checkpoint-refuses-native-change-ids-longer-than-100-bytes
aliases: [BUG-0013]
title: Checkpoint refuses native change IDs longer than 100 bytes that SpecSync mints
status: verified
classification: product-defect
scope: in-scope
impact: Long-described native changes cannot be checkpointed; every gated stage move refuses as no-native-change
criteria: [FLOW-13]
requirements: [REQ-cli-007, REQ-cli-008]
cases: []
evidence: [bug-0013-repro-1, bug-0013-fixed-1]
receipts:
  - id: 2026-10-31-chough-checkpoint-native-change-refused-a-117-byte-id-that
    aliases: bug-0013-repro-1
    claim: chough checkpoint --native-change refused a 117-byte ID that SpecSync minted and reads, so the checkpoint cannot name the change and gates refuse as no-native-change
    result: failed
    revision: 3596a10482def000d78e160cde0f7fa3cbaccbf0
    environment: local-macos-arm64
    fingerprint: sha256:9035061a0ae828bee66eb84382955fbcc201ceaae1d20696f32b87f918b5dceb
    observed: 2026-10-31T23:08:33Z
  - id: 2026-10-31-chough-checkpoint-native-change-records-the-117-byte-release
    aliases: bug-0013-fixed-1
    claim: chough checkpoint --native-change records the 117-byte release-candidate ID after the fix
    result: passed
    revision: 3b0b2f710131f6a444660953d44e378cb9a87713
    environment: local-macos-arm64
    fingerprint: sha256:4805148d1bf8208b578c752ac13089f6ec65d26a336a3e629f83544bb6e8fab9
    observed: 2026-10-31T23:10:20Z
environment: local-macos-arm64
revision: 3596a10482def000d78e160cde0f7fa3cbaccbf0
found_by: Found while checkpointing the release candidate, 2026-10-31
related: []
created: 2026-10-31T23:08:33Z
updated: 2026-10-31T23:10:21Z
---
# Checkpoint refuses native change IDs longer than 100 bytes that SpecSync mints

**What happened:** chough checkpoint --native-change <117-byte id> fails with invalid native change ID
Actual: valid_name caps every record name at 100 bytes

**Steps:** A SpecSync change whose description slug exceeds 100 bytes (SpecSync mints up to 120)
1. specsync change new with a long description
2. chough checkpoint --native-change <id>

**Expected:** Native change IDs are accepted up to SpecSync's own ceiling with SpecSync's character rules

**Why it matters:** Long-described native changes cannot be checkpointed; every gated stage move refuses as no-native-change

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T23:08:33Z · Reported (Found while checkpointing the release candidate, 2026-10-31); evidence: bug-0013-repro-1
- 2026-10-31T23:10:21Z · status: open -> verified. Fixed in 3b0b2f7; TC-cli-0066 passed (tc-cli-0066-1793488207).; evidence: bug-0013-fixed-1
