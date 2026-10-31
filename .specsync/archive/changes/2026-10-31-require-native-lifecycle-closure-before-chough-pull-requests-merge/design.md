---
change: require-native-lifecycle-closure-before-chough-pull-requests-merge
artifact: design
---

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
