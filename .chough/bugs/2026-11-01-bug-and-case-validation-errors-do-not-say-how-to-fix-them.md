---
kind: bug
id: 2026-11-01-bug-and-case-validation-errors-do-not-say-how-to-fix-them
aliases: [BUG-0019]
title: Bug and case validation errors do not say how to fix them
status: verified
classification: product-defect
scope: in-scope
impact: Avoidable trial and error
criteria: [LIFE-5, FLOW-7]
requirements: [REQ-cli-012, REQ-cli-009]
cases: []
evidence:
  - tc-cli-0092-1793500284
  - tc-cli-0093-1793500285
  - bug-0019-repro-1
  - tc-cli-0092-1793500895
  - tc-cli-0093-1793500897
  - bug-0019-fixed-1
receipts:
  - id: 2026-11-01-tc-cli-0092
    aliases: tc-cli-0092-1793500284
    claim: "Ran cargo test --locked --test bugs -- --exact a_scope_given_as_a_classification_and_the_reverse_name_the_right_flag for TC-cli-0092: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test bugs -- --exact a_scope_given_as_a_classification_and_the_reverse_name_the_right_flag
    revision: fce3d98d9b4d093c77513c09bb23f97706f11fbe
    environment: local-macos-arm64
    content: 2576e5f2f6b87c584c38e7fc14434299f7863a6f516aeadd71f8ad9ca7a194e7
    fingerprint: sha256:1a6f25ee4532ae04e8e3da892f0100ef4ac3d7699ffe244a2e21df9d16879d3f
    case: TC-cli-0092
    case_digest: e7077db93b58fdb69da20a59f0186755f46ec1c82a31734bb8261767279211f3
    observed: 2026-11-01T02:31:25Z
  - id: 2026-11-01-tc-cli-0093
    aliases: tc-cli-0093-1793500285
    claim: "Ran cargo test --locked --test cases -- --exact a_requirement_of_another_module_names_the_cases_file_that_binds_it for TC-cli-0093: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test cases -- --exact a_requirement_of_another_module_names_the_cases_file_that_binds_it
    revision: fce3d98d9b4d093c77513c09bb23f97706f11fbe
    environment: local-macos-arm64
    content: 2576e5f2f6b87c584c38e7fc14434299f7863a6f516aeadd71f8ad9ca7a194e7
    fingerprint: sha256:7c2a87451eded5eea1b784445157fed13115b2b1c11d94a1f4e9d1a4f2a8e09d
    case: TC-cli-0093
    case_digest: 0209056257b64c5ffd5778f62a8a66c247a21980718d72503fac20729151086f
    observed: 2026-11-01T02:31:26Z
  - id: 2026-11-01-tc-cli-0092-2ba297
    aliases: bug-0019-repro-1
    claim: At fce3d98 (unfixed), bug add --classification blocking and --scope product-defect gave bare value lists, --help listed no values, and a cli case linking REQ-checkout-ui-004 gave a bare not-found error; case runs tc-cli-0092-1793500284 and tc-cli-0093-1793500285 also failed.
    result: failed
    revision: fce3d98d9b4d093c77513c09bb23f97706f11fbe
    environment: local-macos-arm64
    content: 2576e5f2f6b87c584c38e7fc14434299f7863a6f516aeadd71f8ad9ca7a194e7
    fingerprint: sha256:35dd5007d2b496764f952a2bf44de10bf978f0861d9687083719d81aee1e8e8e
    case: TC-cli-0092
    case_digest: e7077db93b58fdb69da20a59f0186755f46ec1c82a31734bb8261767279211f3
    observed: 2026-11-01T02:32:06Z
  - id: 2026-11-01-tc-cli-0092-493868
    aliases: tc-cli-0092-1793500895
    claim: "Ran cargo test --locked --test bugs -- --exact a_scope_given_as_a_classification_and_the_reverse_name_the_right_flag for TC-cli-0092: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_scope_given_as_a_classification_and_the_reverse_name_the_right_flag
    revision: 282c5bdfeb7bae5cc84f3ac8236652a4536080af
    environment: local-macos-arm64
    content: e9daf2396e35880630ab04c9ea0f4ccc9d1acd8a9b7b4b1f08954372cd329155
    fingerprint: sha256:7e5536a4fe43ea4f784a1329319517b64a4e2ad7512a557f538c2fe9fec58a10
    case: TC-cli-0092
    case_digest: e7077db93b58fdb69da20a59f0186755f46ec1c82a31734bb8261767279211f3
    observed: 2026-11-01T02:41:36Z
  - id: tc-cli-0093-1793500897
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-01-tc-cli-0092-553154
    aliases: bug-0019-fixed-1
    claim: At 282c5bd (fixed), the same reproduction named --scope and --classification in the flag errors, --help listed the values, and the cross-module case named checkout-ui and its cases file; case runs tc-cli-0092-1793500895 and tc-cli-0093-1793500897 passed.
    result: passed
    revision: 282c5bdfeb7bae5cc84f3ac8236652a4536080af
    environment: local-macos-arm64
    content: e9daf2396e35880630ab04c9ea0f4ccc9d1acd8a9b7b4b1f08954372cd329155
    fingerprint: sha256:219db16dd55e37e8f97c37f362d3c26f645636ec5c1ab781626cf22f6864caae
    case: TC-cli-0092
    case_digest: e7077db93b58fdb69da20a59f0186755f46ec1c82a31734bb8261767279211f3
    observed: 2026-11-01T02:42:10Z
environment: local-macos-arm64
revision: 2756dc24a821bbb92021455c704ef7a177123ffa
found_by: CorvidLabs/chough#14 items 4 and 5 (Raven run); reproduced on main 2756dc2
related: []
created: 2026-11-01T00:58:02Z
updated: 2026-11-01T02:42:11Z
---
# Bug and case validation errors do not say how to fix them

**What happened:** A bug add with classification blocking fails without saying blocking is a scope value; a requirement from another module fails without naming the searched companions
Actual: Bare validation errors

**Steps:** chough bug add with a scope value as classification; a case linking another module's requirement
1. chough bug add --classification blocking (other fields valid)

**Expected:** Errors name the right flag or the searched contract

**Why it matters:** Avoidable trial and error

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T00:58:02Z · Reported (CorvidLabs/chough#14 items 4 and 5 (Raven run); reproduced on main 2756dc2)
- 2026-11-01T01:14:19Z · Revision corrected from 57d331e to 2756dc2: 57d331e was the amended-away first version of the bug-record commit (same source as 2756dc2 plus bug records, not an ancestor of 85650d9); the reproduction ran on main 2756dc2.
- 2026-11-01T02:32:13Z · Reproduced at fce3d98 before any fix: cases TC-cli-0092 and TC-cli-0093 failed and the reproduction showed the bare flag and cross-module errors.; evidence: tc-cli-0092-1793500284, tc-cli-0093-1793500285, bug-0019-repro-1
- 2026-11-01T02:42:11Z · status: open -> verified. Fixed by the bundle hygiene change: the unchanged cases TC-cli-0092 and TC-cli-0093 and the reproduction pass at 282c5bd, after the linked failures.; evidence: tc-cli-0092-1793500895, tc-cli-0093-1793500897, bug-0019-fixed-1
