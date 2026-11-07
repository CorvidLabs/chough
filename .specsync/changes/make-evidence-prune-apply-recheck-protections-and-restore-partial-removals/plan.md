---
change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
artifact: plan
---

# Plan

## Approach

Keep the fix in `src/evidence.rs`. Refuse an apply when `.chough/removing/` already holds a file, prepare each candidate's exact bytes and permissions before its ledger write, recheck protections immediately before each removal, and replace direct unlinks with the existing `remove_verified`. Track successful removals. On any later failure, restore them with create-new writes, preserve recreated files and retain unrestorable original bytes in the hold directory, then append the existing `restored` ledger event and fail with an actionable error.

Add deterministic regressions before the fix. A test-only fault seam in the crate controls the interleaving for a newly appearing protection and a recreated-file rollback conflict; no runtime flag or environment variable is added. An integration regression uses a read-only evidence directory to make the observation removal fail after artifact removal through the real CLI.

## Out of scope

- Retention eligibility, protection definitions, authorization, timing, bounds and successful output shape.
- `library archive --apply`, history pruning, native archives, Git history and SpecSync lifecycle behavior.
- A global writer lock, coordinator, runtime, persistent second ledger or new public command.
- Bulk migration or edits to existing observations, artifacts, ledger entries or frozen archives.
- Issue #30 and every unrelated open bug.

## Steps

1. Add the four focused regressions and their per-change case definitions. Run them against the unfixed tree and preserve the actual failure-first commands, assertions and logs; create or link local bugs before implementation.
2. Implement prepared removals, stale-hold refusal, byte-checked deletion, rollback and restored events in `src/evidence.rs` without changing the archive path.
3. Update REQ-history-004 and the CLI Purpose, their testing companions and contract histories, the FRD summary, retention documentation, cases and the Unreleased changelog.
4. Run the focused tests and cases, full Fledge verify on macOS, and the repository's Linux verify in Docker. Record only observed outcomes.
5. Merge current origin/main before PR and merge, resolve only this change's conflicts, rebuild the knowledge index if required, and request implementation review at the final archive tip.

One agent owns all affected files; no subagent is needed.

Estimate: small. The main uncertainty is proving exact race interleavings and rollback conflicts deterministically without timing-sensitive tests. `chough timing --cycle current` reported 52 historical bug fixes with median elapsed wall time 11m11s; that history is not effort, a target or a deadline, and this new session has no relevant gate samples.

## Risks

- A rollback could overwrite a concurrently recreated file or lose uncommitted original bytes. Create-new restoration and a create-only recovery hold prevent both; the conflict regression proves the behavior.
- A restored event could make a partial rollback look complete. The refusal names every unrestored path and recovery hold; tests inspect files and ledger labels rather than only the event.
- Test-only fault injection could alter production behavior. The seam is `#[cfg(test)]` only, and integration coverage exercises the real CLI without it.
- Reusing hold logic could change archive behavior. The archive implementation is not edited; its existing tests remain a regression check.

## Constraints consulted

REQ-history-001 and REQ-history-004 preserve retention meaning and ledger shape. `specs/history/context.md` locates evidence retention in `src/evidence.rs` and warns against adding native reads to closeout. PR #55 supplies the hold and restore pattern. DRIVE-19 requires deterministic rather than lucky-timing tests. No native read is added.
