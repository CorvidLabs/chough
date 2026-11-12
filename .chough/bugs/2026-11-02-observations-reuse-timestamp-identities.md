---
kind: bug
id: 2026-11-02-observations-reuse-timestamp-identities
aliases: [BUG-observations-reuse-timestamp-identities-0d697a3ddd838a3b679e4bcc]
title: Observations reuse timestamp identities
status: verified
classification: product-defect
scope: in-scope
impact: Independent roots and identical repeated runs do not receive the approved collision-resistant observation identities.
criteria: [FLOW-7, LIFE-2]
requirements: [REQ-cli-011, REQ-cli-017]
cases: [TC-cli-slug-observation-allocation]
evidence:
  - OBS-slug-observation-allocation-6f0a2f94c38b11d27e45a901
  - OBS-tc-cli-slug-observation-allocation-fb841732389861bd071a5f13
  - OBS-tc-cli-slug-observation-allocation-f02d9d7cc4d216e577cfce37
  - OBS-tc-cli-slug-observation-allocation-fbba5df6257bc8b5dde6e504
  - OBS-tc-cli-slug-observation-allocation-74cf834b8d5f25c963b434b3
receipts:
  - id: 2026-11-02-direct-regression-exited-101-independent-roots-and-repeated
    aliases: OBS-slug-observation-allocation-6f0a2f94c38b11d27e45a901
    claim: "Direct regression exited 101: independent roots and repeated runs allocated old-form observation tc-cli-0001-1793579540 instead of OBS slug plus 24 lowercase hex."
    result: failed
    revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
    environment: local-macos-arm64
    content: c269fe17329d96e67a5c532044ddc9a9d7c2e435e7e66f0923378c8fd383f5f6
    fingerprint: sha256:760625033ae67467e6673606fd020540bed647de3f65ebf6520e655293c89211
    observed: 2026-11-02T00:32:38Z
  - id: 2026-11-02-tc-cli-slug-observation-allocation
    aliases: OBS-tc-cli-slug-observation-allocation-fb841732389861bd071a5f13
    claim: "Ran cargo test --locked --test cases -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_observations for TC-cli-slug-observation-allocation: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_observations
    revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
    environment: local-macos-arm64
    content: e262857779c52658af2b715d13bb0a137a7fd3653aa345ce8ec2eb33614bfffd
    fingerprint: sha256:57cdb95a7a78b0a0923603cd5632f19a8a2a65a83649017bf63441ed7b97a96f
    case: TC-cli-slug-observation-allocation
    case_digest: b4c964639a94224b07de6ece9176357291fd784205072a5d3cd899925320eb9f
    observed: 2026-11-02T01:10:19Z
    duration_seconds: 9
  - id: OBS-tc-cli-slug-observation-allocation-f02d9d7cc4d216e577cfce37
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-slug-observation-allocation-fbba5df6257bc8b5dde6e504
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-02-tc-cli-slug-observation-allocation-c4de43
    aliases: OBS-tc-cli-slug-observation-allocation-74cf834b8d5f25c963b434b3
    claim: "Ran cargo test --locked --test cases -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_observations for TC-cli-slug-observation-allocation: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_observations
    revision: b118422c826b29a41df2eea38c58120f6dbc77ae
    environment: local-macos-arm64
    content: 1fcc70468c2694ed16f86958849b15503a9febf2c2c7b7b970d33878cb7fff47
    fingerprint: sha256:309a8f7e0e412cf23e317d96b955d2324223930c0c6f427d98d851f424ac6bfe
    case: TC-cli-slug-observation-allocation
    case_digest: b4c964639a94224b07de6ece9176357291fd784205072a5d3cd899925320eb9f
    observed: 2026-11-02T01:42:25Z
    duration_seconds: 1
environment: local-macos-arm64
revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
found_by: manual report
related: []
created: 2026-11-02T00:33:07Z
updated: 2026-11-02T01:42:25Z
---
# Observations reuse timestamp identities

**What happened:** Automatic observations use timestamps and local numeric suffixes instead of random slug identities.
Actual: The direct test observed old-form ID tc-cli-0001-1793579540 and exited 101.

**Steps:** Run the same case repeatedly in one root and in two independent roots with identical starting records.
1. cargo test --locked --test cases -- --exact independent_roots_and_identical_repeated_runs_allocate_distinct_observations

**Expected:** Every attempt receives a distinct OBS slug plus 24 lowercase hex ID and an inherited artifact path.

**Why it matters:** Independent roots and identical repeated runs do not receive the approved collision-resistant observation identities.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T00:33:07Z · Reported (manual report); evidence: OBS-slug-observation-allocation-6f0a2f94c38b11d27e45a901
- 2026-11-02T00:33:07Z · Added despite similar reports: The existing bug covers validation run allocation; this defect is observation and artifact allocation in cases and evidence.
- 2026-11-02T01:15:14Z · status: open -> verified. Takeover rerun passed the original focused regression with an automatically allocated new-form observation and inherited artifact path. Original failed observation and artifact retained. Native acceptance and full-suite verification remain separate.; evidence: OBS-tc-cli-slug-observation-allocation-fb841732389861bd071a5f13
- 2026-11-02T01:20:33Z · Focused rerun at committed production source 27c0964 passed with automatic allocation. Retains original failure and prior pass. Full-suite fixture gate remains separate.; evidence: OBS-tc-cli-slug-observation-allocation-f02d9d7cc4d216e577cfce37
- 2026-11-02T01:22:30Z · Final focused rerun after companion and fixture-definition updates at f4613ebf1673491877802fa2ac879a9cf482b267 passed. No full-suite or acceptance claim.; evidence: OBS-tc-cli-slug-observation-allocation-fbba5df6257bc8b5dde6e504
- 2026-11-02T01:42:25Z · After the explicitly approved fixture-only patch, this focused case passed with automatic observation allocation. All original assertions and failure artifacts remain intact. Full product suites each pass 182 tests; native ordering still blocks the full verify lane.; evidence: OBS-tc-cli-slug-observation-allocation-74cf834b8d5f25c963b434b3
