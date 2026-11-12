---
kind: bug
id: 2026-11-12-automatic-failure-capture-links-an-existing-bug-s-evidence-by
aliases:
  - BUG-automatic-failure-capture-links-an-existing-bug-s-evidence-by-the-bare-dd9c999ec885746264802557
title: Automatic failure capture links an existing bug's evidence by the bare ID while bug add and bug update store it session-qualified
status: verified
classification: product-defect
scope: in-scope
impact: Captured links can read as ambiguous after a merge
criteria: []
requirements: [REQ-cli-012]
cases: []
evidence:
  - OBS-tc-cli-capture-links-qualified-085f9f276092be8eaa478f5b
  - OBS-tc-cli-capture-links-qualified-2b67f45c67183fd2379d85a6
receipts:
  - id: 2026-11-12-tc-cli-capture-links-qualified
    aliases: OBS-tc-cli-capture-links-qualified-085f9f276092be8eaa478f5b
    claim: "Ran cargo test --locked --test bugs -- --exact automatic_failure_capture_links_each_failure_by_its_session_qualified_id for TC-cli-capture-links-qualified: failed (exit 101)."
    result: failed
    command: cargo test --locked --test bugs -- --exact automatic_failure_capture_links_each_failure_by_its_session_qualified_id
    revision: d5d50a38cba25adf6fa46be44cea5938f66cc646
    environment: local-macos-arm64
    content: 345fd037f67997d9fbac91bdfc891ffc75eb572043f69727eb51816da9f42b8c
    fingerprint: sha256:728c7424335ba63fb09b717de9b26d0c8da6309f6dc80e6b01c027e37e188a6e
    case: TC-cli-capture-links-qualified
    case_digest: 955a67acfd26a40813a372abc44319fa646fec22a51e1b07b253fb56da13cf77
    observed: 2026-11-12T02:02:12Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-capture-links-qualified-610f68
    aliases: OBS-tc-cli-capture-links-qualified-2b67f45c67183fd2379d85a6
    claim: "Ran cargo test --locked --test bugs -- --exact automatic_failure_capture_links_each_failure_by_its_session_qualified_id for TC-cli-capture-links-qualified: passed (exit 0)."
    result: passed
    command: cargo test --locked --test bugs -- --exact automatic_failure_capture_links_each_failure_by_its_session_qualified_id
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:23db3beb69a9251dd11d617e184063337fe14e251289463f8c7b9c8a2895f6dd
    case: TC-cli-capture-links-qualified
    case_digest: 955a67acfd26a40813a372abc44319fa646fec22a51e1b07b253fb56da13cf77
    observed: 2026-11-12T02:17:35Z
    duration_seconds: 2
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 18025ad47c730af7589317f76608634b2487896f
found_by: t37j, test stage, areas sessions and bugs
related: []
created: 2026-11-12T01:41:12Z
updated: 2026-11-12T02:18:07Z
---
# Automatic failure capture links an existing bug's evidence by the bare ID while bug add and bug update store it session-qualified

**What happened:** The captured link is bare, so the two writers disagree and a merge can make it ambiguous again
Actual: The capture appends finding.observation as stored by the run

**Steps:** A failure captured onto an existing bug

**Expected:** One stored form for a bug's link to a session entry, <session>/<id>

**Why it matters:** Captured links can read as ambiguous after a merge

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:41:12Z · Reported (t37j, test stage, areas sessions and bugs)
- 2026-11-12T02:02:52Z · status: open -> investigating. Red at d5d50a38 (TC-cli-capture-links-qualified, on t37x's branch before its fix): failed for this bug's reason; fix in fc612691.; evidence: OBS-tc-cli-capture-links-qualified-085f9f276092be8eaa478f5b
- 2026-11-12T02:18:07Z · status: investigating -> verified. Green at 0ee26ac0: TC-cli-capture-links-qualified passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-cli-capture-links-qualified-2b67f45c67183fd2379d85a6
