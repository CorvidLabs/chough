---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: research
---

# Research

- Reproduced on 2756dc2: a case observation is `current-observation`, and after `git add .chough && git commit` it is `stale-revision-or-inputs`. An empty commit has the same effect, and `git reset --soft HEAD~1` restores it. The cause is `hash.update(revision.as_bytes())` in `snapshot()`.
- Prototype (scratch only): an optional `content` digest computed in the same pass, compared when present. It stayed current after a `.chough/`-only commit, after an empty commit, and after committing an edit that was already present. It went stale after a README edit and after a new untracked file. Legacy records stayed stale. The cli, evidence, cases and bugs tests passed.
- SpecSync 6.1.1 bookkeeping, from the materialize, verification and finalize commits of archived changes: `state.json`, `approvals.json`, `verification.json`, `verification-attempts.json`, `review.json`, `review-attempts.json`, `finalization.json`, `accepted-state.json`, and the `state:` front-matter field of `change.md`. Materialization changes only `state: approved -> implementing` there. Finalize also writes `lesson-bundle.md`, a generated summary (see the open question in plan.md). It moves `.specsync/changes/<id>/` to `.specsync/archive/changes/<YYYY-MM-DD>-<id>/`.
- The SpecSync verification digest (`specsync.project-input-digest.v4`) covers every non-git-ignored file except SpecSync's own change records. It has no exclusion setting, and SDD `ignored_paths` does not apply to it. Related upstream issues: CorvidLabs/spec-sync#789, #657 and #433.
- `bugs::check_regression` (src/bugs.rs) uses `snapshot.revision` for ancestry. Retention and `library` display the revision too. None of them use freshness.
