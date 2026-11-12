---
kind: bug
id: 2026-11-02-slug-case-ids-are-rejected
aliases: [BUG-slug-case-ids-are-rejected-e735f2070856c5421707ae14]
title: Slug case IDs are rejected
status: verified
classification: product-defect
scope: in-scope
impact: Approved slug case definitions cannot be loaded or executed.
criteria: [FLOW-7, LIFE-2]
requirements: [REQ-cli-009]
cases: [TC-cli-slug-case-compatibility]
evidence:
  - OBS-slug-case-compatibility-47d2a9106cb83ef519a0c674
  - OBS-tc-cli-slug-case-compatibility-649b7f61182d9e2cbf4c4c38
  - OBS-tc-cli-slug-case-compatibility-a744b8da5fa92a132e471637
  - OBS-tc-cli-slug-case-compatibility-a5f05dbb3b838dedd3c3b422
  - OBS-tc-cli-slug-case-compatibility-46e48b2d212fe44a838f3735
receipts:
  - id: 2026-11-02-direct-regression-exited-101-cases-list-rejected-tc-cli
    aliases: OBS-slug-case-compatibility-47d2a9106cb83ef519a0c674
    claim: "Direct regression exited 101: cases list rejected TC-cli-curated-login-check with id must look like TC-cli-0001."
    result: failed
    revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
    environment: local-macos-arm64
    content: c269fe17329d96e67a5c532044ddc9a9d7c2e435e7e66f0923378c8fd383f5f6
    fingerprint: sha256:90d6757bc3592cf59664ef206df7ff39265663d8df6c871d1747124a7160ce07
    observed: 2026-11-02T00:32:39Z
  - id: 2026-11-02-tc-cli-slug-case-compatibility
    aliases: OBS-tc-cli-slug-case-compatibility-649b7f61182d9e2cbf4c4c38
    claim: "Ran cargo test --locked --test cases -- --exact slug_case_ids_validate_without_changing_legacy_ids_digests_or_global_uniqueness for TC-cli-slug-case-compatibility: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact slug_case_ids_validate_without_changing_legacy_ids_digests_or_global_uniqueness
    revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
    environment: local-macos-arm64
    content: e262857779c52658af2b715d13bb0a137a7fd3653aa345ce8ec2eb33614bfffd
    fingerprint: sha256:198afad5bbd4d4129a1881610377907734a1f91c7e1b9c7f6dac2c2085d0de70
    case: TC-cli-slug-case-compatibility
    case_digest: 228ecdadf163d013d182b2931cc6eee68dcaba68eedaa9b252ae9db4cd8bcc95
    observed: 2026-11-02T01:10:24Z
    duration_seconds: 0
  - id: OBS-tc-cli-slug-case-compatibility-a744b8da5fa92a132e471637
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-slug-case-compatibility-a5f05dbb3b838dedd3c3b422
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-02-tc-cli-slug-case-compatibility-dc5a75
    aliases: OBS-tc-cli-slug-case-compatibility-46e48b2d212fe44a838f3735
    claim: "Ran cargo test --locked --test cases -- --exact slug_case_ids_validate_without_changing_legacy_ids_digests_or_global_uniqueness for TC-cli-slug-case-compatibility: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact slug_case_ids_validate_without_changing_legacy_ids_digests_or_global_uniqueness
    revision: b118422c826b29a41df2eea38c58120f6dbc77ae
    environment: local-macos-arm64
    content: 1fcc70468c2694ed16f86958849b15503a9febf2c2c7b7b970d33878cb7fff47
    fingerprint: sha256:7a36ab18691020b3e4ea05e208c4826f02cbc1ccdb12a543763b1c14e099a134
    case: TC-cli-slug-case-compatibility
    case_digest: 228ecdadf163d013d182b2931cc6eee68dcaba68eedaa9b252ae9db4cd8bcc95
    observed: 2026-11-02T01:42:29Z
    duration_seconds: 0
environment: local-macos-arm64
revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
found_by: manual report
related: []
created: 2026-11-02T00:33:00Z
updated: 2026-11-02T01:42:30Z
---
# Slug case IDs are rejected

**What happened:** Case validation accepts only numeric IDs and rejects approved curated slug identities.
Actual: The direct test observed id must look like TC-cli-0001 and exited 101.

**Steps:** Load a case definition whose ID is TC-cli-curated-login-check.
1. cargo test --locked --test cases -- --exact slug_case_ids_validate_without_changing_legacy_ids_digests_or_global_uniqueness

**Expected:** Lowercase bounded slug case IDs validate while legacy IDs and digests remain stable and duplicates refuse globally.

**Why it matters:** Approved slug case definitions cannot be loaded or executed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T00:33:00Z · Reported (manual report); evidence: OBS-slug-case-compatibility-47d2a9106cb83ef519a0c674
- 2026-11-02T01:15:15Z · status: open -> verified. Takeover rerun passed the original focused regression with an automatically allocated new-form observation and inherited artifact path. Original failed observation and artifact retained. Native acceptance and full-suite verification remain separate.; evidence: OBS-tc-cli-slug-case-compatibility-649b7f61182d9e2cbf4c4c38
- 2026-11-02T01:20:37Z · Focused rerun at committed production source 27c0964 passed with automatic allocation. Retains original failure and prior pass. Full-suite fixture gate remains separate.; evidence: OBS-tc-cli-slug-case-compatibility-a744b8da5fa92a132e471637
- 2026-11-02T01:22:34Z · Final focused rerun after companion and fixture-definition updates at f4613ebf1673491877802fa2ac879a9cf482b267 passed. No full-suite or acceptance claim.; evidence: OBS-tc-cli-slug-case-compatibility-a5f05dbb3b838dedd3c3b422
- 2026-11-02T01:42:30Z · After the explicitly approved fixture-only patch, this focused case passed with automatic observation allocation. All original assertions and failure artifacts remain intact. Full product suites each pass 182 tests; native ordering still blocks the full verify lane.; evidence: OBS-tc-cli-slug-case-compatibility-46e48b2d212fe44a838f3735
