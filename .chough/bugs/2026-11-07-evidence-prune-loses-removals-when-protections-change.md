---
kind: bug
id: 2026-11-07-evidence-prune-loses-removals-when-protections-change
aliases: [BUG-evidence-prune-loses-removals-when-protections-change-f0aabb199383ff63877394c2]
title: Evidence prune loses removals when protections change
status: verified
classification: product-defect
scope: in-scope
impact: Newly protected proof is lost from the working tree
criteria: [LIFE-8, LIFE-2, LIFE-7]
requirements: [REQ-history-004]
cases:
  - TC-history-prune-protection-race
  - TC-history-prune-restores-earlier-sets
  - TC-history-prune-restore-preserves-recreated-file
evidence:
  - OBS-tc-history-prune-protection-race-16793e9e4c84b7f00004251a
  - OBS-tc-history-prune-restores-earlier-sets-bba515e3dbd42cea1399e660
  - OBS-tc-history-prune-restore-preserves-recreated-file-fe5568108c710f47994da0c7
  - OBS-tc-history-prune-protection-race-831610f271a785164c912a6e
  - OBS-tc-history-prune-restores-earlier-sets-42efe7ab63bba4851815235f
  - OBS-tc-history-prune-restore-preserves-recreated-file-ec625ca74058487a167adf05
receipts:
  - id: 2026-11-07-tc-history-prune-protection-race
    aliases: OBS-tc-history-prune-protection-race-16793e9e4c84b7f00004251a
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::prune_protection_race_restores_removed_bytes for TC-history-prune-protection-race: failed (exit 101)."
    result: failed
    command: cargo test --locked --bin chough -- --exact evidence::tests::prune_protection_race_restores_removed_bytes
    revision: 3ad9df48933b880836c812c06369d00b9369e918
    environment: local-macos-arm64
    content: db3db4c3f6021d81312953b0bc5cf8fd5d1d5ca5e682fa02bc8db977c78e3a96
    fingerprint: sha256:8349a7581aba8bab3d75a6a99b023fb9eb5063e7e42cdd653d6ffedb8f959217
    case: TC-history-prune-protection-race
    case_digest: 763949fbada75217dc6f170254ae8a0313bff28996820116e8b9097159f1df00
    observed: 2026-11-07T03:24:16Z
    duration_seconds: 0
  - id: 2026-11-07-tc-history-prune-restores-earlier-sets
    aliases: OBS-tc-history-prune-restores-earlier-sets-bba515e3dbd42cea1399e660
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::prune_restores_earlier_sets_when_a_later_set_becomes_protected for TC-history-prune-restores-earlier-sets: failed (exit 101)."
    result: failed
    command: cargo test --locked --bin chough -- --exact evidence::tests::prune_restores_earlier_sets_when_a_later_set_becomes_protected
    revision: 3ad9df48933b880836c812c06369d00b9369e918
    environment: local-macos-arm64
    content: db3db4c3f6021d81312953b0bc5cf8fd5d1d5ca5e682fa02bc8db977c78e3a96
    fingerprint: sha256:62270aeb93736aad7dbb8655d352090c33750c2b5d6bc9b2f992d3a7af63f621
    case: TC-history-prune-restores-earlier-sets
    case_digest: 171ed7d2d36db96fd78c5d34b7ffa93e16bf5c0403fb267924b90bd1d0ab61fd
    observed: 2026-11-07T03:24:22Z
    duration_seconds: 0
  - id: 2026-11-07-tc-history-prune-restore-preserves-recreated-file
    aliases: OBS-tc-history-prune-restore-preserves-recreated-file-fe5568108c710f47994da0c7
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::prune_restore_preserves_recreated_file_and_uncommitted_original for TC-history-prune-restore-preserves-recreated-file: failed (exit 101)."
    result: failed
    command: cargo test --locked --bin chough -- --exact evidence::tests::prune_restore_preserves_recreated_file_and_uncommitted_original
    revision: 3ad9df48933b880836c812c06369d00b9369e918
    environment: local-macos-arm64
    content: db3db4c3f6021d81312953b0bc5cf8fd5d1d5ca5e682fa02bc8db977c78e3a96
    fingerprint: sha256:2cc8cf88624c42b6d20d99d110ac9aa6d5bfea814af51250bd0a19b34ef5dc26
    case: TC-history-prune-restore-preserves-recreated-file
    case_digest: bb57cfda6d4595d4797054fe06a429bfa00f41b10593bfd196bb9cc5e278847c
    observed: 2026-11-07T03:24:31Z
    duration_seconds: 0
  - id: 2026-11-07-tc-history-prune-protection-race-d33d41
    aliases: OBS-tc-history-prune-protection-race-831610f271a785164c912a6e
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::prune_protection_race_restores_removed_bytes for TC-history-prune-protection-race: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough -- --exact evidence::tests::prune_protection_race_restores_removed_bytes
    revision: 55cb7fa5662fd6bc67b32757d67242df8ed121dd
    environment: local-macos-arm64
    content: da0c63e939b46491710343634b2abce74b5a5a3e68a747d6470db21efda19748
    fingerprint: sha256:769b457288a4fca8f7d8b4d24a6f631dbcb392005a50f8f741d31b8371b151ab
    case: TC-history-prune-protection-race
    case_digest: 763949fbada75217dc6f170254ae8a0313bff28996820116e8b9097159f1df00
    observed: 2026-11-07T03:35:06Z
    duration_seconds: 1
  - id: OBS-tc-history-prune-restores-earlier-sets-42efe7ab63bba4851815235f
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-history-prune-restore-preserves-recreated-file-ec625ca74058487a167adf05
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
fixture: .chough/artifacts/issue-65-prune-race/before/protection-race.log
environment: local-macos-arm64
revision: 3ad9df48933b880836c812c06369d00b9369e918
found_by: "Issue #65 failure-first tests at 3ad9df48"
related: []
created: 2026-11-07T03:25:13Z
updated: 2026-11-07T03:36:13Z
---
# Evidence prune loses removals when protections change

**What happened:** An artifact or a completely pruned earlier set stays missing after a new protection appears during apply
Actual: Earlier removals stay missing and no restored event or recovery hold exists

**Steps:** chough evidence prune --apply while another writer adds a reference between removals
1. Run each deterministic protection-race regression before the fix

**Expected:** Refuse and restore every earlier removal, preserve recreated files and keep original uncommitted bytes in a named recovery hold

**Why it matters:** Newly protected proof is lost from the working tree

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-07T03:25:13Z · Reported (Issue #65 failure-first tests at 3ad9df48); evidence: OBS-tc-history-prune-protection-race-16793e9e4c84b7f00004251a, OBS-tc-history-prune-restores-earlier-sets-bba515e3dbd42cea1399e660, OBS-tc-history-prune-restore-preserves-recreated-file-fe5568108c710f47994da0c7
- 2026-11-07T03:36:13Z · status: open -> verified. All three race and rollback regressions now pass after the original failures; evidence: OBS-tc-history-prune-protection-race-831610f271a785164c912a6e, OBS-tc-history-prune-restores-earlier-sets-42efe7ab63bba4851815235f, OBS-tc-history-prune-restore-preserves-recreated-file-ec625ca74058487a167adf05
