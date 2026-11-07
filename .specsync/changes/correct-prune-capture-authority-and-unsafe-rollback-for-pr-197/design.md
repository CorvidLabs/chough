---
change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
artifact: design
---

# Technical design

## Capture authority

At artifact preparation, read the bytes once and compute their digest. Compare it to the already selected observation's `artifact_fingerprints[artifact]` before adding the bytes to the removal list. A mismatch is refused before this set's ledger write or removal; the existing outer rollback restores any earlier removals. Later changes remain guarded by byte-checked removal. Merely comparing a file against a captured replacement is not authority.

## Per-file rollback containment

Move destination `local_path` validation into the per-file restoration result, alongside directory preparation, private-copy finishing and create-only publication. A validation error follows the same recovery branch as another restoration failure: preserve original bytes through the independent safe hold path, record the problem, and continue every remaining held file. A best-effort hold-directory cleanup must not introduce another early return before restored accounting. Keep recreated symlink identity and target bytes untouched.

Preserve the current `place_copy`, `remove_verified`, recovery-hold naming, restored-event schema and truthful restored-file counts. Do not refactor the archive path or add a lock, runtime option, environment fault flag or public API. Test-only capture injection is `cfg(test)` and local to the unit-test module's existing pattern.
