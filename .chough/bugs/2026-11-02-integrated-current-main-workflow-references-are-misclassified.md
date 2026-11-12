---
kind: bug
id: 2026-11-02-integrated-current-main-workflow-references-are-misclassified
aliases:
  - BUG-integrated-current-main-workflow-references-are-misclassified-as-edits-8cbff17ede6b1bad3926bce3
title: Integrated current-main workflow references are misclassified as edits
status: verified
classification: product-defect
scope: in-scope
impact: Safe guidance upgrades from the newly merged main would be skipped
criteria: [LIFE-4]
requirements: [REQ-cli-029]
cases: [TC-cli-agent-native-workflow-upgrade]
evidence:
  - OBS-tc-cli-agent-native-workflow-upgrade-629794b9905f71763493a1de
  - OBS-tc-cli-agent-native-workflow-upgrade-bb63e838fc5d8248a6d5e303
receipts:
  - id: 2026-11-02-tc-cli-agent-native-workflow-upgrade-22144a
    aliases: OBS-tc-cli-agent-native-workflow-upgrade-629794b9905f71763493a1de
    claim: Integration regression reproduced unchanged current-main3d00e44 plan/accept copies being classified edited instead of known-outdated after the authority/order merge. Original825 predecessor upgrades passed before the new-main fixture failed.
    result: failed
    revision: f2a0c94e4938b1048a75e8bc85d95b0f9feedd22
    environment: local-macos-arm64
    content: 154e7409ebb1bc0615c45d682c9a5b7a7839a07e78ffd7ed1ede1d1c0342cfe1
    fingerprint: sha256:ee5f8dd333004de46f248fce06fead0ad21a93a0ad88615ca657fbfbfabb7f2a
    case: TC-cli-agent-native-workflow-upgrade
    case_digest: df58494ed355c0121e0e12514d3a6b539bec2588121efa437b06a3f2bc9238fd
    observed: 2026-11-02T23:20:35Z
  - id: 2026-11-02-tc-cli-agent-native-workflow-upgrade-c40d31
    aliases: OBS-tc-cli-agent-native-workflow-upgrade-bb63e838fc5d8248a6d5e303
    claim: Both workflow authority tests pass after registering the exact merged-main plan/accept identities. Original825 and incoming3d00e44 copies upgrade safely; every edited variant and checkpoint is preserved, canonical actors and both host copies remain consistent.
    result: passed
    revision: f2a0c94e4938b1048a75e8bc85d95b0f9feedd22
    environment: local-macos-arm64
    content: 07fd515f61956a6f468e4ccacf703311ea092433fc0dca18fd236aa650b4023b
    fingerprint: sha256:0684d642073baedfc5817a0c46d1b0d20b1f894325e4107ac237395aaf5b72c8
    case: TC-cli-agent-native-workflow-upgrade
    case_digest: df58494ed355c0121e0e12514d3a6b539bec2588121efa437b06a3f2bc9238fd
    observed: 2026-11-02T23:22:41Z
environment: local-macos-arm64
revision: f2a0c94e4938b1048a75e8bc85d95b0f9feedd22
found_by: PR44 merge integration regression at pinned main3d00e44
related: []
created: 2026-11-02T23:20:35Z
updated: 2026-11-02T23:22:42Z
---
# Integrated current-main workflow references are misclassified as edits

**What happened:** Exact unchanged plan and accept references from merged main3d00e44 are classified edited and cannot upgrade
Actual: The exact plan and accept copies report edited; the targeted integration regression fails

**Steps:** Merge PR41 case ordering with PR44 authority guidance without registering the integrated main predecessor bytes

**Expected:** Unchanged main predecessor references are known-outdated and safely upgrade, while edited copies remain preserved

**Why it matters:** Safe guidance upgrades from the newly merged main would be skipped

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T23:20:35Z · Reported (PR44 merge integration regression at pinned main3d00e44); evidence: OBS-tc-cli-agent-native-workflow-upgrade-629794b9905f71763493a1de
- 2026-11-02T23:20:35Z · Added despite similar reports: This concerns two new exact predecessor identities from merged main3d00e44; prior verified authority/copy bugs concern825 guidance and user examples, not these newer installed bytes.
- 2026-11-02T23:22:42Z · status: open -> verified. Registered exact current-main plan/accept hashes and preserved both original825 and integrated-main fixture families; no existing assertion or frozen fixture changed.; evidence: OBS-tc-cli-agent-native-workflow-upgrade-bb63e838fc5d8248a6d5e303
