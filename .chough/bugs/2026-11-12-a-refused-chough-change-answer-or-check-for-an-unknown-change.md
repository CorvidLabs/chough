---
kind: bug
id: 2026-11-12-a-refused-chough-change-answer-or-check-for-an-unknown-change
aliases:
  - BUG-a-refused-chough-change-answer-or-check-for-an-unknown-change-id-leaves-c6d36b288696c77b268c7ac4
title: A refused chough change answer or check for an unknown change ID leaves a direct-write baseline under .chough/local/
status: verified
classification: product-defect
scope: in-scope
impact: Junk machine-local files, one per mistyped ID
criteria: []
requirements: [REQ-cli-057]
cases: []
evidence:
  - OBS-tc-cli-an-unknown-change-leaves-no-baseline-122ad3922d42a1221c6ea7e2
  - OBS-tc-cli-an-unknown-change-leaves-no-baseline-bea5a183a3da7b2474b12736
receipts:
  - id: 2026-11-12-tc-cli-an-unknown-change-leaves-no-baseline
    aliases: OBS-tc-cli-an-unknown-change-leaves-no-baseline-122ad3922d42a1221c6ea7e2
    claim: "Ran cargo test --locked --test change -- --exact a_refused_step_for_an_unknown_change_leaves_no_baseline for TC-cli-an-unknown-change-leaves-no-baseline: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test change -- --exact a_refused_step_for_an_unknown_change_leaves_no_baseline
    revision: e80c1b5ba2810e611250a4c413e259c23fda3dd9
    environment: local-macos-arm64
    content: 2af5a632a6f17be142b205c0d4693002bf931e5fb2598d3dacca99972ceaa6db
    fingerprint: sha256:bbba04ebe86f6888043ff20deb96b4a16be3dea8a31e1615bea468b9f70d5686
    case: TC-cli-an-unknown-change-leaves-no-baseline
    case_digest: ac6fe27c04a6c599f16773c53640fbf9b7d0de9065178e5e680a37d85e740889
    observed: 2026-11-12T07:29:35Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-an-unknown-change-leaves-no-baseline-f2666c
    aliases: OBS-tc-cli-an-unknown-change-leaves-no-baseline-bea5a183a3da7b2474b12736
    claim: "Ran cargo test --locked --test change -- --exact a_refused_step_for_an_unknown_change_leaves_no_baseline for TC-cli-an-unknown-change-leaves-no-baseline: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact a_refused_step_for_an_unknown_change_leaves_no_baseline
    revision: 7986e008c32eaaa48c6d1ec029905556edf25ca9
    environment: local-macos-arm64
    content: 14f9035932ae8940e4737cbe2a5f6eeedeedac2f05e722fe0f2140e142f7f7dc
    fingerprint: sha256:c47714ff698920d71e10072e31f8ce9138ddac18b7e4bafcabaf17f673b253ff
    case: TC-cli-an-unknown-change-leaves-no-baseline
    case_digest: ac6fe27c04a6c599f16773c53640fbf9b7d0de9065178e5e680a37d85e740889
    observed: 2026-11-12T07:29:54Z
    duration_seconds: 0
environment: local-macos-arm64, tester build at ad44a0d9, debug, real SpecSync 6.3.0
revision: 7986e008c32eaaa48c6d1ec029905556edf25ca9
found_by: t38b, 6.3.0 re-run, area approvals and gates
related: []
created: 2026-11-12T07:29:25Z
updated: 2026-11-12T07:30:05Z
---
# A refused chough change answer or check for an unknown change ID leaves a direct-write baseline under .chough/local/

**What happened:** chough change answer <unknown-id> ... or chough change check <unknown-id>, refused by SpecSync, leaves .chough/local/native-writes/<unknown-id>.json
Actual: The baseline is taken before SpecSync runs and written whatever its exit

**Steps:** A mistyped change ID in chough change answer or check

**Expected:** A step for a change with no workspace leaves nothing under .chough/ (REQ-cli-057)

**Why it matters:** Junk machine-local files, one per mistyped ID

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T07:29:25Z · Reported (t38b, 6.3.0 re-run, area approvals and gates)
- 2026-11-12T07:30:05Z · status: open -> investigating. Red at e80c1b5b (before its fix, SpecSync 6.3.0 on PATH): TC-cli-an-unknown-change-leaves-no-baseline failed for this bug's reason; fix in 42802428.; evidence: OBS-tc-cli-an-unknown-change-leaves-no-baseline-122ad3922d42a1221c6ea7e2
- 2026-11-12T07:30:05Z · status: investigating -> verified. Green at 7986e008: TC-cli-an-unknown-change-leaves-no-baseline passed; change, relay, humans, direct_writes and native tests pass.; evidence: OBS-tc-cli-an-unknown-change-leaves-no-baseline-bea5a183a3da7b2474b12736
