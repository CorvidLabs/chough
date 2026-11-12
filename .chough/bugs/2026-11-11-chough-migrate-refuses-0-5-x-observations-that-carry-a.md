---
kind: bug
id: 2026-11-11-chough-migrate-refuses-0-5-x-observations-that-carry-a
aliases:
  - BUG-chough-migrate-refuses-0-5-x-observations-that-carry-a-snapshot-input-i-3b098fc76b605d3f8d50e1f6
title: chough migrate refuses 0.5.x observations that carry a snapshot input inventory
status: verified
classification: product-defect
scope: blocking
impact: This repository, and any project that ran 0.5.x release checks, cannot migrate
criteria: [DRIVE-27]
requirements: [REQ-records-009, REQ-cli-017]
cases: [TC-cli-inventory-historical]
evidence:
  - OBS-tc-cli-inventory-historical-5eee57a4d6b333d2e3c163e5
  - OBS-tc-cli-inventory-historical-140254ee4702585e7583c1fa
receipts:
  - id: 2026-11-11-tc-cli-inventory-historical
    aliases: OBS-tc-cli-inventory-historical-5eee57a4d6b333d2e3c163e5
    claim: "Ran cargo test --locked --test evidence_inventory_compatibility -- --exact inventory_exact_nine_originals_stay_historical_partial_and_byte_identical for TC-cli-inventory-historical: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test evidence_inventory_compatibility -- --exact inventory_exact_nine_originals_stay_historical_partial_and_byte_identical
    revision: 981954086af9b7805e282579164ebf019edafd70
    environment: local-macos-arm64
    content: 2fcf8e52923a9548a466bed1e5a68250ea0dea4f01c2fba38f38ae31df983721
    fingerprint: sha256:a138ef4dad4aaddc2631350ee58e3fc82beaaee33de5057773627c0acb9e8b36
    case: TC-cli-inventory-historical
    case_digest: 558662abf69282b368c9e36a626b767bcb23cbe2d65dff8924a0aee3387d2de0
    observed: 2026-11-11T23:24:30Z
    duration_seconds: 1
  - id: 2026-11-11-tc-cli-inventory-historical-8f4552
    aliases: OBS-tc-cli-inventory-historical-140254ee4702585e7583c1fa
    claim: "Ran cargo test --locked --test evidence_inventory_compatibility -- --exact inventory_exact_nine_originals_stay_historical_partial_and_byte_identical for TC-cli-inventory-historical: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test evidence_inventory_compatibility -- --exact inventory_exact_nine_originals_stay_historical_partial_and_byte_identical
    revision: 393749b7090f82148de2d300b197f546ff8eded4
    environment: local-macos-arm64
    content: f9de5776121246ffd4c753853f2712d5f33bc2e128ce89595030591fa9831f0e
    fingerprint: sha256:9a84568f9773a8753c4b6a0d5cd02254843c2c03210ca270aa6700785d95679a
    case: TC-cli-inventory-historical
    case_digest: 558662abf69282b368c9e36a626b767bcb23cbe2d65dff8924a0aee3387d2de0
    observed: 2026-11-11T23:52:04Z
    duration_seconds: 2
environment: local-macos-arm64
revision: 981954086af9b7805e282579164ebf019edafd70
found_by: t37j's second migration rehearsal (plan Migration step 2) at d57d3aea
related: []
created: 2026-11-11T23:24:52Z
updated: 2026-11-11T23:52:04Z
---
# chough migrate refuses 0.5.x observations that carry a snapshot input inventory

**What happened:** The migration preview of this repository stops with 'unknown field input_inventory' on 9 tc-cli-release observations, so no plan is made
Actual: The 0.5.x reader denies the field, refusing the whole plan

**Steps:** Run chough migrate on a 0.5.x project holding an observation written since #203 with snapshot.input_inventory
1. Commit a 0.5.x observation whose snapshot holds input_inventory
2. Run chough migrate

**Expected:** The observation migrates; its input inventory follows #203's rule (absent, null or advisory data never changes authoritative freshness) and round-trips on the entry (REQ-cli-017, REQ-records-009)

**Why it matters:** This repository, and any project that ran 0.5.x release checks, cannot migrate

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T23:24:52Z · Reported (t37j's second migration rehearsal (plan Migration step 2) at d57d3aea); evidence: OBS-tc-cli-inventory-historical-5eee57a4d6b333d2e3c163e5
- 2026-11-11T23:52:04Z · status: open -> verified. Green after t37j's 15d88ca1: TC-cli-inventory-historical passes; all 8 inventory cases pass, and the migration preview of this repository makes a plan (t37j's rehearsal at 247da9d6: unaccounted [], the apply succeeds in a throwaway clone).; evidence: OBS-tc-cli-inventory-historical-140254ee4702585e7583c1fa
