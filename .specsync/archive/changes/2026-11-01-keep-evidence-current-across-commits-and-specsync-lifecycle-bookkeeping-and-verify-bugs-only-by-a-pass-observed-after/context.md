---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: context
---

# Context

The Raven run (CorvidLabs/raven#644) reported two problems, filed as CorvidLabs/chough#10 and #12. During triage both were reproduced on main 2756dc2 in scratch clones, and they were then recorded as bug records BUG-0015 and BUG-0016. Those records link no observations yet, so their verification label is `unverified`; the linked failing reproductions are plan step 1. BUG-0014 was already open from the release-candidate retention verifier and has not been reproduced in this workflow.

- BUG-0015 (#10): `snapshot()` in `src/evidence.rs` hashes `git rev-parse HEAD` into the fingerprint before the file list, and `freshness()` compares the whole fingerprint. Any commit stales every observation. That includes the commit that saves the observation, and even `git commit --allow-empty`. `git reset --soft HEAD~1` makes it current again. specs/cli/testing.md describes the resulting stale reading as "by design", docs/FRD.md says a new HEAD "remains stale under the current implementation", and docs/exercises/cold-start.md calls it the freshness rule "working as designed". docs/DECISIONS.md D-016 keeps reuse after metadata-only commits unapproved and deferred, so approving this change supersedes it.
- BUG-0016 (#12, Chough side): the fingerprint also covers SpecSync's per-change bookkeeping. So `specsync change check --commit` stales the observations it is meant to accompany, and so does `finalize`, which moves the change into the archive. On the SpecSync side, the verification project-input digest hashes every non-ignored file, including `.chough/`. SDD `ignored_paths` does not feed it, so any Chough write after a check forces another check. That part is upstream and is not changed here.
- BUG-0014: `bugs::verification` reports `reproduced-and-later-passed` when any linked observation passed, whatever its order. `bug update --status verified` only needs a pass. This fix makes durable evidence stay current for longer, so ordering matters more.

Verifier corrections adopted:

- Do not exclude all of `.specsync/changes/` and `.specsync/archive/`. Before materialization, a change's deltas and artifacts are the only record of its scope. A prototype that excluded the whole directory let an observation stay current after a delta rewrite.
- A committed deletion that was already in the working tree changed the digest. The prototype hashed a `missing` marker for the deleted path, and after the commit the path was no longer listed at all.
- Older binaries reject records with the new field, because `Snapshot` uses `deny_unknown_fields`. That rejection is fail-closed.

Draft review corrections adopted: D-016 is superseded explicitly; `lesson-bundle.md` is bookkeeping, so finalize does not stale observations; REQ-cli-012 and REQ-cli-017 are modified in the delta rather than narrowed silently; `verified` needs a linked failure; the exact REQ-cli-004 heading edit is shown in requirements.md (it cannot be made before approval, because the audit refuses canonical edits under a draft); overlap with the bundle-usability and timing-report drafts is recorded in design.md.

Ruled out:

- Recomputing H(record revision || current content) per distinct revision. Its cost is unbounded in the number of revisions.
- Reusing SDD `ignored_paths`. It has a different meaning.
- Git-ignoring `.chough/`. Chough's records are meant to be committed.
- Renaming the `stale-revision-or-inputs` label. It breaks JSON consumers.

Out of this change: the upstream SpecSync exclusion (an issue to file, proposing `verification_ignored_paths`), #11, #13 and #14.
