---
kind: bug
id: 2026-10-31-reset-reaches-accept-or-done-without-their-native
aliases: [BUG-0007]
title: Reset reaches accept or done without their native prerequisites
status: verified
classification: product-defect
scope: in-scope
impact: A checkpoint can claim done without acceptance or archival
criteria: [FLOW-2, FLOW-10]
requirements: [REQ-cli-008]
cases: []
evidence: [bug-0007-repro-1, bug-0007-fixed-1]
receipts:
  - id: 2026-10-31-with-the-native-change-draft-chough-stage-reset-to-done-and-to
    aliases: bug-0007-repro-1
    claim: With the native change draft, chough stage reset --to done and --to accept both succeeded
    result: failed
    revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
    environment: local-macos-arm64
    fingerprint: sha256:bf8169ac1a074c532bef7f93f30ead3ee581d3e09a13cd319976fb086e89f1a7
    observed: 2026-10-31T20:44:39Z
  - id: 2026-10-31-with-the-change-draft-reset-to-done-and-to-accept-are-refused
    aliases: bug-0007-fixed-1
    claim: With the change draft, reset --to done and --to accept are refused; the stage stays at intent
    result: passed
    revision: 80757487bddf9172bd757f78fcfa798e1d7e40b3
    environment: local-macos-arm64
    fingerprint: sha256:3de8bc7fe02ac38c185196a2ccbc1d690e9950e6165c8e5d24665c92038fc3f4
    observed: 2026-10-31T21:24:01Z
environment: local-macos-arm64
revision: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
found_by: Review finding B, reproduced 2026-10-31 against main f19823d
related: []
created: 2026-10-31T20:45:00Z
updated: 2026-10-31T21:24:02Z
---
# Reset reaches accept or done without their native prerequisites

**What happened:** stage reset --to done and --to accept succeeded while the native change was draft
Actual: Allowed; only reset --to build is gated

**Steps:** chough stage reset --to <accept|done> with an unapproved or unarchived change
1. Draft native change recorded in the checkpoint
2. chough stage reset --to done --reason x

**Expected:** Reset may move back freely but refuses a destination whose native prerequisites do not hold

**Why it matters:** A checkpoint can claim done without acceptance or archival

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T20:45:00Z · Reported (Review finding B, reproduced 2026-10-31 against main f19823d); evidence: bug-0007-repro-1
- 2026-10-31T21:24:02Z · status: open -> verified. Fixed in 8075748 (fail-closed stage gates); regression TC-cli-0041 passed (observation tc-cli-0041-1793481645).; evidence: bug-0007-fixed-1
