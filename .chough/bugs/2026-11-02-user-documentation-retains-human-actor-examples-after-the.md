---
kind: bug
id: 2026-11-02-user-documentation-retains-human-actor-examples-after-the
aliases:
  - BUG-user-documentation-retains-human-actor-examples-after-the-agent-authori-20661e7a890a9c8895f8832d
title: User documentation retains human actor examples after the agent authority update
status: verified
classification: product-defect
scope: blocking
impact: Blocks consistent authority guidance and PR44 peer acceptance.
criteria: [LIFE-6]
requirements: [REQ-cli-029]
cases: []
evidence:
  - OBS-product-test-4df13417a21256aaa3c5b151
  - OBS-tc-cli-agent-native-workflow-upgrade-670f71f7fbfb5c0263a54195
receipts:
  - id: 2026-11-02-expanded-writer-example-regression-fails-on-four-human-actor
    aliases: OBS-product-test-4df13417a21256aaa3c5b151
    claim: Expanded writer-example regression fails on four human-actor examples in three user docs at baseline 9c41e95; exact test patch, source and binary hashes and failed output are retained before docs repair.
    result: failed
    revision: 9c41e95949adcb1009c986f97db50faa593d81b2
    environment: local-macos-arm64
    content: 96c399fcb93116e3fc1f83137e3dbcdba145820f5f46895a97314ffb3497e3ed
    fingerprint: sha256:e61687ea584ddbb08b3c2485c0257258c59cf39ab4d85127f4aef4c06d46a104
    observed: 2026-11-02T18:04:55Z
  - id: 2026-11-02-tc-cli-agent-native-workflow-upgrade
    aliases: OBS-tc-cli-agent-native-workflow-upgrade-670f71f7fbfb5c0263a54195
    claim: All nine Fledge steps, all198tests across11suites on Rust1.95, and both bound cases pass after user-doc actor/source repair.
    result: passed
    revision: 9c41e95949adcb1009c986f97db50faa593d81b2
    environment: local-macos-arm64
    content: 983bb9f01666418f7e9163725d5ea41c30d271448c4b62b07d35b16604616a60
    fingerprint: sha256:5f131ab8eecf1e30c34fdfb68e29ada1da1ef88dc38b28ebb98d2753a9f4aba0
    case: TC-cli-agent-native-workflow-upgrade
    case_digest: df58494ed355c0121e0e12514d3a6b539bec2588121efa437b06a3f2bc9238fd
    observed: 2026-11-02T18:16:29Z
fixture: .chough/artifacts/native-workflow-user-docs-before
environment: local-macos-arm64
revision: 9c41e95949adcb1009c986f97db50faa593d81b2
found_by: corvid-agent PR44 review5741003490, reproduced by Codex expanded regression
related: []
created: 2026-11-02T18:04:55Z
updated: 2026-11-02T18:16:30Z
---
# User documentation retains human actor examples after the agent authority update

**What happened:** CLI, walkthrough and next-change guidance still uses the human name as native actor.
Actual: The focused regression fails on four human-actor examples across three docs.

**Steps:** Expand the native writer consistency regression to the three user-facing docs.

**Expected:** Current native writer examples use the acting agent account and cite actual human authority source, scope and limits.

**Why it matters:** Blocks consistent authority guidance and PR44 peer acceptance.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T18:04:55Z · Reported (corvid-agent PR44 review5741003490, reproduced by Codex expanded regression); evidence: OBS-product-test-4df13417a21256aaa3c5b151
- 2026-11-02T18:16:30Z · status: open -> verified. All four human-actor examples corrected across three user docs and included in structural coverage. Both cases, all198tests on Rust1.95/Rust1.88 and allnineFledge steps pass. Failure and historical proposal status remain intact.; evidence: OBS-tc-cli-agent-native-workflow-upgrade-670f71f7fbfb5c0263a54195
