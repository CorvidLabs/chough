---
kind: bug
id: 2026-11-07-reapproved-prune-inputs-lack-current-native-verification
aliases: [BUG-reapproved-prune-inputs-lack-current-native-verification-b5e5ca5c85f4d7672d2d43f1]
title: Reapproved prune inputs lack current native verification
status: verified
classification: evidence-gap
scope: blocking
impact: Not yet assessed.
criteria: [LIFE-7]
requirements: [REQ-history-004]
cases: []
evidence:
  - OBS-structural-0569129fb63cffe3aa2b53bf
  - OBS-structural-1b635c037853e3ce2f70d10b
  - OBS-structural-4bb245473772e0d89afd2868
  - OBS-structural-01fdbe567f85ef177eeea144
  - OBS-structural-ec5e908e555a15be59cc7e27
  - OBS-product-test-177ef9b1ad9bc144d0b26aa4
  - OBS-product-test-766be385e3010d3b004a21f6
receipts:
  - id: 2026-11-07-pinned-linux-verify-at
    aliases: OBS-structural-0569129fb63cffe3aa2b53bf
    claim: "Pinned Linux verify at b4b4dae7208570d63b613a0d5e638e42aa975e3a passed the complete product suite and strict spec check, then failed lifecycle audit step 6: src/evidence.rs and tests/evidence.rs reported uncovered despite valid approved scope. Native next action remains change check since reapproval."
    result: failed
    revision: b4b4dae7208570d63b613a0d5e638e42aa975e3a
    environment: local-linux-aarch64
    content: 324e9aaa8a63ba86fb6cb31608a5c5e08962a9011615ba239e32bba746b7718d
    fingerprint: sha256:c661c7de6e18bb98b1b9db257e1c3acaa3f6aa3315f25e11587f9888061a352e
    observed: 2026-11-07T15:00:05Z
  - id: OBS-structural-1b635c037853e3ce2f70d10b
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-07-preparatory-chough-change-check-passed-and-committed-current
    aliases: OBS-structural-4bb245473772e0d89afd2868
    claim: Preparatory Chough change check passed and committed current native verification at 3e1198b4cea01fdbbbb6904a58dce94529dfc29e. specsync change audit --strict, all five pre-push steps and knowledge index check then passed. Product source/spec/docs/test files are byte-identical to b4b4dae7208570d63b613a0d5e638e42aa975e3a.
    result: passed
    revision: 3e1198b4cea01fdbbbb6904a58dce94529dfc29e
    environment: local-macos-arm64
    content: 63720fce57a8bb4623f7ed805347e8968256c838e8889ab57c883f2ccdc48b45
    fingerprint: sha256:81d3a5cee25bbb7f02215f723414ee2474fdb7429b7bbd8f472ba811ad72be7a
    observed: 2026-11-07T15:02:43Z
  - id: 2026-11-07-linux-full-rerun-at-9ebacabe170eee2cf7009c2d181d3093da73ae6d
    aliases: OBS-structural-01fdbe567f85ef177eeea144
    claim: Linux full rerun at 9ebacabe170eee2cf7009c2d181d3093da73ae6d passed product tests and strict specs, then audit rejected stale native verification. The recovery evidence/index commit after the preparatory check invalidated the verification digest; no product source changed. Retain all receipts and freeze all inputs after the next check.
    result: failed
    revision: 9ebacabe170eee2cf7009c2d181d3093da73ae6d
    environment: local-linux-aarch64
    content: 9c3435ba717cb1dfc6540a9c1cd6ab23a4582fcbf14128ac8fa1dbfb6dc3e083
    fingerprint: sha256:b141efcb8d9172b114baa30852d976ce57207c41e6572069ac39e79a60e8a590
    observed: 2026-11-07T15:07:56Z
  - id: 2026-11-07-macos-full-rerun-at-9ebacabe170eee2cf7009c2d181d3093da73ae6d
    aliases: OBS-structural-ec5e908e555a15be59cc7e27
    claim: macOS full rerun at 9ebacabe170eee2cf7009c2d181d3093da73ae6d passed product tests and strict specs, then audit rejected stale native verification. The recovery evidence/index commit after the preparatory check invalidated the verification digest; no product source changed. Retain all receipts and freeze all inputs after the next check.
    result: failed
    revision: 9ebacabe170eee2cf7009c2d181d3093da73ae6d
    environment: local-macos-arm64
    content: 9c3435ba717cb1dfc6540a9c1cd6ab23a4582fcbf14128ac8fa1dbfb6dc3e083
    fingerprint: sha256:b5a7fb10187ea7a999447d4f25e4860ada8e9da5546f0cf3eef34cbe33419e67
    observed: 2026-11-07T15:07:58Z
  - id: OBS-product-test-177ef9b1ad9bc144d0b26aa4
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-07-complete-pinned-linux-verify-lane-passed-all-10-steps-at
    aliases: OBS-product-test-766be385e3010d3b004a21f6
    claim: "Complete pinned linux verify lane passed all 10 steps at frozen commit 8d5bcf430921bf14c8557214354f1928f2358cc3: 597 product tests; formatting/lint, intent, strict specs with 100% coverage, native audit, case definitions, knowledge, real CLI exercise and build. No workspace writes occurred during either platform run."
    result: passed
    revision: 8d5bcf430921bf14c8557214354f1928f2358cc3
    environment: local-linux-aarch64
    content: e7ee35372197946f77f0ab0be149f6769b5c14768ed2cf881715441f85004e62
    fingerprint: sha256:237bb6f9034df4db9f9d76ddb985cb72727af9c4099f220ddc12524c68ce5cbb
    observed: 2026-11-07T15:14:17Z
environment: macos-arm64-and-linux-aarch64
revision: b4b4dae7208570d63b613a0d5e638e42aa975e3a
found_by: Pinned lanes after approved one-line digest refresh, before current native check
related: []
created: 2026-11-07T15:00:29Z
updated: 2026-11-07T15:14:19Z
---
# Reapproved prune inputs lack current native verification

**What happened:** Both pinned verify lanes pass product tests and strict specs but lifecycle audit reports approved paths uncovered
Actual: Linux audit names src/evidence.rs and tests/evidence.rs; macOS also names both spec contracts, history requirements/testing, and tests/spec_split.rs. Native status remains approved with next action change check.

**Steps:** Run the full pinned verify lanes at b4b4dae7208570d63b613a0d5e638e42aa975e3a before the reapproved native change check

**Expected:** The valid reapproved change supplies current native scoped verification for its declared source and spec paths

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-07T15:00:29Z · Reported (Pinned lanes after approved one-line digest refresh, before current native check); evidence: OBS-structural-0569129fb63cffe3aa2b53bf, OBS-structural-1b635c037853e3ce2f70d10b
- 2026-11-07T15:02:45Z · status: open -> verified. The authorized preparatory native check established current scoped verification; strict audit, declared pre-push and knowledge checks passed afterward. Both original failed full lanes are retained; no source changes were needed.; evidence: OBS-structural-4bb245473772e0d89afd2868
- 2026-11-07T15:08:00Z · status: verified -> open. Recovery bookkeeping after the preparatory check staled its digest. Both full reruns passed product/spec but failed audit for stale verification. Freeze all inputs after next check; no source or gate policy changes.; evidence: OBS-structural-01fdbe567f85ef177eeea144, OBS-structural-ec5e908e555a15be59cc7e27
- 2026-11-07T15:14:19Z · status: open -> verified. After all diagnostics were committed, the next native check and strict audit passed at 8d5bcf430921bf14c8557214354f1928f2358cc3. Both full pinned lanes then passed all 10 steps on that exact frozen tree. All earlier audit/uncovered/stale failures remain intact.; evidence: OBS-product-test-177ef9b1ad9bc144d0b26aa4, OBS-product-test-766be385e3010d3b004a21f6
