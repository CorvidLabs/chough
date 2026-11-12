---
kind: bug
id: 2026-10-31-bug-verification-accepts-a-pass-recorded-before-the-failure
aliases: [BUG-0014]
title: Bug verification accepts a pass recorded before the failure
status: verified
classification: product-defect
scope: in-scope
impact: A bug can read as verified on stale evidence
criteria: [LIFE-5]
requirements: [REQ-cli-012]
cases: []
evidence:
  - bug-0014-repro-1
  - bug-0014-fixed-1
  - tc-cli-0076-1793498655
  - tc-cli-0077-1793498658
  - tc-cli-0078-1793498660
  - tc-cli-0079-1793498662
receipts:
  - id: 2026-11-01-before-the-fix-a-bug-whose-only-pass-is-older-than-or-in-the
    aliases: bug-0014-repro-1
    claim: Before the fix, a bug whose only pass is older than, or in the same second as, its latest failure reads reproduced-and-later-passed on every surface, and verified is accepted for a bug with no linked failure
    result: failed
    revision: a386c132598916c6572a633da455b169b825d3e9
    environment: local-macos-arm64
    fingerprint: sha256:f18211b4125c5fcfa72e2e0cef1b19a7bb132ec392953a076eecbe4f0c556ddd
    observed: 2026-11-01T01:31:33Z
  - id: 2026-11-01-tc-cli-0076
    aliases: bug-0014-fixed-1
    claim: After the fix, an older or same-second pass leaves the bug reproduced on every surface and verified is refused naming the failure; a later pass verifies; with no linked failure verified is refused
    result: passed
    revision: e806f783b34792c76961edfb24d42aa73764e6fa
    environment: local-macos-arm64
    content: 7eb66be2577f9e78895e0a93951cf202ab0bfe83611034719cfce6dfd4190e27
    fingerprint: sha256:e7a72878f9241ac363bc16d9f22d895dde40b34f0b329d6bb71ff95c8fb1a443
    case: TC-cli-0076
    case_digest: 5e0b685158277684219c9dfd3502b338659e6090ed8209006d48f7a5eb272546
    observed: 2026-11-01T01:40:33Z
  - id: 2026-11-01-tc-cli-0076-6df383
    aliases: tc-cli-0076-1793498655
    claim: "Ran cargo test --locked --test bugs -- --exact a_pass_observed_before_the_latest_failure_does_not_verify_a_bug for TC-cli-0076: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_pass_observed_before_the_latest_failure_does_not_verify_a_bug
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:afc436c3a13c4645cdcb79ead4875c3883c1df44a13798301d6ebee6e36252e5
    case: TC-cli-0076
    case_digest: 5e0b685158277684219c9dfd3502b338659e6090ed8209006d48f7a5eb272546
    observed: 2026-11-01T02:04:17Z
  - id: 2026-11-01-tc-cli-0077
    aliases: tc-cli-0077-1793498658
    claim: "Ran cargo test --locked --test bugs -- --exact a_pass_observed_after_the_latest_failure_verifies_a_bug for TC-cli-0077: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_pass_observed_after_the_latest_failure_verifies_a_bug
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:3de7650983be971abbfb270af609f80d7623a6d3fee75b7ba721f21b49c31fb1
    case: TC-cli-0077
    case_digest: 5c39b812f2e7776b2bb2c5f78ee4421cd0340bdd2b7280d1fc9cda1908798cb9
    observed: 2026-11-01T02:04:20Z
  - id: 2026-11-01-tc-cli-0078
    aliases: tc-cli-0078-1793498660
    claim: "Ran cargo test --locked --test bugs -- --exact a_pass_in_the_same_second_as_the_failure_does_not_verify_a_bug for TC-cli-0078: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_pass_in_the_same_second_as_the_failure_does_not_verify_a_bug
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:c340afc0bbd55ac8d63347a39c7ebd1a3c3368c0a31943394d9f8d16647560c9
    case: TC-cli-0078
    case_digest: ef231f853a0ff684cd29c91bf8a98371fad92c9e01b7b3c13519309ffa1be516
    observed: 2026-11-01T02:04:22Z
  - id: 2026-11-01-tc-cli-0079
    aliases: tc-cli-0079-1793498662
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_without_a_linked_failure_cannot_be_verified for TC-cli-0079: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_without_a_linked_failure_cannot_be_verified
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:de600454e465a23e4d5e22e4d9e899634298b35d66fbf17eac78d6d4ebcd5d60
    case: TC-cli-0079
    case_digest: 1a2705054f562dfed2ed941e734c627144dd5010aff4f5a7606aa32cdced39a3
    observed: 2026-11-01T02:04:25Z
environment: local-macos-arm64
revision: b07f84901b3c90e250d284591dcc298bca785586
found_by: Release-candidate retention demonstration verifier, 2026-10-31
related: []
created: 2026-10-31T23:33:41Z
updated: 2026-11-01T02:05:06Z
---
# Bug verification accepts a pass recorded before the failure

**What happened:** verification() reports reproduced-and-later-passed when any present linked observation passed
Actual: Order is not checked

**Steps:** A bug linked to a passing observation older than its failing reproduction
1. Link an older passed observation and a newer failed one to a bug
2. chough bug show

**Expected:** Only a pass observed after the reproduction counts as later-passed

**Why it matters:** A bug can read as verified on stale evidence

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T23:33:41Z · Reported (Release-candidate retention demonstration verifier, 2026-10-31)
- 2026-11-01T01:17:09Z · scope: out-of-scope -> in-scope. Moved in scope for draft change keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after (grouping agreed by the human; scope approval still pending).
- 2026-11-01T01:31:34Z · Reproduced before the fix by the new tests; see .chough/artifacts/bug-0014-reproduction.log.; evidence: bug-0014-repro-1
- 2026-11-01T01:40:33Z · status: open -> verified. Fixed in 4bf46da; the reproduction tests pass at e806f78 (.chough/artifacts/bug-0014-fixed-reproduction.log).; evidence: bug-0014-fixed-1
- 2026-11-01T02:05:06Z · The reproduction was a test-log observation recorded with evidence add before the fix, not a case-run observation: the regression case definitions TC-cli-0076 to TC-cli-0079 arrived with the fix commit 4bf46da, so they could not run at the unfixed tip. This note links their current passing case-run observations at the review-fix head.; evidence: tc-cli-0076-1793498655, tc-cli-0077-1793498658, tc-cli-0078-1793498660, tc-cli-0079-1793498662
