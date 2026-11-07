---
change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
artifact: research
---

# Research

## Current failure mechanism

At origin/main `455cb643`, `retention()` in `src/evidence.rs` performs these separate operations for every artifact: protection fingerprint comparison, SHA-256 comparison, then `remove_if_present`. Another writer can add a reference after either comparison. The observation repeats the pattern after all artifacts. Any later `ensure!` or unlink error returns immediately, with no restoration and no `restored` event.

Existing classification already revalidates every set before its ledger line and checks candidate fingerprints. Those checks remain necessary but do not close the per-file window.

## Proven local prior art

PR #55's `library archive --apply` uses:

- `stale_holds` to refuse a new apply while earlier recovery files need inspection;
- `remove_verified` to rename a file into `.chough/removing/`, compare exact bytes and delete or put it back without replacement;
- saved bytes and permissions for create-new rollback;
- `restored` ledger events, which existing readers already understand;
- per-file protection revalidation and actionable failure text.

Its archive tests cover native-validity changes, partial removal, protection appearing during removal and recreated destinations. This change reuses those mechanisms without editing the archive path.

## Evidence-prune difference

Archive-summary candidates are verified byte-identical at HEAD, so its rollback can call removed bytes recoverable from Git. Evidence prune has no such condition. Routine new observations and artifacts may be uncommitted. A create-only recovery hold is therefore required when a recreated destination prevents restoration; assuming HEAD would lose the only local copy.

## Alternatives considered

- Keeping direct unlink plus better diagnostics: rejected because it restores nothing and leaves the race intact.
- A global lock around every Chough writer: rejected as a broad coordinator change that cannot serialize arbitrary external filesystem writers.
- Refactoring both retention commands into a new transaction subsystem: rejected as unnecessary blast radius; the existing hold and ledger primitives suffice.
- Restoring by rename or replacement: rejected because it can overwrite a file another writer recreated.
- Claiming every removed file is recoverable from Git: rejected because evidence-prune candidates may be uncommitted.

No dependency, external service or additional native status read is needed.
