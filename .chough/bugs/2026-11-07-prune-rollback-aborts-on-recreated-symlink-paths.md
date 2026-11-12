---
kind: bug
id: 2026-11-07-prune-rollback-aborts-on-recreated-symlink-paths
aliases: [BUG-prune-rollback-aborts-on-recreated-symlink-paths-b187eee3aee864159e01358c]
title: Prune rollback aborts on recreated symlink paths
status: verified
classification: product-defect
scope: blocking
impact: Not yet assessed.
criteria: [LIFE-8]
requirements: [REQ-history-004]
cases: [TC-history-prune-unsafe-restore-continuation]
evidence:
  - OBS-tc-history-prune-unsafe-restore-continuation-8b9bbae246ad0129e40aa181
  - OBS-tc-history-prune-unsafe-restore-continuation-c9640fdb1cd9bf652847ac4b
receipts:
  - id: 2026-11-07-tc-history-prune-unsafe-restore-continuation
    aliases: OBS-tc-history-prune-unsafe-restore-continuation-8b9bbae246ad0129e40aa181
    claim: "Ran cargo test --locked --bin chough evidence::tests::pr197_unsafe_restore_ -- --nocapture for TC-history-prune-unsafe-restore-continuation: failed (exit 101)."
    result: failed
    command: cargo test --locked --bin chough evidence::tests::pr197_unsafe_restore_ -- --nocapture
    revision: 8f7574956f2f8b8789022e68d6a02e9ef0567062
    environment: local-macos-arm64
    content: 5fff028745dd4b812f2c44ac362e5e5a280224bb9d24c534e611c5a9d923fcae
    fingerprint: sha256:f54ae55a321a6672cceb79048b72467b6a259f38c2d6de5fbf72929bd9c86cc1
    case: TC-history-prune-unsafe-restore-continuation
    case_digest: 46ccbcac16f0273f7c6bde7355d161553fb06ab0b0b89d17a40b40780b5a5670
    observed: 2026-11-07T17:05:37Z
    duration_seconds: 0
  - id: 2026-11-07-tc-history-prune-unsafe-restore-continuation-8b8526
    aliases: OBS-tc-history-prune-unsafe-restore-continuation-c9640fdb1cd9bf652847ac4b
    claim: "Ran cargo test --locked --bin chough evidence::tests::pr197_unsafe_restore_ -- --nocapture for TC-history-prune-unsafe-restore-continuation: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough evidence::tests::pr197_unsafe_restore_ -- --nocapture
    revision: 6748c3d30a005e46abe468fc00eb43900eda374f
    environment: local-macos-arm64
    content: e71b8b6ed7758d1321609e1f534c702307429f0664fc85405e469508db66a807
    fingerprint: sha256:154b0327874b093eb494daf054915d3ee966cb4456e113c57a2fe8d231339290
    case: TC-history-prune-unsafe-restore-continuation
    case_digest: 46ccbcac16f0273f7c6bde7355d161553fb06ab0b0b89d17a40b40780b5a5670
    observed: 2026-11-07T17:07:17Z
    duration_seconds: 0
environment: local-macos-arm64
revision: 8f7574956f2f8b8789022e68d6a02e9ef0567062
found_by: PR 197 discussion_r4502629890
related: []
created: 2026-11-07T17:05:50Z
updated: 2026-11-07T17:09:08Z
---
# Prune rollback aborts on recreated symlink paths

**What happened:** Unsafe rollback path aborts safe hold, earlier restoration and ledger accounting
Actual: Both variants kept writer but lost recovery hold and earlier files; zero restored events; cargo exit 101

**Steps:** Recreate removed file or ancestor as symlink before rollback

**Expected:** Keep symlink identity and target; hold originals; attempt earlier restores and record both sets

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-07T17:05:50Z · Reported (PR 197 discussion_r4502629890); evidence: OBS-tc-history-prune-unsafe-restore-continuation-8b9bbae246ad0129e40aa181
- 2026-11-07T17:09:08Z · status: open -> verified. Same two red symlink variants now pass; restoring outer path-validation escape causes both to fail; full lanes and fresh human acceptance still pending.; evidence: OBS-tc-history-prune-unsafe-restore-continuation-c9640fdb1cd9bf652847ac4b
