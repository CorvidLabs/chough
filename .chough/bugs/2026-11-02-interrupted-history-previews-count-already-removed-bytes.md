---
kind: bug
id: 2026-11-02-interrupted-history-previews-count-already-removed-bytes
aliases: [BUG-interrupted-history-previews-count-already-removed-bytes-e3d3af55cc3d38bd82a3befc]
title: Interrupted history previews count already removed bytes
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: [LIFE-8]
requirements: [REQ-cli-026]
cases: []
evidence: [OBS-product-test-c2b42eee72403d04f8705539, OBS-product-test-fca5c5e603111f6dc31f5f4d]
receipts:
  - id: 2026-11-02-the-interrupted-cleanup-fixture-reproduced-a-preview
    aliases: OBS-product-test-c2b42eee72403d04f8705539
    claim: "The interrupted-cleanup fixture reproduced a preview accounting defect: bytes and files included content already removed under an existing receipt instead of only the remaining reclaimable content."
    result: failed
    revision: 9a8b9b11e16bedac4808c56d48307fa29ad9cfe3
    environment: local-macos-arm64
    content: f806fbc5a053d5a6333e4a198dd4e783dba24dcd3b68461d6fea023ab08046b6
    fingerprint: sha256:b763044a9cdb8b1570e404e67b7e631a82f439ea0a6125e681bce10a270416fe
    observed: 2026-11-02T14:49:27Z
  - id: 2026-11-02-the-mandatory-fledge-verify-lane-passed-all-9-steps-on-the
    aliases: OBS-product-test-fca5c5e603111f6dc31f5f4d
    claim: "The mandatory Fledge verify lane passed all 9 steps on the final implementation: 219 Rust tests passed with no failures or exclusions, format/lint/HI/structural/native audit/case definitions passed, and the disposable real CLI feature-and-bug exercise passed. The log includes the initialization preflight and interrupted-byte-count regressions."
    result: passed
    revision: 9a8b9b11e16bedac4808c56d48307fa29ad9cfe3
    environment: local-macos-arm64
    content: 0706650c13e025a3e3af738b2b3a63ed9e32dcdf22d03c217ee27d913c34e031
    fingerprint: sha256:38778d7538086fb1b8d4fe3e3089014191c33964209ce1668a2e5cb6b33e51d5
    observed: 2026-11-02T15:02:09Z
fixture: tests/history.rs
environment: local-macos-arm64
revision: 9a8b9b11e16bedac4808c56d48307fa29ad9cfe3
found_by: Current-source exact-byte assertion before the reporting fix
related: []
created: 2026-11-02T14:51:52Z
updated: 2026-11-02T15:02:10Z
---
# Interrupted history previews count already removed bytes

**What happened:** An interrupted-cycle preview overstates remaining reclaimable files and bytes
Actual: Preview reported 847 bytes when the remaining file held 494 bytes in the controlled fixture

**Steps:** Publish a valid cycle removal receipt, remove one planned file, then preview resumed cleanup

**Expected:** Preview reports only still-present candidate files and their measured bytes while preserving the original receipt plan separately

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T14:51:52Z · Reported (Current-source exact-byte assertion before the reporting fix); evidence: OBS-product-test-c2b42eee72403d04f8705539
- 2026-11-02T15:02:10Z · status: open -> verified. The later full lane passes the exact remaining-byte assertion; current files/bytes and original receipt plan are reported separately.; evidence: OBS-product-test-fca5c5e603111f6dc31f5f4d
