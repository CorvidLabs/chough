---
kind: bug
id: 2026-11-02-snapshots-reuse-dated-counter-identities
aliases: [BUG-snapshots-reuse-dated-counter-identities-21340fe57fbad0d2cedb747c]
title: Snapshots reuse dated counter identities
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: [FLOW-12]
requirements: [REQ-cli-015]
cases: [TC-cli-slug-snapshot-allocation]
evidence:
  - OBS-slug-snapshot-allocation-b731a60c294f8de0175c34a2
  - OBS-tc-cli-slug-snapshot-allocation-ea1999d3c7388098074a17e6
  - OBS-tc-cli-slug-snapshot-allocation-6e6f67c6b9c8a661f088ecd3
  - OBS-tc-cli-slug-snapshot-allocation-1070e1a735b6a22ba273ea76
  - OBS-tc-cli-slug-snapshot-allocation-af22af889455b7ae173e7a35
receipts:
  - id: 2026-11-02-direct-independent-root-and-repeated-run-snapshot-regression
    aliases: OBS-slug-snapshot-allocation-b731a60c294f8de0175c34a2
    claim: Direct independent-root and repeated-run snapshot regression allocated the old dated ID 2026-11-02-1 instead of a random SNAP slug ID.
    result: failed
    revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
    environment: local-macos-arm64
    content: c269fe17329d96e67a5c532044ddc9a9d7c2e435e7e66f0923378c8fd383f5f6
    fingerprint: sha256:b90492c1a5abfe58592d8bb93dab09eef91c4944b40cb1cc83e95f396b773bb3
    observed: 2026-11-02T00:32:32Z
  - id: 2026-11-02-tc-cli-slug-snapshot-allocation
    aliases: OBS-tc-cli-slug-snapshot-allocation-ea1999d3c7388098074a17e6
    claim: "Ran cargo test --locked --test optional -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_snapshots for TC-cli-slug-snapshot-allocation: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test optional -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_snapshots
    revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
    environment: local-macos-arm64
    content: e262857779c52658af2b715d13bb0a137a7fd3653aa345ce8ec2eb33614bfffd
    fingerprint: sha256:634b8e3cca8e0b7ad7c9fea4b59f68e410dc850f68ad14c8b377ea9e65182dcb
    case: TC-cli-slug-snapshot-allocation
    case_digest: 6588c7737c95f31d281441e4844e26b6208c42c219bccd41dad67f9cf28978e8
    observed: 2026-11-02T01:10:23Z
    duration_seconds: 0
  - id: OBS-tc-cli-slug-snapshot-allocation-6e6f67c6b9c8a661f088ecd3
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-slug-snapshot-allocation-1070e1a735b6a22ba273ea76
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-02-tc-cli-slug-snapshot-allocation-d0f25b
    aliases: OBS-tc-cli-slug-snapshot-allocation-af22af889455b7ae173e7a35
    claim: "Ran cargo test --locked --test optional -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_snapshots for TC-cli-slug-snapshot-allocation: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test optional -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_snapshots
    revision: b118422c826b29a41df2eea38c58120f6dbc77ae
    environment: local-macos-arm64
    content: 1fcc70468c2694ed16f86958849b15503a9febf2c2c7b7b970d33878cb7fff47
    fingerprint: sha256:a06aa701dac1e3bccb548df802d62e8cddbef1adc2d28985b7254580034dfd9b
    case: TC-cli-slug-snapshot-allocation
    case_digest: 6588c7737c95f31d281441e4844e26b6208c42c219bccd41dad67f9cf28978e8
    observed: 2026-11-02T01:42:28Z
    duration_seconds: 0
environment: local-macos-arm64
revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
found_by: failure-first direct cargo regression
related: []
created: 2026-11-02T00:32:57Z
updated: 2026-11-02T01:42:28Z
---
# Snapshots reuse dated counter identities

**What happened:** Independent roots and repeated snapshots use dated counter directories instead of random SNAP slug identities.
Actual: The unfixed regression allocated 2026-11-02-1 and failed the required SNAP form assertion.

**Steps:** Run the direct independent-root and identical repeated-run snapshot regression.

**Expected:** Every identical snapshot has a distinct SNAP slug ID while source bytes, digests and timestamp chronology remain unchanged.

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T00:32:57Z · Reported (failure-first direct cargo regression); evidence: OBS-slug-snapshot-allocation-b731a60c294f8de0175c34a2
- 2026-11-02T00:32:57Z · Added despite similar reports: Separate librarian directory allocation, SNAP grammar, manifest-last publication and legacy dated index compatibility from validation JSON allocation.
- 2026-11-02T01:15:15Z · status: open -> verified. Takeover rerun passed the original focused regression with an automatically allocated new-form observation and inherited artifact path. Original failed observation and artifact retained. Native acceptance and full-suite verification remain separate.; evidence: OBS-tc-cli-slug-snapshot-allocation-ea1999d3c7388098074a17e6
- 2026-11-02T01:20:36Z · Focused rerun at committed production source 27c0964 passed with automatic allocation. Retains original failure and prior pass. Full-suite fixture gate remains separate.; evidence: OBS-tc-cli-slug-snapshot-allocation-6e6f67c6b9c8a661f088ecd3
- 2026-11-02T01:22:33Z · Final focused rerun after companion and fixture-definition updates at f4613ebf1673491877802fa2ac879a9cf482b267 passed. No full-suite or acceptance claim.; evidence: OBS-tc-cli-slug-snapshot-allocation-1070e1a735b6a22ba273ea76
- 2026-11-02T01:42:28Z · After the explicitly approved fixture-only patch, this focused case passed with automatic observation allocation. All original assertions and failure artifacts remain intact. Full product suites each pass 182 tests; native ordering still blocks the full verify lane.; evidence: OBS-tc-cli-slug-snapshot-allocation-af22af889455b7ae173e7a35
