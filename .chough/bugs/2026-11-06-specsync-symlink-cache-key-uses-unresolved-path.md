---
kind: bug
id: 2026-11-06-specsync-symlink-cache-key-uses-unresolved-path
aliases: [BUG-specsync-symlink-cache-key-uses-unresolved-path-9205c0afb5fb77ec63d04630]
title: SpecSync symlink cache key uses unresolved path
status: verified
classification: product-defect
scope: in-scope
impact: A symlink target change could invalidate or reuse the wrong binary identity unless the resolved path is keyed.
criteria: [DRIVE-15, DRIVE-2]
requirements: [REQ-cli-053]
cases: [TC-cli-specsync-symlink-reuse]
evidence:
  - OBS-tc-cli-specsync-symlink-reuse-ff73f0fe4f4b6d80220a43e7
  - OBS-tc-cli-specsync-symlink-reuse-b5fb5bc36c4a751af0c59fb3
receipts:
  - id: 2026-11-06-tc-cli-specsync-symlink-reuse
    aliases: OBS-tc-cli-specsync-symlink-reuse-ff73f0fe4f4b6d80220a43e7
    claim: "Preserved failing-first receipt: the resolved-SpecSync-symlink regression keyed <redacted:home-path> instead of the fixture's resolved real-specsync executable. This is a historical record, not a fresh execution."
    result: failed
    revision: cfa2d10422259d677e9e6f28ae49e7380628de78
    environment: local-macos-arm64
    content: d9381ff305c9a69260e3adee50f95ee7a92fbc834d6f621a6ef70e9d66e0cc5f
    fingerprint: sha256:de70aa00d8d598dec8b2155283f0bc8158548e5de991c32aa59768d0b0859778
    case: TC-cli-specsync-symlink-reuse
    case_digest: 74c5e86c830c2f021be2bcb4d8f12bba871e3539e0f2bbf5a1020756cbdc06d8
    observed: 2026-11-06T23:17:59Z
  - id: 2026-11-06-tc-cli-specsync-symlink-reuse-0125a9
    aliases: OBS-tc-cli-specsync-symlink-reuse-b5fb5bc36c4a751af0c59fb3
    claim: "Ran cargo test --locked --test native -- --exact a_resolved_specsync_symlink_can_prime_and_reuse for TC-cli-specsync-symlink-reuse: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact a_resolved_specsync_symlink_can_prime_and_reuse
    revision: cfa2d10422259d677e9e6f28ae49e7380628de78
    environment: local-macos-arm64
    content: d9381ff305c9a69260e3adee50f95ee7a92fbc834d6f621a6ef70e9d66e0cc5f
    fingerprint: sha256:b359b806f91d8da1ea5318b7feea4a9443c1be6732e068bd15113fe53cff1f9d
    case: TC-cli-specsync-symlink-reuse
    case_digest: 74c5e86c830c2f021be2bcb4d8f12bba871e3539e0f2bbf5a1020756cbdc06d8
    observed: 2026-11-06T23:18:44Z
    duration_seconds: 1
fixture: .chough/artifacts/issue-32-next-status-latency/failing-specsync-symlink-before.log
environment: local-macos-arm64
revision: cfa2d10422259d677e9e6f28ae49e7380628de78
found_by: preserved failing-first product test
related: []
created: 2026-11-06T23:18:20Z
updated: 2026-11-06T23:18:52Z
---
# SpecSync symlink cache key uses unresolved path

**What happened:** The archive cache identity names the PATH symlink instead of the exact resolved SpecSync executable.
Actual: The key contains <redacted:home-path> rather than the fixture's real-specsync path.

**Steps:** Prime and reuse an archive entry through a symlinked SpecSync executable in a fixture.
1. Run a_resolved_specsync_symlink_can_prime_and_reuse before the resolved-path correction.

**Expected:** The cache key uses the resolved executable path and bytes supplied by the fixture.

**Why it matters:** A symlink target change could invalidate or reuse the wrong binary identity unless the resolved path is keyed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-06T23:18:20Z · Reported (preserved failing-first product test); evidence: OBS-tc-cli-specsync-symlink-reuse-ff73f0fe4f4b6d80220a43e7
- 2026-11-06T23:18:52Z · status: open -> verified. The resolved SpecSync executable path and bytes now pass the symlink regression after the preserved failure.; evidence: OBS-tc-cli-specsync-symlink-reuse-b5fb5bc36c4a751af0c59fb3
