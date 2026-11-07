---
change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
artifact: design
---

# Design

## Prepared removal

At apply start, call `stale_holds` and refuse if any prior recovery file needs inspection, before writing a new ledger line. For each eligible set, the existing fresh classification and artifact fingerprint checks still run. Read the observation and removable artifact bytes and permissions before appending the set's existing ledger entry. The ledger entry is unchanged and remains first.

Artifacts are removed first and the observation last so the set stays classifiable. Immediately before each file, recompute `history::protection_fingerprint` and require equality with the set's captured value. Call `remove_verified(root, path, expected_bytes)` instead of digest-checking plus `remove_if_present`. Its rename into `.chough/removing/` and exact byte comparison close the check/unlink window without replacing a writer's file. Record each successful removal's path, bytes and permissions.

## Rollback and recovery

Any protection mismatch, changed file or removal error after a ledger line triggers rollback for all earlier removals of that set and any earlier ledgered sets. Restore each with `create_new(true)`, write the exact saved bytes and restore permissions. Never overwrite a path recreated meanwhile. Continue best-effort restoration so one conflict does not strand other files.

If a recreated path or another restoration failure prevents recovery, write the original bytes create-only to the existing digest-derived `.chough/removing/<digest>.hold` path and name it in the refusal. Unlike archive-summary proof, an evidence-prune candidate need not be committed, so the error never substitutes "recover from HEAD" for an actual retained copy. Existing holds are never replaced and cause the next apply to refuse with an inspection remedy.

Append one `restored` event per ledgered set using the current schema: `schema_version`, `event`, `id`, `reason` and `restored_at_unix_seconds`. The reason names the original failure and every unrestored path or recovery hold. Existing ledger readers therefore undo the removal label, while the refusal makes partial recovery explicit. Return an error; never include the failed set in `pruned`.

Successful applies keep their existing `eligible`, `protected`, `totals`, `pruned`, `skipped`, `ledger`, `note` and policy output. Preview remains read-only.

## Deterministic fault injection

A narrow `#[cfg(test)]` hook in the evidence module runs at the boundary after a successful removal and before the next protection check. Tests use it to add a checkpoint reference, fail a later set after an earlier set completed, and recreate a path during rollback. It is not compiled into release binaries and has no environment variable or CLI setting. The partial-removal integration test instead makes the evidence directory read-only, exercising the real CLI without a seam.

## Ownership and compatibility

Only `src/evidence.rs` changes production behavior. `library archive --apply` and `src/history.rs` are untouched. Existing remove/hold and ledger reader functions are reused, with no new persisted schema, native read, lock, service or coordinator.
