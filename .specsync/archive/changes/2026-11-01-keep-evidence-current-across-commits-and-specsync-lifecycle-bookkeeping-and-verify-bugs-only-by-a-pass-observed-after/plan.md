---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: plan
---

# Plan

## Approach

Make freshness compare a revision-free content digest that also leaves out SpecSync bookkeeping. Keep the legacy fingerprint and the revision. Order the bug verification checks. Document the accept order. One bug-fix change, because #10 and #12 share one root cause and BUG-0014 becomes more likely once evidence stays current.

This implements a narrow form of the observation reuse that docs/DECISIONS.md D-016 keeps unapproved and deferred (see open question 5). Approving this scope is that decision; it is recorded as a new decision that supersedes D-016.

## Out of scope

- The upstream SpecSync verification exclusion: a separate CorvidLabs/spec-sync issue proposing `verification_ignored_paths`.
- Renaming freshness labels, and a separate "recorded at an older revision" label.
- Ancestry checks for freshness. Content equality decides freshness at any revision (see open question 5).
- Rewriting existing evidence records or frozen archives.
- Issues #11, #13 and #14, and any release.

## Steps

Each step gives its relative size and main uncertainty.

1. Record a failing reproduction for each of BUG-0014, BUG-0015 and BUG-0016 as new case definitions with failing observations at the unfixed tip, and link them to the bugs. Today all three bugs have no linked observations and read `unverified`; the #10 and #12 reproductions exist only in triage scratch clones, and BUG-0014 has not been reproduced in this workflow. Small. Uncertainty: fixtures for SpecSync bookkeeping without a real approval, using stubbed files in a temporary repository.
2. Content digest, key helper, legacy fallback and record validation in src/evidence.rs, with tests. Medium. Uncertainty: `change.md` front-matter normalization and moving into the archive.
3. Ordered bug verification and the `verified` rule in src/bugs.rs, with tests. Small.
4. Accept and lifecycle references, the bugs.md line, the three copies, and the `KNOWN_PREVIOUS` digests. Small.
5. Docs and contract wording, the specs/cli/testing.md note, the D-016 superseding decision and a context.md lesson. Small.
6. Normalize the canonical REQ-cli-004 heading exactly as requirements.md shows, before the scoped `change check`. Small.
7. Rerun the cases (passing), then follow the documented order end to end in a disposable exercise copy (check, commit, check, review with a SIMULATED label, finalize) and show that observations stay current throughout. Medium. Unverified until run: the triage end-to-end run (12-F) used the rejected exclusion of all of `.specsync/changes/`, so it does not support the narrower bookkeeping list. Uncertainty: SpecSync writing files outside the known bookkeeping set.

The REQ-cli-004 heading edit is made after approval because `specsync change audit` refuses canonical spec edits under a draft; its exact text is in requirements.md, so it is reviewed with the scope.

One agent builds. No file ownership split is needed.

## Risks

- Over-exclusion would let real scope edits read as current. Mitigation: an explicit allow-list of top-level file names only, plus tests that a delta, requirements or other artifact edit stales an observation.
- SpecSync may add bookkeeping files in later versions. Those read as stale, which fails closed, and the list is revised with the pinned version.
- Older Chough binaries reject new records. Fail-closed, and noted in the change log.
- Existing records stay stale until they are rerun once.
- Two other drafts edit the same files and one also modifies REQ-cli-012 (see "Coordination with other drafts" in design.md).

## Constraints consulted

See the `constraints` answer, Invariants 2, 4, 6 and 8, REQ-cli-012, REQ-cli-017, D-016, and the triage and verifier records for #10 and #12.

## Open questions for the human

1. Archive move: should the move use the location-independent key (recommended, and in the criteria), or should one stale reading after finalize be accepted and documented?
2. `lesson-bundle.md`, written by finalize: the criteria treat it as bookkeeping, because it is a summary generated from records already hashed, and leaving it hashed would make every finalize stale the observations. The alternative is to keep it hashed and document one stale reading at finalize, which also changes criteria 2 and 3. This must be settled before approval, because the criteria and delta freeze then.
3. Verified rule with no linked failure: the criteria now refuse `verified` unless a failure is linked (recommended, since REQ-cli-012 keeps a bug unverified until a failed observation is linked, and today `verified` can stand on a pass with no proof of the defect). The alternative keeps today's "any present pass" for that case, and drops the last sentence of criterion 6 and of the REQ-cli-012 acceptance line.
4. Schema: keep evidence schema 2 with an optional field and modify REQ-cli-017 to list it (recommended, and in the delta), or bump to 3, so that old binaries say "unsupported schema" instead of "invalid record"? Either way, older binaries fail closed.
5. D-016: docs/DECISIONS.md says observation reuse after metadata-only commits "stays unapproved and deferred ... needs its own decision", and docs/FRD.md and docs/architecture.md repeat it. This change implements a narrower form than docs/NEXT_CHANGE.md proposed: it keeps the recorded revision and does not claim a run at the new one, but it has no ancestry check and no separate reuse label. An observation reads current at any revision with the same content identity, including unrelated or rewritten history with identical files. Do you approve that as the decision that supersedes D-016 (recommended, since the observation tested exactly that content), or do you want ancestry or a separate label first?
6. BUG-0014 was recorded `out-of-scope`. This draft moves it in scope, because the change ID, criterion 6 and the REQ-cli-012 delta item fix it. If you would rather keep it out, criterion 6 and the REQ-cli-012 item come out of this change.
7. docs/NEXT_CHANGE.md: mark it as addressed for content-identical commits by this change, keeping the ancestry and label ideas as unapproved history (recommended), or retire it?
8. Upstream: file the CorvidLabs/spec-sync issue proposing `verification_ignored_paths` now or after this lands? No duplicate was found. It would cross-link #657, #789, #433 and #676 (ignored_paths semantics), and note that `exclude_dirs` in `.specsync/config.toml` does not exempt `.chough/` from the verification digest either.
9. Order with the other drafts: `bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors` also modifies REQ-cli-012 and documents when to run `change check`. Which is approved first? The second must carry the first's REQ-cli-012 sentence and the same check-order wording before its own approval.
