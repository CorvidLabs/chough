---
kind: bug
id: 2026-11-02-compact-next-fixture-writes-an-obsolete-observation-id
aliases: [BUG-compact-next-fixture-writes-an-obsolete-observation-id-d23860cbf663336b9f83ae75]
title: Compact next fixture writes an obsolete observation ID
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: [LIFE-3]
requirements: [REQ-cli-021]
cases: [TC-cli-0120]
evidence: [OBS-tc-cli-0120-76a960d09d62f9e548d773cf, OBS-tc-cli-0120-a8f83f0b745b921305211bfd]
receipts:
  - id: 2026-11-02-tc-cli-0120
    aliases: OBS-tc-cli-0120-76a960d09d62f9e548d773cf
    claim: "Ran cargo test --locked --test next -- --exact both_forms_stay_within_the_cap_with_totals_for_large_records for TC-cli-0120: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test next -- --exact both_forms_stay_within_the_cap_with_totals_for_large_records
    revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
    environment: local-macos-arm64
    content: e262857779c52658af2b715d13bb0a137a7fd3653aa345ce8ec2eb33614bfffd
    fingerprint: sha256:d2b234f1e42fc289ad8629ccc251c933163b7052c07dc56cc7db78137292b3d4
    case: TC-cli-0120
    case_digest: 9f386b129c8042c3c14877daf78a5d365c2396d8eabd1b43993a15f724707a89
    observed: 2026-11-02T01:19:02Z
    duration_seconds: 1
  - id: 2026-11-02-tc-cli-0120-c651f8
    aliases: OBS-tc-cli-0120-a8f83f0b745b921305211bfd
    claim: "Ran cargo test --locked --test next -- --exact both_forms_stay_within_the_cap_with_totals_for_large_records for TC-cli-0120: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact both_forms_stay_within_the_cap_with_totals_for_large_records
    revision: b118422c826b29a41df2eea38c58120f6dbc77ae
    environment: local-macos-arm64
    content: 1fcc70468c2694ed16f86958849b15503a9febf2c2c7b7b970d33878cb7fff47
    fingerprint: sha256:a9b72d3209cdb1499eb650c938912208fef7de615b40b19259df32ad147ab730
    case: TC-cli-0120
    case_digest: 9f386b129c8042c3c14877daf78a5d365c2396d8eabd1b43993a15f724707a89
    observed: 2026-11-02T01:42:22Z
    duration_seconds: 3
environment: local-macos-arm64
revision: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
found_by: Takeover regression test
related: []
created: 2026-11-02T01:19:03Z
updated: 2026-11-02T01:42:23Z
---
# Compact next fixture writes an obsolete observation ID

**What happened:** The compact-next output-bound fixture fails at evidence creation before its report assertions run.
Actual: evidence add rejects current-one as an old-style ID on a new write.

**Steps:** Run TC-cli-0120 after the approved slug observation identity implementation.
1. chough cases run TC-cli-0120 --environment local-macos-arm64

**Expected:** Create a new observation automatically and reference the returned ID while preserving every report assertion.

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T01:19:03Z · Reported (Takeover regression test); evidence: OBS-tc-cli-0120-76a960d09d62f9e548d773cf
- 2026-11-02T01:19:03Z · Added despite similar reports: BUG-0025 covers archived done action wording; this is test fixture construction excluded from the identity change affected_paths.
- 2026-11-02T01:42:23Z · status: open -> verified; scope: out-of-scope -> in-scope. After the explicitly approved fixture-only patch, this focused case passed with automatic observation allocation. All original assertions and failure artifacts remain intact. Full product suites each pass 182 tests; native ordering still blocks the full verify lane.; evidence: OBS-tc-cli-0120-a8f83f0b745b921305211bfd
