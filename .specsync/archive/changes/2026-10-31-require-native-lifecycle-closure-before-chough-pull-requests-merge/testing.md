---
change: require-native-lifecycle-closure-before-chough-pull-requests-merge
artifact: testing
---

# Testing

This is the concrete regression plan for the explicitly authorized CI work. Results below are recorded only after observation. This native scope remains draft; no authentic human approval, review, or archive is claimed.

## Real native integration fixtures

| Case | Expected | Observable assertion |
| --- | --- | --- |
| Clean native repository, no active or historical changes | pass | Native audit succeeds, roster is empty, complete inventory is empty |
| Valid archived native fixture and complete Git history | pass | Native status returns archived plus authenticated-history |
| Legitimate later revision changes an originally affected input plus unrelated content | pass | Native authenticates historical archive inputs without re-running old behavior against current HEAD |
| Main advances with later-only archives after the PR branch forks | pass | Inventory uses the trusted merge base; inherited omissions remain distinct from later-only base additions |
| Draft scope only, implementation never begun | fail | Native roster contains it and output names the unfinished scope |
| Approved / implementing / verifying / accepted active fixture | fail | Every nonempty active roster blocks closure, including accepted-before-finalize |
| One archived sibling plus one draft sibling | fail | Every scope is accounted for; one archive cannot hide another change |
| Missing state with tracked artifact; malformed JSON; invalid ID; ambiguous active/archive ID | fail | Native roster/status error or degraded result remains a failure |
| Archived label with missing verification, review, approval ledger, or invalid ancestry | fail | Native historical authentication rejects it |
| Alter archived evidence/delta after archival | fail | Native returns corrupt-history; no record is silently skipped |
| Changed live verification inputs while a change is active | fail | Audit reports staleness and active roster still blocks Archive |
| Policy deleted, malformed, wrong schema, or undecodable | fail | Required repository input/native parse failure stays visible |
| Explicitly disabled bootstrap policy with a draft | fail | Active inventory runs despite native audit enabled=false |
| Required base/head absent, shallow history, bad ancestry | fail | No fallback to an empty comparison or no-change result |
| Existing or newly introduced draft removed/renamed before the head | fail | Per-commit native inventory ID is unresolved as authenticated archive at head |
| Archive package removed or partial package left | fail | Historical inventory obligations and package completeness remain visible |
| Symlink or unreadable inventory entry | fail | Inventory refuses an incomplete view |
| Missing native binary; wrong version; nonzero exit | fail | No placeholder pass or skipped-tool result |
| Native stdout malformed, concatenated, truncated, wrong type, or unexpected schema | fail | Adapter rejects protocol output rather than substituting empty inventory |
| Every pass/failure fixture | unchanged | Before/after Git status and lifecycle byte hashes are identical |

All lifecycle fixtures live under a temporary directory, use synthetic fixture-only identities and local Git history, and never write real Chough approvals. A passing historical fixture must use real native semantics; mocked positive integrity output is forbidden. Protocol-negative fixtures may use deliberately broken tools to prove fail-closed behavior.

## Workflow validation

Assert both PR and main-push event coverage, `contents: read`, absence of `pull_request_target` and write permissions, full checkout history, actual head/base handling, bounded job timeouts, exact six check names, and no draft/path/stage filter. Verify pinned action SHAs and SpecSync version agreement. Ensure all current product/fmt/Clippy/HI/strict-coverage commands remain represented.

Run the new `fledge run ci-archive-fixtures`, the current Fledge verify lane, and hosted matrix/native checks after implementation. Compare dirty lifecycle hashes around inspection. Archive is expected red for this change until genuine human review and native finalization create its archive tip; green product jobs alone do not authorize merge.

## Runtime integration follow-up

PR #2's interfaces are on main. Stage-gate, bounded-process and stop-timing regressions are being added by the bug-fix changes for BUG-0006 to BUG-0011. Still open as follow-up: real stage/profile/traceability, full installed skill bundle, cold-resume, and complete-current-evidence fixtures against that source. Preserve existing current CLI tests for initialization, bounded context, traceability, stale evidence, and unavailable/invalid native JSON. Do not manufacture green checks for planned behavior or infer semantic human acceptance from fixture text.

## Observed local results — 2026-10-31

- Fledge 1.7.2 verify lane passed all six steps: formatting, Clippy with warnings denied, all 14 current CLI integration tests, HI 0.8.0 (7 criteria; no problems), SpecSync 6.1.1 strict 100% structural coverage, and build. Product source/tests were unchanged.
- The real-native Archive fixture suite passed all 12 unittest methods, including subcases for authenticated archived success, all active states, disabled policy, malformed/missing packages and configuration, invalid tool protocol, actual stale verification, deleted scopes/archives, and history/unsafe-input failures. Each invocation asserts unchanged lifecycle bytes and Git status.
- `actionlint .github/workflows/ci.yml` and `git diff --check` passed.
- Archive inspection of this branch correctly returned exit 1 and named this active draft. It did not create any approval, review, verification, or archive.
- Independent read-only review found no blocking source/workflow issue; inspection against actual main57aa passed with zero archives and against remote runtime draft5886607 rejected its active scope, leaving both review trees clean.
- Hosted Ubuntu/macOS/Windows and new Actions jobs remain pending until this exact implementation tip is pushed. Local results do not claim hosted checks or human acceptance.

Local tool selection used an invocation-only PATH shim for Fledge1.7.2/HI0.8.0/SpecSync6.1.1. The initial ordinary local PATH resolved an older SpecSync6.1.0; the explicit version guard refused it before running fixtures. Global tooling was not changed.

- Focused positive fixture passed after the full suite: valid archive remains authenticated after a later committed change to its originally affected README plus unrelated content; advanced-main/older-branch history also passes. The gate delegates historical integrity to native status and performs no behavioral re-verification of archived work.

## Observed after the rebase — 2026-10-31

Historical results above were taken at base `57aa169`. At rebased tip `7929160` (base
main `2d782c4`): hosted Product on Ubuntu, macOS and Windows, `CI gate fixtures`
(12 tests OK) and CodeQL passed; `Fledge / HI / SpecSync` and `SpecSync Archive` failed
at `specsync change audit` because this draft does not yet cover its meaningful paths
under main's enabled policy (expected until approval). Locally: 39 tests, fmt, Clippy,
HI, strict SpecSync, cases, exercise and build passed; the 12 fixtures passed; archive
inspection of a fresh clone of main `f19823d` passed with 2 authenticated archives.
