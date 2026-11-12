---
kind: bug
id: 2026-11-01-cases-check-cannot-link-a-requirement-from-an-approved
aliases: [BUG-0017]
title: cases check cannot link a requirement from an approved, unmaterialized delta
status: verified
classification: product-defect
scope: in-scope
impact: Cases run twice
criteria: [FLOW-7]
requirements: [REQ-cli-005]
cases: []
evidence:
  - tc-cli-0094-1793500286
  - bug-0017-repro-1
  - tc-cli-0094-1793500898
  - bug-0017-fixed-1
  - OBS-product-test-11dfd04637cfc41549d8bec2
  - OBS-product-test-61a4a912f4e142a100dd919e
  - OBS-product-test-694fad65ec7621ab2e3e2b32
  - OBS-tc-cli-skill-orders-the-change-check-before-new-requirement-cases-f2fe3e74105b6b50775d28cd
receipts:
  - id: 2026-11-01-tc-cli-0094
    aliases: tc-cli-0094-1793500286
    claim: "Ran cargo test --locked --test cases -- --exact an_unresolved_requirement_is_refused_with_the_check_first_hint for TC-cli-0094: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test cases -- --exact an_unresolved_requirement_is_refused_with_the_check_first_hint
    revision: fce3d98d9b4d093c77513c09bb23f97706f11fbe
    environment: local-macos-arm64
    content: 2576e5f2f6b87c584c38e7fc14434299f7863a6f516aeadd71f8ad9ca7a194e7
    fingerprint: sha256:8c9c8ef5a5e07d5a7928ef8591594a4952533b4762f0d755395f6598a4031c95
    case: TC-cli-0094
    case_digest: 058cef64fdff2d9ed0f88b66cc5503eaf63caaf73b195df6fc6485238ef273b2
    observed: 2026-11-01T02:31:27Z
  - id: 2026-11-01-tc-cli-0094-f4890f
    aliases: bug-0017-repro-1
    claim: At fce3d98 (unfixed), a cli case linking a requirement absent from specs/cli/requirements.md (as an unmaterialized delta ID would be) gave the bare 'requirement not found in contract companions' error with no hint; case run tc-cli-0094-1793500286 also failed.
    result: failed
    revision: fce3d98d9b4d093c77513c09bb23f97706f11fbe
    environment: local-macos-arm64
    content: 2576e5f2f6b87c584c38e7fc14434299f7863a6f516aeadd71f8ad9ca7a194e7
    fingerprint: sha256:b8e45cb461bcf6588d3bdee422a3553edae88eb0c34f15fc2d88bc03710be7cd
    case: TC-cli-0094
    case_digest: 058cef64fdff2d9ed0f88b66cc5503eaf63caaf73b195df6fc6485238ef273b2
    observed: 2026-11-01T02:32:07Z
  - id: 2026-11-01-tc-cli-0094-862e1e
    aliases: tc-cli-0094-1793500898
    claim: "Ran cargo test --locked --test cases -- --exact an_unresolved_requirement_is_refused_with_the_check_first_hint for TC-cli-0094: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact an_unresolved_requirement_is_refused_with_the_check_first_hint
    revision: 282c5bdfeb7bae5cc84f3ac8236652a4536080af
    environment: local-macos-arm64
    content: e9daf2396e35880630ab04c9ea0f4ccc9d1acd8a9b7b4b1f08954372cd329155
    fingerprint: sha256:6924c8da854a7a38ab4c13ce5b359cd64c034cda8ec01c83344c4f90cd165ddd
    case: TC-cli-0094
    case_digest: 058cef64fdff2d9ed0f88b66cc5503eaf63caaf73b195df6fc6485238ef273b2
    observed: 2026-11-01T02:41:39Z
  - id: 2026-11-01-tc-cli-0094-6d516c
    aliases: bug-0017-fixed-1
    claim: At 282c5bd (fixed), the same reproduction named specs/cli/requirements.md and gave the typo or not-yet-materialized hint to run specsync change check first; case run tc-cli-0094-1793500898 passed.
    result: passed
    revision: 282c5bdfeb7bae5cc84f3ac8236652a4536080af
    environment: local-macos-arm64
    content: e9daf2396e35880630ab04c9ea0f4ccc9d1acd8a9b7b4b1f08954372cd329155
    fingerprint: sha256:88a447fe7c6cb3cce07c7e5f080e6e292756308cee482d06df5c44a776c18ac2
    case: TC-cli-0094
    case_digest: 058cef64fdff2d9ed0f88b66cc5503eaf63caaf73b195df6fc6485238ef273b2
    observed: 2026-11-01T02:42:10Z
  - id: 2026-11-02-at-46790b4-unfixed-skill-text-a-disposable-shipping-sample
    aliases: OBS-product-test-11dfd04637cfc41549d8bec2
    claim: "At 46790b4 (unfixed skill text), a disposable shipping sample with an unchecked 'Record Chough cases' task in tasks.md: chough cases check exits 1 with the check-first hint and specsync change check exits 1 (tasks.md contains incomplete tasks), so the order the shipped skill allows deadlocks. Without the box, change check exits 0, materializes REQ-shipping-002 and cases check exits 0."
    result: failed
    revision: 46790b408f8f418bdbaafef86119c5a642c55941
    environment: linux-x86_64
    content: 934c2c9d8325186a70eea72fc6d6979e6e2342dbbd24d57af2aafb3a79c1f94c
    fingerprint: sha256:f5200f08fbbf3c5fed063734b8c735359c5fd6814ca8b73fc64317f472469c0f
    observed: 2026-11-02T15:58:17Z
  - id: 2026-11-02-before-the-skill-fix-the-two-new-package-tests-fail-the
    aliases: OBS-product-test-61a4a912f4e142a100dd919e
    claim: "Before the skill fix, the two new package tests fail: the 825a380 plan reference reads current instead of known-outdated, and the canonical plan reference lacks the ordering guidance."
    result: failed
    revision: 46790b408f8f418bdbaafef86119c5a642c55941
    environment: linux-x86_64
    content: 59e31c0d521046fb10a67f35008e8db8175e1264266f6f23125a001414bbf26f
    fingerprint: sha256:fbb1a5b9b3f27642d18fc35147aa9d0c3c398912f8cd5610010721d195f4aed6
    observed: 2026-11-02T15:59:16Z
  - id: 2026-11-02-after-the-skill-fix-the-two-package-tests-that-failed-in-obs
    aliases: OBS-product-test-694fad65ec7621ab2e3e2b32
    claim: "After the skill fix, the two package tests that failed in OBS-product-test-61a4a912f4e142a100dd919e pass: 825a380 plan, testing and accept copies read known-outdated, update to current, edited copies stay, and the canonical references carry the ordering guidance."
    result: passed
    revision: 46790b408f8f418bdbaafef86119c5a642c55941
    environment: linux-x86_64
    content: 59b7590451926a1df2941dc39cfbedf2b2d0bb005fb0d69cf33520e3daa293b1
    fingerprint: sha256:0a6b546f93c7b1fdf8ac4dfd720f90380cbea942a31df43cb19a7a6d60b3fb21
    observed: 2026-11-02T16:01:44Z
  - id: 2026-11-02-tc-cli-skill-orders-the-change-check-before-new-requirement
    aliases: OBS-tc-cli-skill-orders-the-change-check-before-new-requirement-cases-f2fe3e74105b6b50775d28cd
    claim: At the implementation tip after the scoped change check materialized REQ-cli-009, the canonical plan, testing and accept references say the scoped check precedes cases linking a requirement the change adds and that running or recording cases is not a tasks.md checkbox; no em or en dash; chough skill status reports all 48 host files current.
    result: passed
    revision: 46790b408f8f418bdbaafef86119c5a642c55941
    environment: repository-text
    content: 59b7590451926a1df2941dc39cfbedf2b2d0bb005fb0d69cf33520e3daa293b1
    fingerprint: sha256:4ab4028983f8c95e35f991a550031fc4b60c78a6657d9ffd74fe61184eb405eb
    case: TC-cli-skill-orders-the-change-check-before-new-requirement-cases
    case_digest: c95a3cb3f548d3bbbe233dfa84743012078e9a158875fbe0520b9aa8b32ff30e
    observed: 2026-11-02T16:01:44Z
environment: local-macos-arm64
revision: 2756dc24a821bbb92021455c704ef7a177123ffa
found_by: CorvidLabs/chough#11 (Raven run); reproduced on main 2756dc2
related: []
created: 2026-11-01T00:57:53Z
updated: 2026-11-02T16:01:52Z
---
# cases check cannot link a requirement from an approved, unmaterialized delta

**What happened:** requirement not found in contract companions for a REQ that exists only in the approved delta
Actual: A bare not-found error; change check refuses while the case task is open

**Steps:** Case linked to a new requirement before change check materializes it
1. Approve a change adding REQ-m-N
2. Link a case to REQ-m-N; chough cases check

**Expected:** An actionable error naming the ordering, and documented order without double runs

**Why it matters:** Cases run twice

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T00:57:53Z · Reported (CorvidLabs/chough#11 (Raven run); reproduced on main 2756dc2)
- 2026-11-01T02:32:14Z · Error half reproduced at fce3d98 before any fix: case TC-cli-0094 failed and the reproduction showed the bare not-found error.; evidence: tc-cli-0094-1793500286, bug-0017-repro-1
- 2026-11-01T02:42:11Z · Error half fixed by the bundle hygiene change: the unchanged case TC-cli-0094 and the reproduction pass at 282c5bd. The bug stays open: its documented-order half (no double runs) belongs to the evidence-freshness change.; evidence: tc-cli-0094-1793500898, bug-0017-fixed-1
- 2026-11-02T15:58:22Z · Ordering half reproduced on Linux at 46790b4 before the skill fix (issue #11, native change case-recording-order-...): an unchecked case-recording task blocks the change check that would let cases check link the new requirement.; evidence: OBS-product-test-11dfd04637cfc41549d8bec2
- 2026-11-02T16:01:52Z · status: open -> verified. Documented-order half fixed in the shipped skill (plan, testing, accept references): package tests failed before the fix and pass after; the agent-assisted case review passed at the implementation tip. TC-cli-0094 still passes (OBS-tc-cli-0094-b36b0ba7e05a14833ddbe65f), so REQ-cli-005's refusal and hint are unchanged.; evidence: OBS-product-test-61a4a912f4e142a100dd919e, OBS-product-test-694fad65ec7621ab2e3e2b32, OBS-tc-cli-skill-orders-the-change-check-before-new-requirement-cases-f2fe3e74105b6b50775d28cd
