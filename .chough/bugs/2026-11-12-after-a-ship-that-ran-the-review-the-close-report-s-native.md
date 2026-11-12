---
kind: bug
id: 2026-11-12-after-a-ship-that-ran-the-review-the-close-report-s-native
aliases:
  - BUG-after-a-ship-that-ran-the-review-the-close-report-s-native-step-says-ra-fa4ae334a3ac5df1ec0d498e
title: After a ship that ran the review, the close report's native_step says ran null while its why says the review ran
status: verified
classification: product-defect
scope: in-scope
impact: A reader of the output sees a contradiction; the recorded relay is right
criteria: []
requirements: [REQ-cli-059]
cases: []
evidence:
  - OBS-tc-cli-a-ship-reports-the-review-it-ran-37ee2939244c07df013faaad
  - OBS-tc-cli-a-ship-reports-the-review-it-ran-d6ee6645146cecdfc8c27744
receipts:
  - id: 2026-11-12-tc-cli-a-ship-reports-the-review-it-ran
    aliases: OBS-tc-cli-a-ship-reports-the-review-it-ran-37ee2939244c07df013faaad
    claim: "Ran cargo test --locked --test change -- --exact a_ship_that_runs_the_review_reports_it_in_native_step for TC-cli-a-ship-reports-the-review-it-ran: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test change -- --exact a_ship_that_runs_the_review_reports_it_in_native_step
    revision: f9c268fd243e6614f0054b4ce62aa5333695ea4c
    environment: local-macos-arm64
    content: 178603b27723196cd051013d6fb93e64bce082adb4af1fb1de2fe52bb8387aad
    fingerprint: sha256:92e982f866d36299767f503af30b57815c76513430b58f804e53247579aa0742
    case: TC-cli-a-ship-reports-the-review-it-ran
    case_digest: 8091cd53f515c0cc6a1f4992705bfb49996d311a56b205ad67920ad6001ece4b
    observed: 2026-11-12T07:38:19Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-a-ship-reports-the-review-it-ran-d5e2f9
    aliases: OBS-tc-cli-a-ship-reports-the-review-it-ran-d6ee6645146cecdfc8c27744
    claim: "Ran cargo test --locked --test change -- --exact a_ship_that_runs_the_review_reports_it_in_native_step for TC-cli-a-ship-reports-the-review-it-ran: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact a_ship_that_runs_the_review_reports_it_in_native_step
    revision: 935422b894c67b01bfe21ae7b83f71c58aea7bf4
    environment: local-macos-arm64
    content: 524bb0bc843e0caf0acc1d91696a68dfac75e09e726b2c4c04bdec10dda541af
    fingerprint: sha256:4b38e5ea8a0f9198c212e8380ae877f7bd286d96ff07b75e2a94131b5366e9f6
    case: TC-cli-a-ship-reports-the-review-it-ran
    case_digest: 8091cd53f515c0cc6a1f4992705bfb49996d311a56b205ad67920ad6001ece4b
    observed: 2026-11-12T07:38:37Z
    duration_seconds: 2
environment: local-macos-arm64, tester build at 9c30b881, debug, real SpecSync 6.3.0
revision: 9c30b8817bc403d447e7469bb03a9fbc3351542e
found_by: t38b, A3-1 retest residual
related: []
created: 2026-11-12T07:37:38Z
updated: 2026-11-12T07:38:37Z
---
# After a ship that ran the review, the close report's native_step says ran null while its why says the review ran

**What happened:** relayed_decision.native_step is {ran: null, why: 'ran specsync change review <id> as <actor> with verdict pass'}
Actual: The close reports only why, with ran null

**Steps:** chough change ship <id> --close-gate <review gate> with the relay, with no current scoped review

**Expected:** native_step names the step that ran, as a close that runs it does

**Why it matters:** A reader of the output sees a contradiction; the recorded relay is right

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T07:37:38Z · Reported (t38b, A3-1 retest residual)
- 2026-11-12T07:37:38Z · Added despite similar reports: A3-1 fixed the recorded relay (native and actor on the Timeline); this is the command's printed close report, whose native_step still says ran null
- 2026-11-12T07:38:37Z · status: open -> investigating. Red at f9c268fd (before its fix): native_step said ran null; fix in 935422b8.; evidence: OBS-tc-cli-a-ship-reports-the-review-it-ran-37ee2939244c07df013faaad
- 2026-11-12T07:38:37Z · status: investigating -> verified. Green at 935422b8: TC-cli-a-ship-reports-the-review-it-ran passed; clippy --all-targets clean; change, relay and humans tests pass.; evidence: OBS-tc-cli-a-ship-reports-the-review-it-ran-d6ee6645146cecdfc8c27744
