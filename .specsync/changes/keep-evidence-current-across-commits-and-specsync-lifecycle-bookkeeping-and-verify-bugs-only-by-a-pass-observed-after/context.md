---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: context
---

# Context

The Raven run (CorvidLabs/raven#644) reported two problems, filed as CorvidLabs/chough#10 and #12. Both were reproduced on main 2756dc2 in fresh clones and recorded as BUG-0015 and BUG-0016. BUG-0014 was already open from the release-candidate retention verifier.

- BUG-0015 (#10): `snapshot()` in `src/evidence.rs` hashes `git rev-parse HEAD` into the fingerprint before the file list, and `freshness()` compares the whole fingerprint. Any commit stales every observation. That includes the commit that saves the observation, and even `git commit --allow-empty`. `git reset --soft HEAD~1` makes it current again. specs/cli/testing.md and docs/FRD.md currently describe this as "by design".
- BUG-0016 (#12, Chough side): the fingerprint also covers SpecSync's per-change bookkeeping. So `specsync change check --commit` stales the observations it is meant to accompany, and so does `finalize`, which moves the change into the archive. On the SpecSync side, the verification project-input digest hashes every non-ignored file, including `.chough/`. SDD `ignored_paths` does not feed it, so any Chough write after a check forces another check. That part is upstream and is not changed here.
- BUG-0014: `bugs::verification` reports `reproduced-and-later-passed` when any linked observation passed, whatever its order. `bug update --status verified` only needs a pass. This fix makes durable evidence stay current for longer, so ordering matters more.

Verifier corrections adopted:

- Do not exclude all of `.specsync/changes/` and `.specsync/archive/`. Before materialization, a change's deltas and artifacts are the only record of its scope. A prototype that excluded the whole directory let an observation stay current after a delta rewrite.
- A committed deletion that was already in the working tree changed the digest. The prototype hashed a `missing` marker for the deleted path, and after the commit the path was no longer listed at all.
- Older binaries reject records with the new field, because `Snapshot` uses `deny_unknown_fields`. That rejection is fail-closed.

Ruled out:

- Recomputing H(record revision || current content) per distinct revision. Its cost is unbounded in the number of revisions.
- Reusing SDD `ignored_paths`. It has a different meaning.
- Git-ignoring `.chough/`. Chough's records are meant to be committed.
- Renaming the `stale-revision-or-inputs` label. It breaks JSON consumers.

Out of this change: the upstream SpecSync exclusion (an issue to file, proposing `verification_ignored_paths`), #11, #13 and #14.
