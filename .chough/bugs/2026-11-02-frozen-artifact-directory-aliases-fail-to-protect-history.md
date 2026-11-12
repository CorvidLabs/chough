---
kind: bug
id: 2026-11-02-frozen-artifact-directory-aliases-fail-to-protect-history
aliases:
  - BUG-frozen-artifact-directory-aliases-fail-to-protect-history-cycles-6dfc916446fda2d03ddb49a3
title: Frozen artifact directory aliases fail to protect history cycles
status: verified
classification: product-defect
scope: blocking
impact: "P2: referenced proof can be removed despite its retained protection source"
criteria: [LIFE-8]
requirements: [REQ-cli-026, REQ-cli-027]
cases: [TC-cli-history-retention]
evidence:
  - OBS-product-test-063bada1f0797d7f9fc3d83e
  - OBS-product-test-4ca251f7695567122f3b2bcd
  - OBS-product-test-ba4c86bbf0c00f16f5636082
receipts:
  - id: 2026-11-02-independent-retention-review-reproduced-protected-cycle
    aliases: OBS-product-test-063bada1f0797d7f9fc3d83e
    claim: "Independent retention review reproduced protected cycle deletion: a frozen artifact directory reference with canonical slashes protects all7cycles, while equivalent mixed-case Windows separators leave3eligible and authorized fixture apply deletes all3 with complete:true."
    result: failed
    revision: 76da6287c145240ce580ddf35fedb71aac53acc4
    environment: local-macos-arm64
    content: 84b7cd392431a504dc99aca3bb108d89429779d75e6d16c1ef5357001233b7c5
    observed: 2026-11-02T17:10:48Z
  - id: 2026-11-02-frozen-plaintext-artifact-path-aliases-now-protect-every
    aliases: OBS-product-test-4ca251f7695567122f3b2bcd
    claim: Frozen/plaintext artifact path aliases now protect every history cycle. The new regression failed before the fix and passes for four equivalent path spellings under backtick/double-quote wrappers; previews and authorized fixture apply preserve every .chough byte. All19 history tests passed, plus formatting and warnings-denied Clippy.
    result: passed
    revision: 76da6287c145240ce580ddf35fedb71aac53acc4
    environment: local-macos-arm64
    content: 711f626412a6e3ca98fb05f2b3a0332e6da399d5ed807119d1cd76fddb6dd0cf
    fingerprint: sha256:d0be3acc4f06acf5e7bf9b2204b81da5f6fe50565aaa8e410bcf8df3179c8d6f
    observed: 2026-11-02T17:14:22Z
  - id: OBS-product-test-ba4c86bbf0c00f16f5636082
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
environment: local-macos-arm64
revision: 76da6287c145240ce580ddf35fedb71aac53acc4
found_by: Independent merge-readiness retention review at76da628
related: []
created: 2026-11-02T17:10:48Z
updated: 2026-11-02T17:44:17Z
---
# Frozen artifact directory aliases fail to protect history cycles

**What happened:** Windows-style directory references in frozen text are missed by history retention
Actual: Three referenced cycles are eligible and deleted with complete:true

**Steps:** A retained artifact names the cycles directory with mixed case or backslash separators

**Expected:** Equivalent references protect every matching cycle in preview and apply

**Why it matters:** P2: referenced proof can be removed despite its retained protection source

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T17:10:48Z · Reported (Independent merge-readiness retention review at76da628); evidence: OBS-product-test-063bada1f0797d7f9fc3d83e
- 2026-11-02T17:14:22Z · status: open -> verified. Structured and plaintext reference paths share normalization; equivalent frozen artifact aliases no longer expose protected cycles to pruning. Eight fixture variants and all19 history tests passed.; evidence: OBS-product-test-4ca251f7695567122f3b2bcd
- 2026-11-02T17:44:17Z · The final full verification also passes the new frozen-reference alias preservation case and all222tests; all original failures and proof remain retained.; evidence: OBS-product-test-ba4c86bbf0c00f16f5636082
