---
kind: bug
id: 2026-11-02-validation-runs-reuse-timestamp-identities
aliases: [BUG-validation-runs-reuse-timestamp-identities-2070061bfd91bd06bdc0ba00]
title: Validation runs reuse timestamp identities
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: [FLOW-9, LIFE-7]
requirements: [REQ-cli-011, REQ-cli-013]
cases: [TC-cli-slug-validation-allocation]
evidence:
  - OBS-slug-validation-allocation-18b3c7409ad62ef510b47c92
  - OBS-tc-cli-slug-validation-allocation-b1eb4af179f28f15e8224463
  - OBS-tc-cli-slug-validation-allocation-71fe2212baa0de8c7e2caab1
  - OBS-tc-cli-slug-validation-allocation-f467e013bacf0b99eddafc60
  - OBS-tc-cli-slug-validation-allocation-87329eb940db06d9d61aa0c8
receipts:
  - id: 2026-11-02-direct-independent-root-and-repeated-run-validation-regression
    aliases: OBS-slug-validation-allocation-18b3c7409ad62ef510b47c92
    claim: Direct independent-root and repeated-run validation regression allocated the old timestamp ID tc-cli-0001-1793579525 instead of a random VAL slug ID.
    result: failed
    revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
    environment: local-macos-arm64
    content: c269fe17329d96e67a5c532044ddc9a9d7c2e435e7e66f0923378c8fd383f5f6
    fingerprint: sha256:66c97678f658caf414a92e6fa597b8962de7c7bcc1c6da18cd777b8fd8ab0eb9
    observed: 2026-11-02T00:32:27Z
  - id: 2026-11-02-tc-cli-slug-validation-allocation
    aliases: OBS-tc-cli-slug-validation-allocation-b1eb4af179f28f15e8224463
    claim: "Ran cargo test --locked --test optional -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_validation_runs for TC-cli-slug-validation-allocation: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test optional -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_validation_runs
    revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
    environment: local-macos-arm64
    content: e262857779c52658af2b715d13bb0a137a7fd3653aa345ce8ec2eb33614bfffd
    fingerprint: sha256:4b6032017809a34ae7b4268312e36f34b510d54a020ea8a9d1596115cd8439ef
    case: TC-cli-slug-validation-allocation
    case_digest: f3d269885c7e794742abee7801915f2a731c07eefe9d8f762b5abb3cccfc28c3
    observed: 2026-11-02T01:10:22Z
    duration_seconds: 2
  - id: OBS-tc-cli-slug-validation-allocation-71fe2212baa0de8c7e2caab1
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-slug-validation-allocation-f467e013bacf0b99eddafc60
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-02-tc-cli-slug-validation-allocation-4b139a
    aliases: OBS-tc-cli-slug-validation-allocation-87329eb940db06d9d61aa0c8
    claim: "Ran cargo test --locked --test optional -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_validation_runs for TC-cli-slug-validation-allocation: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test optional -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_validation_runs
    revision: b118422c826b29a41df2eea38c58120f6dbc77ae
    environment: local-macos-arm64
    content: 1fcc70468c2694ed16f86958849b15503a9febf2c2c7b7b970d33878cb7fff47
    fingerprint: sha256:3e1d26d3eb6bb1121f5e2926f1e6b1216740ca189ec9957d3b621692315b8d5f
    case: TC-cli-slug-validation-allocation
    case_digest: f3d269885c7e794742abee7801915f2a731c07eefe9d8f762b5abb3cccfc28c3
    observed: 2026-11-02T01:42:27Z
    duration_seconds: 1
environment: local-macos-arm64
revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
found_by: failure-first direct cargo regression
related: []
created: 2026-11-02T00:32:45Z
updated: 2026-11-02T01:42:27Z
---
# Validation runs reuse timestamp identities

**What happened:** Independent roots and repeated validation runs use timestamp and counter IDs instead of random VAL slug identities.
Actual: The unfixed regression allocated tc-cli-0001-1793579525 and failed the required VAL form assertion.

**Steps:** Run the direct independent-root and identical repeated-run validation regression.

**Expected:** Every new run has a distinct VAL slug ID and every attempt has its own OBS ID and inherited artifact.

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T00:32:45Z · Reported (failure-first direct cargo regression); evidence: OBS-slug-validation-allocation-18b3c7409ad62ef510b47c92
- 2026-11-02T01:15:14Z · status: open -> verified. Takeover rerun passed the original focused regression with an automatically allocated new-form observation and inherited artifact path. Original failed observation and artifact retained. Native acceptance and full-suite verification remain separate.; evidence: OBS-tc-cli-slug-validation-allocation-b1eb4af179f28f15e8224463
- 2026-11-02T01:20:35Z · Focused rerun at committed production source 27c0964 passed with automatic allocation. Retains original failure and prior pass. Full-suite fixture gate remains separate.; evidence: OBS-tc-cli-slug-validation-allocation-71fe2212baa0de8c7e2caab1
- 2026-11-02T01:22:32Z · Final focused rerun after companion and fixture-definition updates at f4613ebf1673491877802fa2ac879a9cf482b267 passed. No full-suite or acceptance claim.; evidence: OBS-tc-cli-slug-validation-allocation-f467e013bacf0b99eddafc60
- 2026-11-02T01:42:27Z · After the explicitly approved fixture-only patch, this focused case passed with automatic observation allocation. All original assertions and failure artifacts remain intact. Full product suites each pass 182 tests; native ordering still blocks the full verify lane.; evidence: OBS-tc-cli-slug-validation-allocation-87329eb940db06d9d61aa0c8
