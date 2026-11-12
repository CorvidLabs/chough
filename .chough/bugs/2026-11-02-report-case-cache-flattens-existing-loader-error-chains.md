---
kind: bug
id: 2026-11-02-report-case-cache-flattens-existing-loader-error-chains
aliases: [BUG-report-case-cache-flattens-existing-loader-error-chains-ba1e469eeb3970b370b6456b]
title: Report case cache flattens existing loader error chains
status: verified
classification: product-defect
scope: blocking
impact: Not yet assessed.
criteria: [LIFE-7]
requirements: [REQ-cli-028]
cases: [TC-cli-report-trace-parity]
evidence: [OBS-product-test-553ec9b0ffe80fd46a96145d, OBS-product-test-4e2d19e426e48d18085f0b50]
receipts:
  - id: 2026-11-02-final-review-reproduced-a-compatibility-defect-in-the-first
    aliases: OBS-product-test-553ec9b0ffe80fd46a96145d
    claim: "Final review reproduced a compatibility defect in the first request-local cache: status flattens a malformed case loader error into one context string, whereas the original cases list path preserves its multiline anyhow cause chain. Both exit1 and stdoutempty, but diagnostics differ."
    result: failed
    revision: 825a3805530456005e361dfa6d96d93eee52cd09
    environment: local-macos-arm64
    content: d2f1383fbea4b65669fce020c498726b2fc5c227303cf627e3755bb1c2a8d1e9
    fingerprint: sha256:a6d2c0a675fbe927339869f192b1424f00cabf319618514b108b27a75fb2a7a1
    observed: 2026-11-02T16:38:51Z
  - id: 2026-11-02-the-strengthened-error-chain-regression-failed-before-the-fix
    aliases: OBS-product-test-4e2d19e426e48d18085f0b50
    claim: The strengthened error-chain regression failed before the fix, then all5focusedreportregressions passed after the cache preserved the typed error and original source chain. Independent direct loader/status reproduction now has byte-identical error output and matching exit1; trace on/off and managed records remain unchanged.
    result: passed
    revision: 825a3805530456005e361dfa6d96d93eee52cd09
    environment: local-macos-arm64
    content: 0ccbbb014958bc7f9c70afdddf4f7f7b0f1f63e573b2ede9ca9c0b521ea22da9
    fingerprint: sha256:0260981d99cf93143feed236e999bda485a365c19293402048d39d4657e647cd
    observed: 2026-11-02T16:46:36Z
environment: local-macos-arm64
revision: 825a3805530456005e361dfa6d96d93eee52cd09
found_by: Independent self-review of the new request-local cache after the initial full lane
related: []
created: 2026-11-02T16:38:51Z
updated: 2026-11-02T16:46:36Z
---
# Report case cache flattens existing loader error chains

**What happened:** Malformed case definitions produce different diagnostic formatting through report freshness than through the original structural loader
Actual: The report prints a flattened context string instead of the original multiline Caused by chain

**Steps:** A report with a bound case encounters malformed TOML after the new cache stores loader errors as strings

**Expected:** The existing loader error context and source chain remain unchanged while the suite is loaded at most once

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T16:38:51Z · Reported (Independent self-review of the new request-local cache after the initial full lane); evidence: OBS-product-test-553ec9b0ffe80fd46a96145d
- 2026-11-02T16:38:51Z · Added despite similar reports: This is a new cache error-chain compatibility defect, not the earlier actionable requirement hints or test filter newline issue
- 2026-11-02T16:46:36Z · status: open -> verified. Retained the original typed loader error behind request-local shared ownership; original multiline cause diagnostics now match direct loading byte-for-byte. The regression failed first and all5focusedtests pass after the fix.; evidence: OBS-product-test-4e2d19e426e48d18085f0b50
