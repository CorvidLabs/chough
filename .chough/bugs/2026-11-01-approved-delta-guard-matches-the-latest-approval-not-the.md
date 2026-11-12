---
kind: bug
id: 2026-11-01-approved-delta-guard-matches-the-latest-approval-not-the
aliases: [BUG-0020]
title: Approved-delta guard matches the latest approval, not the digest-matching one, for older workflow versions
status: verified
classification: requirement-gap
scope: in-scope
impact: "Low: SpecSync 6.1.1 records workflow_version 2 or later, so only changes approved by older SpecSync versions are affected. Found in the PR #23 review; deferred by 0xLeif because fixing it changes REQ-cli-022."
criteria: []
requirements: [REQ-cli-022]
cases: []
evidence:
  - OBS-tc-cli-bug-0020-guard-uses-effective-approval-e4e4ae20c3d4b3665a50fc19
  - OBS-tc-cli-bug-0020-guard-uses-effective-approval-01c9ea9bdadd02ee6d933114
receipts:
  - id: 2026-11-11-tc-cli-bug-0020-guard-uses-effective-approval
    aliases: OBS-tc-cli-bug-0020-guard-uses-effective-approval-e4e4ae20c3d4b3665a50fc19
    claim: "Ran cargo test --locked --test stages -- --exact the_delta_guard_compares_with_the_effective_approval for TC-cli-bug-0020-guard-uses-effective-approval: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test stages -- --exact the_delta_guard_compares_with_the_effective_approval
    revision: 4b783df6b771ff4f32d571c3bff203d7ead6fa63
    environment: local-macos-arm64
    content: 6218560a651ba38566a23b60f80b69a6f88d700590a9a49b11de1e2cbb91c964
    fingerprint: sha256:7d657783e33212d349c9f78c7882c26b57a95cd5cd8df31dcb42a31ac60c1ccf
    case: TC-cli-bug-0020-guard-uses-effective-approval
    case_digest: 169c8cf7de2e748b8ce4349b918824a99315db0ea2a64cdfae00f89b9e6d7066
    observed: 2026-11-11T19:32:57Z
    duration_seconds: 2
  - id: 2026-11-11-tc-cli-bug-0020-guard-uses-effective-approval-f5f880
    aliases: OBS-tc-cli-bug-0020-guard-uses-effective-approval-01c9ea9bdadd02ee6d933114
    claim: "Ran cargo test --locked --test stages -- --exact the_delta_guard_compares_with_the_effective_approval for TC-cli-bug-0020-guard-uses-effective-approval: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact the_delta_guard_compares_with_the_effective_approval
    revision: f9c0ae674c89f3a4c4a6c5b9d29057f22f7b9bf0
    environment: local-macos-arm64
    content: 4adc6e184b41d042b73284db8f143439ad1bd2bdc6fc389e3a5266068b430ba2
    fingerprint: sha256:4e422562042480a6ac49a96bb5879f8ab1bc98040b17c56eb647d2627ec8e4b9
    case: TC-cli-bug-0020-guard-uses-effective-approval
    case_digest: 169c8cf7de2e748b8ce4349b918824a99315db0ea2a64cdfae00f89b9e6d7066
    observed: 2026-11-11T19:42:51Z
    duration_seconds: 1
environment: local-macos-arm64
revision: 99a61ec92fe81e32f6bd7955357d23a00a0c86d7
found_by: manual report
related: []
created: 2026-11-01T18:45:37Z
updated: 2026-11-11T19:43:31Z
---
# Approved-delta guard matches the latest approval, not the digest-matching one, for older workflow versions

**What happened:** For a native change whose approvals were recorded by SpecSync workflow_version below 2, Chough's approved-delta guard compares the deltas with the latest definition approval, while SpecSync picks its effective approval by matching the definition digest.
Actual: The guard uses the latest definition approval, as REQ-cli-022 currently words it, so it can refuse or pass against a different approval than SpecSync.

**Steps:** A change with several definition approvals recorded by an older SpecSync workflow version, where the latest approval is not the one whose definition digest matches the current definition.

**Expected:** The guard compares the deltas with the same approval SpecSync treats as effective.

**Why it matters:** Low: SpecSync 6.1.1 records workflow_version 2 or later, so only changes approved by older SpecSync versions are affected. Found in the PR #23 review; deferred by 0xLeif because fixing it changes REQ-cli-022.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T18:45:37Z · Reported (manual report)
- 2026-11-11T19:33:16Z · status: open -> fix-pending-verification; scope: out-of-scope -> in-scope. Red on 0.5.0: TC-cli-bug-0020-guard-uses-effective-approval failed on leif/0.6.0-0020-red at 4b783df6 (step 1's 0.5.0 code with the test): the guard compared with the latest approval and refused although the effective, digest-matching one matched. Fixed in plan step 12 (PR #238): the guard reads summary.definition_digest and compares with the approval whose digest matches.; evidence: OBS-tc-cli-bug-0020-guard-uses-effective-approval-e4e4ae20c3d4b3665a50fc19
- 2026-11-11T19:43:31Z · status: fix-pending-verification -> verified. Fixed in plan step 12 (PR #238, d507cf28); the case passed after its 0.5.0 failure.; evidence: OBS-tc-cli-bug-0020-guard-uses-effective-approval-01c9ea9bdadd02ee6d933114
