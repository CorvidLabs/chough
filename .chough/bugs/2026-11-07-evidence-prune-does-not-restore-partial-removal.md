---
kind: bug
id: 2026-11-07-evidence-prune-does-not-restore-partial-removal
aliases: [BUG-evidence-prune-does-not-restore-partial-removal-14338a2c0a36ae54472c1b07]
title: Evidence prune does not restore partial removal
status: verified
classification: product-defect
scope: in-scope
impact: A refused apply destroys uncommitted evidence
criteria: [LIFE-8, LIFE-2, LIFE-7]
requirements: [REQ-history-004]
cases: [TC-history-prune-restores-partial-removal]
evidence:
  - OBS-tc-history-prune-restores-partial-removal-9d8c4669e6b99ad1f7b3504d
  - OBS-tc-history-prune-restores-partial-removal-c5ff0e975581686f00f9f078
  - OBS-product-test-972af92fbd4cb64f5dd3094a
  - OBS-product-test-51a832506d132b756faaf82a
receipts:
  - id: 2026-11-07-tc-history-prune-restores-partial-removal
    aliases: OBS-tc-history-prune-restores-partial-removal-9d8c4669e6b99ad1f7b3504d
    claim: "Ran cargo test --locked --test evidence -- --exact evidence_prune_restores_partial_removal for TC-history-prune-restores-partial-removal: failed (exit 101)."
    result: failed
    command: cargo test --locked --test evidence -- --exact evidence_prune_restores_partial_removal
    revision: 3ad9df48933b880836c812c06369d00b9369e918
    environment: local-macos-arm64
    content: db3db4c3f6021d81312953b0bc5cf8fd5d1d5ca5e682fa02bc8db977c78e3a96
    fingerprint: sha256:68002027fad3d48796db36407ca4274a39c8200540898326746d902600222929
    case: TC-history-prune-restores-partial-removal
    case_digest: 7293ab7ec230fc592298ec9915eaab8b78054de3265729abcff9834a7d4e2af2
    observed: 2026-11-07T03:24:47Z
    duration_seconds: 9
  - id: 2026-11-07-tc-history-prune-restores-partial-removal-5788f1
    aliases: OBS-tc-history-prune-restores-partial-removal-c5ff0e975581686f00f9f078
    claim: "Ran cargo test --locked --test evidence -- --exact evidence_prune_restores_partial_removal for TC-history-prune-restores-partial-removal: passed (exit 0)."
    result: passed
    command: cargo test --locked --test evidence -- --exact evidence_prune_restores_partial_removal
    revision: 55cb7fa5662fd6bc67b32757d67242df8ed121dd
    environment: local-macos-arm64
    content: da0c63e939b46491710343634b2abce74b5a5a3e68a747d6470db21efda19748
    fingerprint: sha256:83cfea1d15344d944e8dc295dcce846032fa6a2eba3f66e6b8f148f4140e6825
    case: TC-history-prune-restores-partial-removal
    case_digest: 7293ab7ec230fc592298ec9915eaab8b78054de3265729abcff9834a7d4e2af2
    observed: 2026-11-07T03:35:59Z
    duration_seconds: 3
  - id: OBS-product-test-972af92fbd4cb64f5dd3094a
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-product-test-51a832506d132b756faaf82a
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
fixture: .chough/artifacts/issue-65-prune-race/before/partial-removal.log
environment: local-macos-arm64
revision: 3ad9df48933b880836c812c06369d00b9369e918
found_by: "Issue #65 failure-first test at 3ad9df48"
related: []
created: 2026-11-07T03:25:13Z
updated: 2026-11-07T06:51:16Z
---
# Evidence prune does not restore partial removal

**What happened:** A failed observation unlink after artifact removal leaves the artifact missing
Actual: The artifact stays deleted and only the initial prune ledger line exists

**Steps:** chough evidence prune --apply when the evidence directory rejects the observation rename
1. Make the evidence directory read-only and run evidence prune --apply

**Expected:** Restore the artifact with original bytes and permissions and append a restored event

**Why it matters:** A refused apply destroys uncommitted evidence

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-07T03:25:13Z · Reported (Issue #65 failure-first test at 3ad9df48); evidence: OBS-tc-history-prune-restores-partial-removal-9d8c4669e6b99ad1f7b3504d
- 2026-11-07T03:36:14Z · status: open -> verified. The real CLI regression now restores exact bytes and permissions and leaves no hold; evidence: OBS-tc-history-prune-restores-partial-removal-c5ff0e975581686f00f9f078
- 2026-11-07T06:44:09Z · status: verified -> open. Finish review found an in-scope restore publication race: after create_new/write, a concurrent replacement receives the original chmod. Deterministic failure checks replacement mode, inode and bytes plus preserved original recovery bytes.; evidence: OBS-product-test-972af92fbd4cb64f5dd3094a
- 2026-11-07T06:51:16Z · status: open -> verified. Private staged restoration passes the deterministic replacement inode/mode/bytes regression and full focused evidence/history suites; original failing chmod receipt is preserved. Full lanes remain separate and not passed.; evidence: OBS-product-test-51a832506d132b756faaf82a
