# Lesson bundle — require-native-lifecycle-closure-before-chough-pull-requests-merge

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Require native lifecycle closure before Chough pull requests merge
- **Kind**: Operations
- **Paths**: .github/, docs/ci.md, fledge.toml, .specsync/config.toml
- **Acceptance**: A required-check candidate named SpecSync Archive runs for every pull request, including drafts, without path or stage filters; it rejects every active native change and every unreadable or incomplete lifecycle package. Native SpecSync semantics authenticate archive closure and history; malformed, missing, ambiguous, stale, unknown, or unavailable inputs fail closed. CI performs no lifecycle mutation or human approval and explains that draft work can be expected red. Existing three-platform product tests/builds, HI, strict 100 percent SpecSync, formatting and Clippy remain present with consistent pinned tool versions. Isolated fixtures prove allowed no-active and authentic archived cases and rejected draft, in-progress, stale, incomplete, missing, malformed, omitted-record and unavailable-tool cases. The PR documents exact check contexts and read-only protection findings without changing branch rules or claiming human acceptance.

## Evidence

- Verification commit: `0e395a02c8b297af8d8ed897e4c56ad0f2371914`
- Base commit: `57aa169588522ef44438236b9f03f409dc1254c2`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

The human requested complete, useful Chough GitHub Actions checks, specifically a SpecSync Archive check that prevents merging any in-progress native change, including a scope-review draft. Product tests alone cannot establish lifecycle closure. The human explicitly authorized implementing this separate CI change and reviewable draft PR while the runtime review stays read-only. When drafted, the committed policy was optional; PR #2 has since enabled it on main (`enabled` and `require_change_for_meaningful_files` true). Implementation proceeds within the original authorization without changing that policy or inventing an approval record. This execution request does not authenticate human approval of this native definition digest.

Drafted from main `57aa169588522ef44438236b9f03f409dc1254c2` (the recorded base_commit), independently of the then-draft runtime PR #2. On 2026-10-31 PR #2 and PR #4 merged, and this branch was rebased onto main `2d782c4`; main has since advanced to `f19823d` with a checkpoint-only commit. No runtime source, approval, review, or archive is changed here.

The current workflow runs Rust tests and release builds on Ubuntu, macOS, and Windows. Its native job runs Fledge, HI, strict 100% SpecSync structural coverage, formatting, and Clippy. It has no archive check. Main's SDD policy is now enabled, so `specsync change audit` (run by the Fledge job) refuses this change's meaningful paths until its definition is approved: both `Fledge / HI / SpecSync` and `SpecSync Archive` are expected red while it is draft.

The merged runtime work added a native audit task and full-history checkout. The rebase kept both (Fledge `audit`, `cases`, `exercise` tasks; `fetch-depth: 0`) alongside this change's pinned actions. Native audit remains different from archive closure; both stay.

Applicable intent: LIFE-6 keeps native tools authoritative; LIFE-7 distinguishes structural tests, product verification, human acceptance, and delivery. Applicable repository policy: later scoped changes use the full native lifecycle with genuine human decisions, and agents must not create human approval/review records from actor labels.

This package records the concrete technical scope. Implementation and isolated verification proceed under the explicit CI request and current optional policy. Genuine native approval/review/finalization and merge are not claimed. No approval record is present; Archive remains expected red while this scope is active.

## From the change's design.md

# Design

## Proposed files and check contexts

Update `.github/workflows/ci.yml`; add `.github/scripts/check-specsync-archive.py` and `.github/scripts/test-specsync-archive.py`; add the local Fledge task `ci-archive-fixtures`; document operation and enforcement in `docs/ci.md`; align the `.specsync/config.toml` writer/CI comment with 6.1.1. Native package files remain owned by SpecSync. The current CLI contract and `src/` and `tests/` product implementation do not change.

Keep existing job contexts `Product / ubuntu-latest`, `Product / macos-latest`, `Product / windows-latest`, and `Fledge / HI / SpecSync`. Add independent contexts `SpecSync Archive` and `CI gate fixtures`. These names are the concrete required-check recommendation after successful introduction on main, not a claim that the names are currently protected.

## Workflow

Keep `push` to main and `pull_request` to main, including draft PR events. Use read-only `contents` permission, no `pull_request_target`, no secrets or write token, no comments, and no lifecycle mutations. Do not filter by changed path, draft status, implementation stage, or a claim that product work has not begun. The Archive job must run even when product checks fail; do not make it dependent on a product pass.

Use full history and the actual PR head SHA. Preserve the base ref required by native SpecSync comparison semantics and fail if the PR base/head or ancestry cannot be established. Push-to-main runs validate the same closure property against the available previous main revision, and must not silently substitute an empty base. Use bounded job timeouts and actionable failure summaries.

Pin third-party actions to verified commit SHAs with version comments. Keep the reviewed Fledge action commit and binary `v1.7.2`, HI `0.8.0`, and use SpecSync action commit `e8fd149d4d9e7b8df9ea1d40d446df67da13f2fa` (published v6.1.1) with explicit `version: '6.1.1'`. The tagged action otherwise defaults to 6.1.0. Verify `specsync --version` before native consumers and fail on disagreement or missing binaries. Use the upstream action's release archive/checksum validation; no global tool changes outside the ephemeral runner.

## Closure algorithm

The helper is an adapter around published native read surfaces, not a second lifecycle state machine. It never runs `change check`, approve, review, ship, finalize, archive, or any command that writes lifecycle evidence.

1. Require the repository's committed native configuration and policy inputs. Require explicit Chough policy schema version 2 and a boolean enabled field; parse the TOML configuration, then let native parsing validate policy semantics. Reject symlinks throughout lifecycle inputs. An explicitly disabled bootstrap policy does not hide active changes: closure inventory runs independently of `report.enabled`. A missing or malformed policy is a failure for this repository.
2. Run `specsync change audit --json` and require exit zero, one complete object, the expected `report` shape, and zero native errors. Keep warnings visible. Audit covers current active definitions, deltas, policy coherence, and current evidence; it intentionally does not validate archived history.
3. Run `specsync change list --json` and require exit zero and the exact native roster shape. The zero-active result is an empty array in 6.1.1. A nonempty result always fails Archive, regardless of state or implementation work. Unreadable or degraded output, unknown shape, diagnostics before/after JSON, and unknown status must fail, never become an empty list.
4. Inventory all native archive packages without trusting state strings as proof. Require readable package metadata solely to identify an ID, refuse missing/invalid IDs, duplicates, symlinks, partial packages, and unexpected entries. Resolve each ID using `specsync change status <ID> --json`; require native `change`/`summary` ID agreement, archived state, and `summary.terminal_evidence.validity == 'authenticated-history'`. Missing, stale, `corrupt-history`, or unknown evidence fails. Native summary is authoritative for historical content, ancestry, review, and approval ledger consistency; an actor label never proves a human identity.
5. Detect omitted records. Resolve the trusted base SHA to its merge base with the checked head, then inventory that common ancestor plus each distinct PR commit snapshot using the native roster (in isolated temporary Git snapshots retaining full object history), and inventory archive IDs in those snapshots. Every discovered lifecycle ID must resolve at the checked head to an authenticated archive. Removing a draft, renaming a folder, deleting state, or introducing then removing a scope package must not turn an incomplete change into no-active success. Do not read only the final diff or only implementation files. Temporary snapshots may change temporary Git bookkeeping; the checked source tree and every lifecycle byte remain unchanged.
6. Verify the tracked worktree is unchanged after all native calls. Fail if inspection wrote source/lifecycle files. Diagnostics may go to temporary files or the Actions summary, not an observation or approval record.

Native 6.1.1 has no public nonmutating full-history audit switch and no public archive roster. Therefore the narrow inventory/shape adapter above is needed; all historical acceptance decisions stay with native status. If a verified newer published native command supplies these surfaces before implementation, prefer it and submit the changed semantics for definition review rather than silently altering this scope.

## Regression architecture

Use Python's standard library and real pinned native SpecSync, invoked in temporary Git repositories. A small protocol fault fixture may substitute a deliberately broken executable only to prove missing/nonzero/malformed native output fails. It must never supply a fake positive native integrity decision. Fixture actors are clearly synthetic and local, never Chough approval records.

The authenticated archived success fixture must complete a native test lifecycle with fixture-only records and history, then establish native `authenticated-history`; merely writing `state: archived` is deliberately a rejection fixture. Snapshot hashes before and after prove no fixture lifecycle mutation by the gate. All fixtures are isolated from the actual Chough package.

## Human and integration boundaries

CI checks portable record integrity and observed deterministic behavior. It cannot authenticate human identity or product/design acceptance from filenames, model text, actor names, or synthetic fixture approvals. A human reviews the implementation through native review, then finalizes before merge. Archive is expected red throughout draft/in-progress work.

Do not change repository rules in this PR. PR #2 is merged. After the check exists on main, a separately authorized administrator can require all six named contexts. Stage-gate, bounded-process and stop-during-interval regressions belong to the Chough bug-fix changes for BUG-0006 to BUG-0011, not to this CI change; do not make planned checks report green here.

## From the change's testing.md

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

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
