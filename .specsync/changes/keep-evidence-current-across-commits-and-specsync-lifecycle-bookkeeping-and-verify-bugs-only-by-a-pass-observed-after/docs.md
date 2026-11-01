---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: docs
---

# Docs

- docs/cli.md: the freshness table says `stale-revision-or-inputs` means a fingerprinted working file changed (the revision is recorded, not compared), plus the bookkeeping exclusions, the legacy record rule and the ordered bug verification.
- docs/architecture.md, docs/walkthrough.md, docs/FRD.md (the REQ-cli-004 section): replace the statement that every commit stales observations.
- docs/NEXT_CHANGE.md: a status note that this change addresses its problem for content-identical commits, without ancestry or reuse labels.
- docs/lifecycle.md: the accept order and the interim SpecSync dependency.
- specs/cli/testing.md: replace the "stale ... by design" note.
- specs/cli/context.md: the lesson from CorvidLabs/raven#644, with its source, scope, future use and a retirement condition (retire it when SpecSync ships a verification exclusion).
- Skill references: accept.md, lifecycle.md and bugs.md, in all three bundle copies.
