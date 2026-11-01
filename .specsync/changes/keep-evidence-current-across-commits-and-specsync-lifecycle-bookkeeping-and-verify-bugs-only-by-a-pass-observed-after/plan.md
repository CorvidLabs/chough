---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: plan
---

# Plan

## Approach

Make freshness compare a revision-free content digest that also leaves out SpecSync bookkeeping. Keep the legacy fingerprint and the revision. Order the bug verification checks. Document the accept order. One bug-fix change, because #10 and #12 share one root cause and BUG-0014 becomes more likely once evidence stays current.

## Out of scope

- The upstream SpecSync verification exclusion: a separate CorvidLabs/spec-sync issue proposing `verification_ignored_paths`.
- Renaming freshness labels, and a separate "recorded at an older revision" label.
- Ancestry checks for freshness, and reuse across unrelated or rewritten history.
- Rewriting existing evidence records or frozen archives.
- Issues #11, #13 and #14, and any release.

## Steps

Each step gives its relative size and main uncertainty.

1. Record a failing reproduction for each of BUG-0014, BUG-0015 and BUG-0016 as new case definitions with failing observations at the unfixed tip. Small. Uncertainty: fixtures for SpecSync bookkeeping without a real approval, using stubbed files in a temporary repository.
2. Content digest, key helper, legacy fallback and record validation in src/evidence.rs, with tests. Medium. Uncertainty: `change.md` front-matter normalization and moving into the archive.
3. Ordered bug verification and the `verified` rule in src/bugs.rs, with tests. Small.
4. Normalize the REQ-cli-004 heading. Small.
5. Accept and lifecycle references, the three copies, and the `KNOWN_PREVIOUS` digests. Small.
6. Docs and contract wording, the specs/cli/testing.md note and a context.md lesson. Small.
7. Rerun the cases (passing), then follow the documented order end to end in a disposable exercise copy (check, commit, check, review with a SIMULATED label, finalize) and show that observations stay current throughout. Medium. Uncertainty: SpecSync writing files outside the known bookkeeping set.

One agent builds. No file ownership split is needed.

## Risks

- Over-exclusion would let real scope edits read as current. Mitigation: an explicit allow-list of file names only, plus tests that a delta, requirements or other artifact edit stales an observation.
- SpecSync may add bookkeeping files in later versions. Those read as stale, which fails closed, and the list is revised with the pinned version.
- Older Chough binaries reject new records. Fail-closed, and noted in the change log.
- Existing records stay stale until they are rerun once.

## Constraints consulted

See the `constraints` answer, Invariants 2, 4, 6 and 8, REQ-cli-017, and the triage and verifier records for #10 and #12.

## Open questions for the human

1. Archive move: should the move use the location-independent key (recommended), or should one stale reading after finalize be accepted and documented?
2. `lesson-bundle.md`, written by finalize: should it count as bookkeeping (recommended, since it is a generated summary), or stay hashed, so that observations go stale once at finalize?
3. Verified rule with no linked failure: keep today's "any present pass" (recommended), or require a failure first?
4. Schema: keep evidence schema 2 with an optional field (recommended), or bump to 3, so that old binaries say "unsupported schema" instead of "invalid record"?
5. Should REQ-cli-012 gain an explicit ordering sentence in this delta? Not included, because only REQ-cli-004 was agreed.
6. BUG-0014 is recorded `out-of-scope`. Should it be moved in scope for this change once the scope is approved?
