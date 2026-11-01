---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: docs
---

# Docs

- docs/cli.md: the freshness table says `stale-revision-or-inputs` means a fingerprinted working file changed (the revision is recorded, not compared), plus the bookkeeping exclusions, the legacy record rule and the ordered bug verification.
- docs/architecture.md: the "Evidence is scoped, not permanent achievement" section (around line 123) that says a later commit, even a metadata-only one, stales observations and calls reuse an "unapproved, deferred proposal".
- docs/walkthrough.md: the "Read freshness precisely" section.
- docs/FRD.md: the REQ-cli-004 section ("Every new HEAD, including a metadata-only commit, remains stale under the current implementation") and the "Excluded" list, which names "reuse of observations after metadata-only commits".
- docs/DECISIONS.md: a new decision, worded as the human approves it, that supersedes D-016 (observation reuse after metadata-only commits deferred); D-016 is marked superseded, and the "Agreed direction" note that calls the proposal deferred is updated.
- docs/NEXT_CHANGE.md: a status note that this change addresses its problem for content-identical commits, without ancestry or reuse labels.
- docs/lifecycle.md: the accept order and the interim SpecSync dependency.
- docs/exercises/cold-start.md: a dated note under "Limits and follow-ups" that the stale reading it calls "Chough's conservative freshness rule working as designed" came from the earlier revision-bound rule. The exercise record itself is not rewritten.
- README.md: the `evidence add` row ("bound to revision, inputs, ...") says the revision is recorded and freshness compares content.
- specs/cli/testing.md: replace the "stale ... by design" note.
- specs/cli/context.md: the lesson from CorvidLabs/raven#644, with its source, scope, future use and a retirement condition (retire it when SpecSync ships a verification exclusion).
- Skill references: accept.md, lifecycle.md (including the "Evidence records" paragraph that says observations bind to the Git revision and working-tree fingerprint) and bugs.md, in all three bundle copies.
