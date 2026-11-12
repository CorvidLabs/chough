---
kind: bug
id: 2026-11-12-when-chough-change-ship-runs-the-review-the-gate-closed-relay
aliases:
  - BUG-when-chough-change-ship-runs-the-review-the-gate-closed-relay-does-not-6c22f2030a487b14bd03b461
title: When chough change ship runs the review, the gate-closed relay does not name the review or its actor
status: verified
classification: product-defect
scope: in-scope
impact: A reader of the Timeline cannot tell a review ran for the gate, or as whom
criteria: []
requirements: [REQ-history-010, REQ-cli-059]
cases: []
evidence:
  - OBS-tc-cli-a-ship-records-the-review-it-ran-e37f004b73d0aa76f0102272
  - OBS-tc-cli-a-ship-records-the-review-it-ran-7ada09c2961ca49fed81d585
receipts:
  - id: 2026-11-12-tc-cli-a-ship-records-the-review-it-ran
    aliases: OBS-tc-cli-a-ship-records-the-review-it-ran-e37f004b73d0aa76f0102272
    claim: "Ran cargo test --locked --test change -- --exact a_ship_that_runs_the_review_records_it_on_the_gate_closed_relay for TC-cli-a-ship-records-the-review-it-ran: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test change -- --exact a_ship_that_runs_the_review_records_it_on_the_gate_closed_relay
    revision: 5ab111b75e7920d144891258a89bb7f653a04bbc
    environment: local-macos-arm64
    content: 15efa82450eaa11cbeb0b089859bb2384e546eeaecace5f8a9e70d1c7d26a6f8
    fingerprint: sha256:11b66ca8451f4dcdd73f9fb49e0ad79309c4a5802f924ac3d360a7e84afb9f62
    case: TC-cli-a-ship-records-the-review-it-ran
    case_digest: 004456711a208a2922a9bbdba216f913996ee90aad0d5477e76484a09830b4bd
    observed: 2026-11-12T07:29:30Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-a-ship-records-the-review-it-ran-14701e
    aliases: OBS-tc-cli-a-ship-records-the-review-it-ran-7ada09c2961ca49fed81d585
    claim: "Ran cargo test --locked --test change -- --exact a_ship_that_runs_the_review_records_it_on_the_gate_closed_relay for TC-cli-a-ship-records-the-review-it-ran: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact a_ship_that_runs_the_review_records_it_on_the_gate_closed_relay
    revision: 7986e008c32eaaa48c6d1ec029905556edf25ca9
    environment: local-macos-arm64
    content: 14f9035932ae8940e4737cbe2a5f6eeedeedac2f05e722fe0f2140e142f7f7dc
    fingerprint: sha256:c456fb709b1b5de0283e1275acc0af0e5cbc9b2700fcbb791588cac16f834e92
    case: TC-cli-a-ship-records-the-review-it-ran
    case_digest: 004456711a208a2922a9bbdba216f913996ee90aad0d5477e76484a09830b4bd
    observed: 2026-11-12T07:29:53Z
    duration_seconds: 2
environment: local-macos-arm64, tester build at ad44a0d9, debug, real SpecSync 6.3.0
revision: 7986e008c32eaaa48c6d1ec029905556edf25ca9
found_by: t38b, 6.3.0 re-run, area approvals and gates
related: []
created: 2026-11-12T07:29:25Z
updated: 2026-11-12T07:30:05Z
---
# When chough change ship runs the review, the gate-closed relay does not name the review or its actor

**What happened:** After a ship that ran specsync change review as the human, the gate-closed entry's relayed_by has decided_by, words, where and recorded_by but no native or actor
Actual: The close is passed only a summary, so native and actor stay unset

**Steps:** chough change ship <id> --close-gate <review gate> with the relay, with no current scoped review

**Expected:** relayed_by names native and actor when the gate drove a native step (REQ-history-010)

**Why it matters:** A reader of the Timeline cannot tell a review ran for the gate, or as whom

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T07:29:25Z · Reported (t38b, 6.3.0 re-run, area approvals and gates)
- 2026-11-12T07:30:05Z · status: open -> investigating. Red at 5ab111b7 (before its fix, SpecSync 6.3.0 on PATH): TC-cli-a-ship-records-the-review-it-ran failed for this bug's reason; fix in b5453b53.; evidence: OBS-tc-cli-a-ship-records-the-review-it-ran-e37f004b73d0aa76f0102272
- 2026-11-12T07:30:05Z · status: investigating -> verified. Green at 7986e008: TC-cli-a-ship-records-the-review-it-ran passed; change, relay, humans, direct_writes and native tests pass.; evidence: OBS-tc-cli-a-ship-records-the-review-it-ran-7ada09c2961ca49fed81d585
